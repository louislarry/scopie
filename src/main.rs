mod alg;
mod camera;
mod dms;
mod image_display;
mod mount;
mod platesolve;
mod text_input;

use camera::display::CameraDisplay;
use dms::Angle;
use glutin::{
    self,
    dpi::{LogicalPosition, PhysicalPosition},
    event::{
        ElementState, Event, MouseButton, MouseScrollDelta, TouchPhase, VirtualKeyCode as Key,
        WindowEvent,
    },
    event_loop::{ControlFlow, EventLoop, EventLoopProxy},
    window::WindowBuilder,
    ContextBuilder, GlProfile,
};
use khygl::{
    check_gl, gl_register_debug, render_text::TextRenderer, render_texture::TextureRenderer,
    texture::CpuTexture, Rect,
};
use mount::{display::MountDisplay, thread::MountAsync};
use std::{
    collections::HashSet,
    convert::TryInto,
    fmt::Write,
    fs::File,
    path::Path,
    sync::Arc,
    time::{Duration, Instant},
};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Debug)]
pub enum UserUpdate {
    MountUpdate(mount::thread::MountData),
    CameraUpdate(camera::thread::CameraData),
    CameraData(Arc<camera::interface::ROIImage>),
    SolveFinished(Angle, Angle),
    ProcessResult(alg::process::ProcessResult),
}
type SendUserUpdate = EventLoopProxy<UserUpdate>;

fn read_png(path: impl AsRef<Path>) -> Result<CpuTexture<u16>> {
    let mut decoder = png::Decoder::new(File::open(path)?);
    decoder.set_transformations(png::Transformations::IDENTITY);
    let (info, mut reader) = decoder.read_info()?;
    assert_eq!(info.bit_depth, png::BitDepth::Sixteen);
    assert_eq!(info.color_type, png::ColorType::Grayscale);
    let mut buf = vec![0; info.buffer_size()];
    reader.next_frame(&mut buf)?;
    let mut buf16 = vec![0; info.width as usize * info.height as usize];
    for i in 0..buf16.len() {
        buf16[i] = u16::from(buf[i * 2]) << 8 | u16::from(buf[i * 2 + 1]);
    }
    Ok(CpuTexture::new(
        buf16,
        (info.width as usize, info.height as usize),
    ))
}

fn write_png(path: impl AsRef<Path>, img: &CpuTexture<u16>) -> Result<()> {
    let mut encoder = png::Encoder::new(File::create(path)?, img.size.0 as u32, img.size.1 as u32);
    encoder.set_color(png::ColorType::Grayscale);
    encoder.set_depth(png::BitDepth::Sixteen);
    let mut writer = encoder.write_header()?;
    let mut output = vec![0; img.size.0 * img.size.1 * 2];
    let data = img.data();
    for i in 0..(img.size.0 * img.size.1) {
        output[i * 2] = (data[i] >> 8) as u8;
        output[i * 2 + 1] = (data[i]) as u8;
    }
    writer.write_image_data(&output)?;
    Ok(())
}

struct Display {
    camera_display: camera::display::CameraDisplay,
    mount_display: Option<mount::display::MountDisplay>,
    next_frequent_update: Instant,
    next_infrequent_update: Instant,
    window_size: (usize, usize),
    last_mouse: Option<PhysicalPosition<f64>>,
    pressed_mouse: HashSet<MouseButton>,
    status: String,
    old_status: String,
    text_input: text_input::TextInput,
    texture_renderer: TextureRenderer,
    text_renderer: TextRenderer,
    wasd_mount_mode: bool,
    wasd_camera_mode: bool,
}

impl Display {
    fn window_size_f32(&self) -> (f32, f32) {
        (self.window_size.0 as f32, self.window_size.1 as f32)
    }

    fn run_cmd_impl(&mut self, text: &str) -> Result<()> {
        let cmd = text.split_whitespace().collect::<Vec<_>>();
        let mut command_okay = cmd.is_empty();
        match &cmd as &[&str] {
            ["wasd"] if self.mount_display.is_some() => {
                self.wasd_mount_mode = true;
                command_okay |= true;
            }
            ["zoom"] => {
                self.wasd_camera_mode = true;
                command_okay |= true;
            }
            _ => (),
        }
        command_okay |= self.camera_display.cmd(&cmd)?;
        if let Some(ref mut mount_display) = self.mount_display {
            match mount_display.cmd(&cmd) {
                Ok(ok) => command_okay |= ok,
                Err(mount::thread::MountSendError {}) => self.mount_display = None,
            }
        }
        if command_okay {
            Ok(())
        } else {
            Err("Unknown command".into())
        }
    }

