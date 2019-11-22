mod camera;
mod camera_display;
mod dms;
mod image_display;
mod mount;
mod mount_async;
mod mount_display;
mod platesolve;
mod process;
mod qhycamera;
mod text_input;

use camera_display::CameraDisplay;
use glutin::{
    self,
    event::{ElementState, Event, VirtualKeyCode as Key, WindowEvent},
    event_loop::{ControlFlow, EventLoop, EventLoopClosed},
    window::WindowBuilder,
    ContextBuilder, GlProfile,
};
use khygl::{
    check_gl, gl_register_debug, render_text::TextRenderer, render_texture::TextureRenderer,
    texture::CpuTexture, Rect,
};
use mount_async::MountAsync;
use mount_display::MountDisplay;
use std::{
    convert::TryInto,
    fmt::Write,
    fs::File,
    path::Path,
    time::{Duration, Instant},
};

type Result<T> = std::result::Result<T, failure::Error>;

pub enum UserUpdate {
    MountUpdate(mount_async::MountData),
}
type SendUserUpdate =
    dyn Fn(UserUpdate) -> std::result::Result<(), EventLoopClosed> + Send + 'static;

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
    camera_display: Option<camera_display::CameraDisplay>,
    mount_display: Option<mount_display::MountDisplay>,
    next_frequent_update: Instant,
    next_infrequent_update: Instant,
    window_size: (usize, usize),
    status: String,
    old_status: String,
    text_input: text_input::TextInput,
    texture_renderer: TextureRenderer,
    text_renderer: TextRenderer,
    wasd_mode: bool,
}

impl Display {
    fn window_size_f32(&self) -> (f32, f32) {
        (self.window_size.0 as f32, self.window_size.1 as f32)
    }

