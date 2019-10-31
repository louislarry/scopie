mod camera;
mod camera_display;
mod dms;
mod mount;
mod mount_display;
mod process;
mod qhycamera;

use camera_display::CameraDisplay;
use khygl::{
    display::Key,
    render_text::TextRenderer,
    render_texture::{TextureRendererF32, TextureRendererU8},
    texture::{CpuTexture, Texture},
    Rect,
};
use mount_display::MountDisplay;
use std::convert::TryInto;

type Result<T> = std::result::Result<T, failure::Error>;

/*
fn format_duration(duration: Duration) -> String {
    format!("{}.{:03}", duration.as_secs(), duration.subsec_millis())
}
*/

/*
fn display(camera: Option<camera::Camera>, mount: Option<mount::Mount>) -> Result<()> {
    let sdl = init().map_err(failure::err_msg)?;
    let video = sdl.video().map_err(failure::err_msg)?;
    // let input = video.text_input();
    let ttf = ttf::init()?;
    let font = ttf.load_font(find_font()?, 20).map_err(failure::err_msg)?;
    let window = video.window("Scopie", 400, 400).resizable().build()?;
    let mut canvas = window.into_canvas().present_vsync().build()?;
    let creator = canvas.texture_creator();
    let mut event_pump = sdl.event_pump().map_err(failure::err_msg)?;
    let mut camera_display = camera.map(CameraDisplay::new);
    let mut mount_display = mount.map(MountDisplay::new);
    let mut status = String::new();
    let mut input_text = String::new();
    let mut command_okay = true;
    // input.start();
    'outer: loop {
        while let Some(event) = event_pump.poll_event() {
            eprintln!("{:?}", event);
            match event {
                Event::Quit { .. } => break 'outer,
                Event::TextEditing {
                    text,
                    start,
                    length,
                    ..
                } => input_text.replace_range((start as usize)..((start + length) as usize), &text),
                Event::TextInput { text, .. } => input_text.push_str(&text),
                Event::KeyDown {
                    keycode: Some(keycode),
                    ..
                } => match keycode {
                    Keycode::Backspace => {
                        input_text.pop();
                    }
                    Keycode::Return => {
                        let cmd = input_text.split_whitespace().collect::<Vec<_>>();
                        command_okay = cmd.is_empty();
                        if let Some(ref mut camera_display) = camera_display {
                            command_okay |= camera_display.cmd(&cmd)?;
                        }
                        if let Some(ref mut mount_display) = mount_display {
                            command_okay |= mount_display.cmd(&cmd)?;
                        }
                        input_text.clear();
                    }
                    _ => (),
                },
                _ => (),
            }
        }

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        let (window_width, window_height) = canvas.output_size().map_err(failure::err_msg)?;
        status.clear();
        //status.push_str("wau");
        if let Some(ref mut camera_display) = camera_display {
            status.push_str(camera_display.status()?);
        }
        if let Some(ref mut mount_display) = mount_display {
            status.push_str(mount_display.status()?);
        }
        let text_size = render_block(
            &font,
            &creator,
            &mut canvas,
            Color::RGB(255, 128, 128),
            Point::new(10, 10),
            &status,
        )?;
        if let Some(ref mut camera_display) = camera_display {
            let width = (window_width as i32 - text_size.right())
                .try_into()
                .unwrap_or(1);
            let camera_rect = Rect::new(text_size.right(), 0, width, window_height);
            camera_display.draw(&mut canvas, &creator, camera_rect)?;
        }
        let input_height = window_height as i32 - font.recommended_line_spacing();
        let input_pos = Point::new(10, input_height.try_into().unwrap_or(0));
        render_line(
            &font,
            &creator,
            &mut canvas,
            Color::RGB(255, 128, 128),
            input_pos,
            &input_text,
        )?;
        if command_okay {
            canvas.set_draw_color(Color::RGB(128, 128, 128));
        } else {
            canvas.set_draw_color(Color::RGB(255, 255, 255));
        }
        canvas
            .draw_rect(Rect::new(
                input_pos.x() - 1,
                input_pos.y() - 1,
                (window_width as i32 - input_pos.x() * 2)
                    .try_into()
                    .unwrap_or(2),
                font.recommended_line_spacing() as u32,
            ))
            .map_err(failure::err_msg)?;
        canvas.present();
    }
    // input.stop();
    Ok(())
}
*/

struct Display {
    camera_display: Option<camera_display::CameraDisplay>,
    mount_display: Option<mount_display::MountDisplay>,
    window_size: (usize, usize),
    status: String,
    input_text: String,
    texture_renderer_u8: TextureRendererU8,
    texture_renderer_f32: TextureRendererF32,
    text_renderer: TextRenderer,
    texture1x1: Texture<[u8; 4]>,
    command_okay: bool,
}

impl Display {
    fn window_size_f32(&self) -> (f32, f32) {
        (self.window_size.0 as f32, self.window_size.1 as f32)
    }

    fn line_x(&self, x_start: usize, x_end: usize, y: usize, color: [f32; 4]) -> Result<()> {
        self.texture_renderer_u8.render(
            &self.texture1x1,
            None,
            Rect::new(x_start as f32, y as f32, (x_end - x_start) as f32, 1.0),
            color,
            self.window_size_f32(),
        )
    }

