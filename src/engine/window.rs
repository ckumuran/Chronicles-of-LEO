use glfw::{Action, Context, CursorMode, Key};

use crate::engine::camera::Camera;
use crate::engine::input::Input;
use crate::engine::renderer::Renderer;

use std::time::Instant;

pub struct Window {
    glfw: glfw::Glfw,
    window: glfw::PWindow,
    events: glfw::GlfwReceiver<(f64, glfw::WindowEvent)>,

    input: Input,
    camera: Camera,

    renderer: Renderer,

    last_frame: Instant,
}

impl Window {
    pub fn new(width: u32, height: u32, title: &str) -> Self {
        let mut glfw = glfw::init(glfw::fail_on_errors)
            .expect("Failed to initialize GLFW");

        glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));

        glfw.window_hint(glfw::WindowHint::OpenGlProfile(
            glfw::OpenGlProfileHint::Core,
        ));

        let (mut window, events) = glfw
            .create_window(
                width,
                height,
                title,
                glfw::WindowMode::Windowed,
            )
            .expect("Failed to create GLFW window");

        window.make_current();

        window.set_key_polling(true);
        window.set_cursor_pos_polling(true);

        window.set_cursor_mode(CursorMode::Disabled);

        gl::load_with(|symbol| {
            window
                .get_proc_address(symbol)
                .map_or(std::ptr::null(), |p| p as *const _)
        });

        unsafe {
            gl::Viewport(0, 0, width as i32, height as i32);

            gl::Enable(gl::DEPTH_TEST);
        }

        Self {
            glfw,
            window,
            events,

            input: Input::new(),
            camera: Camera::new(),

            renderer: Renderer::new(),

            last_frame: Instant::now(),
        }
    }

    pub fn should_close(&self) -> bool {
        self.window.should_close()
    }

    pub fn update(&mut self) {
        let now = Instant::now();

        let delta_time =
            now.duration_since(self.last_frame).as_secs_f32();

        self.last_frame = now;

        self.glfw.poll_events();

        for (_, event) in glfw::flush_messages(&self.events) {
            self.input.handle_event(&event);

            match event {
                glfw::WindowEvent::Key(
                    Key::Escape,
                    _,
                    Action::Press,
                    _,
                ) => {
                    self.window.set_should_close(true);
                }

                _ => {}
            }
        }

        self.camera.update(&self.input, delta_time);

        self.input.reset_mouse_delta();
    }

    pub fn render(&mut self) {
        unsafe {
            gl::ClearColor(0.4, 0.7, 1.0, 1.0);

            gl::Clear(
                gl::COLOR_BUFFER_BIT |
                gl::DEPTH_BUFFER_BIT
            );
        }

        self.renderer.render(&self.camera);

        self.window.swap_buffers();
    }
}