    fn run_cmd_impl(&mut self, text: &str) -> Result<()> {
        let cmd = text.split_whitespace().collect::<Vec<_>>();
        let mut command_okay = cmd.is_empty();
        if let ["wasd"] = &cmd as &[&str] {
            self.wasd_mode = true;
            command_okay |= true;
        }
        if let Some(ref mut camera_display) = self.camera_display {
            let mount = self.mount_display.as_mut().map(|m| &mut m.mount);
            command_okay |= camera_display.cmd(&cmd, mount)?;
        }
        if let Some(ref mut mount_display) = self.mount_display {
            command_okay |= mount_display.cmd(&cmd)?;
        }
        if command_okay {
            Ok(())
        } else {
            Err(failure::err_msg("Unknown command"))
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
}

impl Display {
    fn setup(
        camera: Option<camera::Camera>,
        mount: Option<mount::Mount>,
        input_error: String,
        command_okay: bool,
        window_size: (usize, usize),
        dpi: f64,
        send_user_update: Box<SendUserUpdate>,
    ) -> Result<Self> {
        let texture_renderer = TextureRenderer::new()?;
        let height = 20.0 * dpi as f32;
        let text_renderer = TextRenderer::new(height)?;
        let camera_display = Some(CameraDisplay::new(camera));
        let mount_display = mount
            .map(|m| MountAsync::new(m, send_user_update))
            .map(MountDisplay::new);
        let mut text_input = text_input::TextInput::new();
        text_input.set_exec_result(input_error, command_okay);
        Ok(Self {
            camera_display,
            mount_display,
            next_frequent_update: Instant::now(),
            next_infrequent_update: Instant::now(),
            window_size,
            status: String::new(),
            old_status: String::new(),
            text_input,
            texture_renderer,
            text_renderer,
            wasd_mode: false,
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
        if let Some(ref mut camera_display) = self.camera_display {
            camera_display.status(&mut self.status, infrequent_update)?;
        }
        if let Some(ref mut mount_display) = self.mount_display {
            mount_display.status(&mut self.status)?;
        }
        if self.wasd_mode {
            write!(
                &mut self.status,
                "WASD/RF mount control mode (esc to cancel)"
            )?;
        }
        if self.old_status != self.status {
            self.old_status = self.status.clone();
            redraw = true;
        }
        if let Some(ref mut camera_display) = self.camera_display {
            redraw |= camera_display.update()?;
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
        if let Some(ref mut camera_display) = self.camera_display {
            let width = (self.window_size.0 as isize - text_size.right() as isize)
                .try_into()
                .unwrap_or(1);
            let camera_rect = Rect::new(text_size.right(), 0, width, input_pos_y);
            camera_display.draw(camera_rect, &self.texture_renderer, window_size_f32)?;
        }
        Ok(())
    }

    fn resize(&mut self, size: (usize, usize)) -> Result<()> {
        self.window_size = size;
        Ok(())
    }

    fn key_up(&mut self, key: Key) -> Result<()> {
        if self.wasd_mode {
            if let Some(ref mut mount_display) = self.mount_display {
                mount_display.key_up(key)?;
            }
        }
        Ok(())
    }

    fn key_down(&mut self, key: Key) -> Result<()> {
        if self.wasd_mode {
            if key == Key::Escape {
                self.wasd_mode = false;
            } else if let Some(ref mut mount_display) = self.mount_display {
                mount_display.key_down(key)?;
            }
        } else {
            self.text_input.key_down(key);
            self.run_cmd();
        }
        Ok(())
    }

    fn received_character(&mut self, ch: char) -> Result<()> {
        if !self.wasd_mode {
            self.text_input.received_character(ch);
        }
        Ok(())
    }

    fn user_update(&mut self, user_update: UserUpdate) -> Result<()> {
        if let Some(ref mut mount_display) = self.mount_display {
            mount_display.user_update(&user_update);
        }
        Ok(())
    }
}

fn handle<T>(res: Result<T>) -> T {
    match res {
        Ok(ok) => ok,
        Err(err) => panic!("{:?}", err),
    }
}

fn main() -> Result<()> {
    let live = true;
    let mut command_okay = true;
    let mut input_error = String::new();
    let camera = match camera::autoconnect(live) {
        Ok(ok) => Some(ok),
        Err(err) => {
            command_okay = false;
            writeln!(&mut input_error, "Error connecting to camera: {}", err)?;
            None
        }
    };
    let mount = match mount::autoconnect() {
        Ok(ok) => Some(ok),
        Err(err) => {
            command_okay = false;
            writeln!(&mut input_error, "Error connecting to mount: {}", err)?;
            None
        }
    };

    let el = EventLoop::with_user_event();
    let wb = WindowBuilder::new()
        .with_title("clam5")
        .with_inner_size(glutin::dpi::LogicalSize::new(800.0, 800.0));
    let windowed_context = ContextBuilder::new()
        .with_gl_profile(GlProfile::Core)
        .with_vsync(true)
        .build_windowed(wb, &el)?;

    let windowed_context = unsafe { windowed_context.make_current().map_err(|(_, e)| e)? };

    let dpi = windowed_context.window().hidpi_factor();
    let initial_size = windowed_context.window().inner_size().to_physical(dpi);

    gl::load_with(|symbol| windowed_context.get_proc_address(symbol) as *const _);

    if !gl::GetError::is_loaded() {
        return Err(failure::err_msg("glGetError not loaded"));
    }

    if cfg!(debug_assertions) {
        unsafe { gl::Enable(gl::DEBUG_OUTPUT_SYNCHRONOUS) };
        check_gl()?;
        gl_register_debug()?;
    }

    let proxy = el.create_proxy();
    let send_user_update = Box::new(move |u| proxy.send_event(u));

    let mut display = Some(Display::setup(
        camera,
        mount,
        input_error,
        command_okay,
        (initial_size.width as usize, initial_size.height as usize),
        dpi,
        send_user_update,
    )?);

    el.run(move |event, _, control_flow| match event {
        Event::WindowEvent { event, .. } => match event {
            WindowEvent::Resized(logical_size)
                if logical_size.width > 0.0 && logical_size.height > 0.0 =>
            {
                if let Some(ref mut display) = display {
                    let dpi_factor = windowed_context.window().hidpi_factor();
                    let physical = logical_size.to_physical(dpi_factor);
                    handle(display.resize((physical.width as usize, physical.height as usize)));
                    unsafe { gl::Viewport(0, 0, physical.width as i32, physical.height as i32) };
                }
            }
            WindowEvent::KeyboardInput { input, .. } => {
                if let Some(ref mut display) = display {
                    if let Some(code) = input.virtual_keycode {
                        match input.state {
                            ElementState::Pressed => handle(display.key_down(code)),
                            ElementState::Released => handle(display.key_up(code)),
                        }
                        windowed_context.window().request_redraw();
                    }
                }
            }
            WindowEvent::ReceivedCharacter(ch) => {
                if let Some(ref mut display) = display {
                    handle(display.received_character(ch));
                    windowed_context.window().request_redraw();
                }
            }
            WindowEvent::RedrawRequested => {
                if let Some(ref mut display) = display {
                    handle(display.render());
                    handle(windowed_context.swap_buffers().map_err(|e| e.into()));
                }
            }
            WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
            _ => (),
        },
        Event::UserEvent(user_update) => {
            if let Some(ref mut display) = display {
                handle(display.user_update(user_update));
                windowed_context.window().request_redraw();
            }
        }
        Event::EventsCleared => {
            if *control_flow == ControlFlow::Exit {
                display = None;
            } else {
                let wait_until = if let Some(ref mut display) = display {
                    let (wait_until, redraw) = handle(display.update());
                    if redraw {
                        windowed_context.window().request_redraw();
                    }
                    wait_until
                } else {
                    Instant::now() + Duration::from_millis(10)
                };
                *control_flow = ControlFlow::WaitUntil(wait_until);
            }
        }
        _ => (),
    })
}