    fn line_y(&self, x: usize, y_start: usize, y_end: usize, color: [f32; 4]) -> Result<()> {
        self.texture_renderer_u8.render(
            &self.texture1x1,
            None,
            Rect::new(x as f32, y_start as f32, 1.0, (y_end - y_start) as f32),
            color,
            self.window_size_f32(),
        )
    }

    fn rect(&self, rect: Rect<usize>, color: [f32; 4]) -> Result<()> {
        self.line_x(rect.x, rect.right(), rect.y, color)?;
        self.line_x(rect.x, rect.right(), rect.bottom(), color)?;
        self.line_y(rect.x, rect.y, rect.bottom(), color)?;
        self.line_y(rect.right(), rect.y, rect.bottom(), color)?;
        Ok(())
    }
}

impl khygl::display::Display for Display {
    fn setup(window_size: (usize, usize)) -> Result<Self> {
        let texture_renderer_u8 = TextureRendererU8::new()?;
        let texture_renderer_f32 = TextureRendererF32::new()?;
        let text_renderer = TextRenderer::new()?;
        let mut texture1x1 = Texture::new((1, 1))?;
        texture1x1.upload(&CpuTexture::new(vec![[255, 255, 255, 255]], (1, 1)))?;
        let live = false;
        let camera = match camera::autoconnect(live) {
            Ok(ok) => {
                println!("Using camera: {}", ok.name());
                Some(ok)
            },
            Err(err) => {
                println!("Error connecting to camera: {}", err);
                None
            }
        };
        let mount = match mount::autoconnect() {
            Ok(ok) => Some(ok),
            Err(err) => {
                println!("Error connecting to mount: {}", err);
                None
            }
        };
        let camera_display = camera.map(CameraDisplay::new);
        let mount_display = mount.map(MountDisplay::new);
        Ok(Self {
            camera_display,
            mount_display,
            window_size,
            status: String::new(),
            input_text: String::new(),
            texture_renderer_u8,
            texture_renderer_f32,
            text_renderer,
            texture1x1,
            command_okay: true,
        })
    }

    fn render(&mut self) -> Result<()> {
        let window_size_f32 = self.window_size_f32();
        unsafe {
            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }
        self.status.clear();
        if let Some(ref mut camera_display) = self.camera_display {
            self.status.push_str(camera_display.status()?);
        }
        if let Some(ref mut mount_display) = self.mount_display {
            self.status.push_str(mount_display.status()?);
        }
        let text_size = self.text_renderer.render(
            &self.texture_renderer_f32,
            &self.status,
            [1.0, 1.0, 1.0, 1.0],
            (10, 10),
            self.window_size,
        )?;
        if let Some(ref mut camera_display) = self.camera_display {
            let width = (self.window_size.0 as isize - text_size.right() as isize)
                .try_into()
                .unwrap_or(1);
            let camera_rect = Rect::new(text_size.right(), 0, width, self.window_size.1);
            camera_display.draw(camera_rect, &self.texture_renderer_u8, window_size_f32)?;
        }
        let input_height = self.window_size.1 as isize - self.text_renderer.spacing as isize;
        let input_pos = (10, input_height.try_into().unwrap_or(0));
        self.text_renderer.render(
            &self.texture_renderer_f32,
            &self.input_text,
            [1.0, 1.0, 1.0, 1.0],
            input_pos,
            self.window_size,
        )?;
        let command_color = if self.command_okay {
            [0.5, 0.5, 0.5, 1.0]
        } else {
            [1.0, 0.5, 0.5, 1.0]
        };
        self.rect(
            Rect::new(
                input_pos.0,
                input_pos.1,
                (self.window_size.0 as isize - input_pos.0 as isize * 2)
                    .try_into()
                    .unwrap_or(2),
                self.text_renderer.spacing,
            ),
            command_color,
        )?;
        Ok(())
    }

    fn resize(&mut self, size: (usize, usize)) -> Result<()> {
        self.window_size = size;
        Ok(())
    }

    fn key_up(&mut self, _key: Key) -> Result<()> {
        Ok(())
    }

    fn key_down(&mut self, key: Key) -> Result<()> {
        match key {
            Key::Back => {
                self.input_text.pop();
            }
            Key::Return => {
                let cmd = self.input_text.split_whitespace().collect::<Vec<_>>();
                self.command_okay = cmd.is_empty();
                if let Some(ref mut camera_display) = self.camera_display {
                    self.command_okay |= camera_display.cmd(&cmd)?;
                }
                if let Some(ref mut mount_display) = self.mount_display {
                    self.command_okay |= mount_display.cmd(&cmd)?;
                }
                self.input_text.clear();
            }
            _ => (),
        }
        Ok(())
    }

    fn received_character(&mut self, ch: char) -> Result<()> {
        self.input_text.push(ch);
        Ok(())
    }
}

fn main() {
    match khygl::display::run::<Display>((600.0, 600.0)) {
        Ok(ok) => ok,
        Err(err) => println!("Error: {:?}", err),
    }
}