    fn try_run_cmd(&mut self, text: &str) {
        match self.run_cmd_impl(text) {
            Ok(()) => self.text_input.set_exec_result(String::new(), true),
            Err(err) => self.text_input.set_exec_result(format!("{}", err), false),
        }
    }

    fn run_cmd(&mut self) {
        if let Some(cmd) = self.text_input.try_get_exec_cmd() {
            self.try_run_cmd(&cmd);
        }
    }

    fn setup(
        window_size: (usize, usize),
        scale_factor: f64,
        send_user_update: SendUserUpdate,
    ) -> Result<Self> {
        let texture_renderer = TextureRenderer::new()?;
        let height = 20.0 * scale_factor as f32;
        let text_renderer = TextRenderer::new(height)?;
        let send_user_update_2 = send_user_update.clone();
        let camera_display = CameraDisplay::new(send_user_update_2);
        let mount_display = Some(MountDisplay::new(MountAsync::new(send_user_update)));
        let text_input = text_input::TextInput::new();
        Ok(Self {
            camera_display,
            mount_display,
            next_frequent_update: Instant::now(),
            next_infrequent_update: Instant::now(),
            window_size,
            last_mouse: None,
            pressed_mouse: HashSet::new(),
            status: String::new(),
            old_status: String::new(),
            text_input,
            texture_renderer,
            text_renderer,
            wasd_mount_mode: false,
            wasd_camera_mode: false,
        })
    }

    // (wait until, redraw now)
    fn update(&mut self) -> Result<(Instant, bool)> {
        let mut redraw = false;
        let now = Instant::now();
        let frequent_update_rate = Duration::from_millis(50);
        let is_next_frequent_update = now >= self.next_frequent_update;
        if is_next_frequent_update {
            self.next_frequent_update += frequent_update_rate;
            let too_fast = now >= self.next_frequent_update + frequent_update_rate * 3;
            if too_fast {
                self.next_frequent_update = now + frequent_update_rate;
                self.text_input
                    .set_exec_result("Warning: target FPS too fast".to_string(), true);
            }
        } else {
            return Ok((self.next_frequent_update, false));
        }

        let infrequent_update = now >= self.next_infrequent_update;
        if infrequent_update {
            self.next_infrequent_update += Duration::from_secs(1);
        }
        self.status.clear();
        redraw |= self.camera_display.update();
        self.camera_display
            .status(&mut self.status, infrequent_update)?;
        if let Some(ref mut mount_display) = self.mount_display {
            mount_display.status(&mut self.status)?;
        }
        if self.wasd_mount_mode {
            writeln!(&mut self.status, "WASD/RF mount control mode (esc to stop)")?;
        } else if self.wasd_camera_mode {
            writeln!(&mut self.status, "WASD/RF camera zoom mode (esc to stop)")?;
            writeln!(&mut self.status, "G: set ROI (crop)")?;
        } else {
            if self.mount_display.is_some() {
                writeln!(&mut self.status, "wasd: mount control mode")?;
            }
            writeln!(&mut self.status, "zoom: camera zoom mode")?;
        }
        if self.old_status != self.status {
            self.old_status = self.status.clone();
            redraw = true;
        }
        Ok((self.next_frequent_update, redraw))
    }

    fn render(&mut self) -> Result<()> {
        let window_size_f32 = self.window_size_f32();
        unsafe {
            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }
        let text_size = self.text_renderer.render(
            &self.texture_renderer,
            &self.status,
            [1.0, 1.0, 1.0, 1.0],
            (10, 10),
            self.window_size,
        )?;
        let input_pos_y = self.text_input.render(
            &self.texture_renderer,
            &mut self.text_renderer,
            self.window_size,
        )?;
        let width = (self.window_size.0 as isize - text_size.right() as isize)
            .try_into()
            .unwrap_or(1);
        let camera_rect = Rect::new(text_size.right(), 0, width, input_pos_y);
        self.camera_display
            .draw(camera_rect, &self.texture_renderer, window_size_f32)?;
        Ok(())
    }

