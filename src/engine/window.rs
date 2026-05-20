use glfw::{Action, Context, Key};

pub struct Window {
    glfw: glfw::Glfw,
    window: glfw::PWindow,
    events: glfw::GlfwReceiver<(f64, glfw::WindowEvent)>,
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
            .expect("Failed to create window");

        window.make_current();

        window.set_key_polling(true);

        gl::load_with(|symbol| {
            window.get_proc_address(symbol)
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
        }
    }

    pub fn should_close(&self) -> bool {
        self.window.should_close()
    }

    pub fn update(&mut self) {
        self.glfw.poll_events();

        for (_, event) in glfw::flush_messages(&self.events) {
            match event {
                glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                    self.window.set_should_close(true);
                }

                _ => {}
            }
        }
    }

    pub fn render(&mut self) {
        unsafe {
            gl::ClearColor(0.4, 0.7, 1.0, 1.0);

            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        self.window.swap_buffers();
    }
}