    fn resize(&mut self, size: (usize, usize)) -> Result<()> {
        self.window_size = size;
        Ok(())
    }

    fn key_up(&mut self, key: Key) -> Result<()> {
        if self.wasd_mount_mode {
            if let Some(ref mut mount_display) = self.mount_display {
                match mount_display.key_up(key) {
                    Ok(()) => (),
                    Err(mount::thread::MountSendError {}) => self.mount_display = None,
                }
            }
            if self.mount_display.is_none() {
                self.wasd_mount_mode = false;
            }
        } else if self.wasd_camera_mode {
            self.camera_display.key_up(key);
        }
        Ok(())
    }

    fn key_down(&mut self, key: Key) -> Result<()> {
        if self.wasd_mount_mode {
            if key == Key::Escape {
                self.wasd_mount_mode = false;
            } else if let Some(ref mut mount_display) = self.mount_display {
                match mount_display.key_down(key) {
                    Ok(()) => (),
                    Err(mount::thread::MountSendError {}) => self.mount_display = None,
                }
                if self.mount_display.is_none() {
                    self.wasd_mount_mode = false;
                }
            }
        } else if self.wasd_camera_mode {
            if key == Key::Escape {
                self.wasd_camera_mode = false;
            } else {
                self.camera_display.key_down(key);
            }
        } else {
            self.text_input.key_down(key);
            self.run_cmd();
        }
        Ok(())
    }

    fn mouse_dragged(&mut self, _button: MouseButton, _delta: LogicalPosition<f64>) -> Result<()> {
        Ok(())
    }

    fn mouse_down(&mut self, _button: MouseButton) -> Result<()> {
        Ok(())
    }

    fn mouse_up(&mut self, _button: MouseButton) -> Result<()> {
        Ok(())
    }

    fn mouse_input(&mut self, state: ElementState, button: MouseButton) -> Result<()> {
        match state {
            ElementState::Pressed => {
                if self.pressed_mouse.insert(button) {
                    self.mouse_down(button)?;
                }
            }
            ElementState::Released => {
                if self.pressed_mouse.remove(&button) {
                    self.mouse_up(button)?;
                }
            }
        }
        Ok(())
    }

    fn mouse_wheel_y(&mut self, _delta: f64) -> Result<()> {
        Ok(())
    }

    fn mouse_wheel(&mut self, delta: MouseScrollDelta, _phase: TouchPhase) -> Result<()> {
        let pixels = match delta {
            MouseScrollDelta::LineDelta(_, y) => self.text_renderer.spacing as f64 * y as f64,
            MouseScrollDelta::PixelDelta(logical) => logical.y,
        };
        self.mouse_wheel_y(pixels)
    }

    fn mouse_moved(&mut self, position: PhysicalPosition<f64>) -> Result<()> {
        if let Some(last_mouse) = self.last_mouse {
            let delta = LogicalPosition::new(position.x - last_mouse.x, position.y - last_mouse.y);
            // dumb
            if self.pressed_mouse.contains(&MouseButton::Left) {
                self.mouse_dragged(MouseButton::Left, delta)?;
            }
            if self.pressed_mouse.contains(&MouseButton::Right) {
                self.mouse_dragged(MouseButton::Right, delta)?;
            }
            if self.pressed_mouse.contains(&MouseButton::Middle) {
                self.mouse_dragged(MouseButton::Middle, delta)?;
            }
        }
        self.last_mouse = Some(position);
        Ok(())
    }

    fn received_character(&mut self, ch: char) -> Result<()> {
        if !self.wasd_mount_mode && !self.wasd_camera_mode {
            self.text_input.received_character(ch);
        }
        Ok(())
    }

    fn user_update(&mut self, user_update: UserUpdate) -> Result<()> {
        match user_update {
            UserUpdate::MountUpdate(_) => {
                if let Some(ref mut mount_display) = self.mount_display {
                    mount_display.user_update(user_update);
                }
            }
            _ => {
                self.camera_display
                    .user_update(user_update, &mut self.mount_display)?;
            }
        }
        Ok(())
    }
}

fn handle(control_flow: &mut ControlFlow, res: Result<()>) {
    match res {
        Ok(()) => (),
        Err(err) => {
            println!("{:?}", err);
            *control_flow = ControlFlow::Exit;
        }
    }
}

fn main() -> Result<()> {
    let el = EventLoop::with_user_event();
    let wb = WindowBuilder::new()
        .with_title("clam5")
        .with_inner_size(glutin::dpi::LogicalSize::new(800.0, 800.0));
    let windowed_context = ContextBuilder::new()
        .with_gl_profile(GlProfile::Core)
        .with_vsync(true)
        .build_windowed(wb, &el)?;

    let windowed_context = unsafe { windowed_context.make_current().map_err(|(_, e)| e)? };

    let initial_size = windowed_context.window().inner_size();

    gl::load_with(|symbol| windowed_context.get_proc_address(symbol) as *const _);

    if !gl::GetError::is_loaded() {
        return Err("glGetError not loaded".into());
    }

    if cfg!(debug_assertions) {
        unsafe { gl::Enable(gl::DEBUG_OUTPUT_SYNCHRONOUS) };
        check_gl()?;
        gl_register_debug()?;
    }

    let proxy = el.create_proxy();

    let scale_factor = windowed_context.window().scale_factor();

    let mut display = Some(Display::setup(
        (initial_size.width as usize, initial_size.height as usize),
        scale_factor,
        proxy,
    )?);

    el.run(move |event, _, control_flow| match event {
        Event::WindowEvent { event, .. } => match event {
            WindowEvent::Resized(physical_size)
                if physical_size.width > 0 && physical_size.height > 0 =>
            {
                if let Some(ref mut display) = display {
                    let (width, height) = (physical_size.width, physical_size.height);
                    handle(
                        control_flow,
                        display.resize((width as usize, height as usize)),
                    );
                    unsafe { gl::Viewport(0, 0, width as i32, height as i32) };
                }
            }
            WindowEvent::KeyboardInput { input, .. } => {
                if let Some(ref mut display) = display {
                    if let Some(code) = input.virtual_keycode {
                        match input.state {
                            ElementState::Pressed => handle(control_flow, display.key_down(code)),
                            ElementState::Released => handle(control_flow, display.key_up(code)),
                        }
                        windowed_context.window().request_redraw();
                    }
                }
            }
            WindowEvent::MouseInput { state, button, .. } => {
                if let Some(ref mut display) = display {
                    handle(control_flow, display.mouse_input(state, button));
                }
            }
            WindowEvent::MouseWheel { delta, phase, .. } => {
                if let Some(ref mut display) = display {
                    handle(control_flow, display.mouse_wheel(delta, phase));
                }
            }
            WindowEvent::CursorMoved { position, .. } => {
                if let Some(ref mut display) = display {
                    handle(control_flow, display.mouse_moved(position));
                }
            }
            WindowEvent::CursorLeft { .. } => {
                if let Some(ref mut display) = display {
                    display.last_mouse = None
                }
            }
            WindowEvent::ReceivedCharacter(ch) => {
                if let Some(ref mut display) = display {
                    handle(control_flow, display.received_character(ch));
                    windowed_context.window().request_redraw();
                }
            }
            WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
            _ => (),
        },
        Event::UserEvent(user_update) => {
            if let Some(ref mut display) = display {
                handle(control_flow, display.user_update(user_update));
                windowed_context.window().request_redraw();
            }
        }
        Event::RedrawRequested(_) => {
            if let Some(ref mut display) = display {
                handle(control_flow, display.render());
                handle(
                    control_flow,
                    windowed_context.swap_buffers().map_err(|e| e.into()),
                );
            }
        }
        Event::MainEventsCleared => {
            if *control_flow == ControlFlow::Exit {
                display = None;
            } else if let Some(ref mut display) = display {
                match display.update() {
                    Ok((wait_until, redraw)) => {
                        if redraw {
                            windowed_context.window().request_redraw();
                        }
                        *control_flow = ControlFlow::WaitUntil(wait_until);
                    }
                    Err(err) => {
                        println!("{:?}", err);
                        *control_flow = ControlFlow::Exit;
                    }
                }
            } else {
                let wait_until = Instant::now() + Duration::from_millis(10);
                *control_flow = ControlFlow::WaitUntil(wait_until);
            }
        }
        _ => (),
    })
}
