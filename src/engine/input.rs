use glfw::{Action, Key, WindowEvent};

pub struct Input {
    pub forward: bool,
    pub backward: bool,
    pub left: bool,
    pub right: bool,

    pub mouse_dx: f32,
    pub mouse_dy: f32,

    last_mouse_x: f64,
    last_mouse_y: f64,

    first_mouse: bool,
}

impl Input {
    pub fn new() -> Self {
        Self {
            forward: false,
            backward: false,
            left: false,
            right: false,

            mouse_dx: 0.0,
            mouse_dy: 0.0,

            last_mouse_x: 0.0,
            last_mouse_y: 0.0,

            first_mouse: true,
        }
    }

    pub fn handle_event(&mut self, event: &WindowEvent) {
        match event {
            WindowEvent::Key(Key::W, _, action, _) => {
                self.forward = *action != Action::Release;
            }

            WindowEvent::Key(Key::S, _, action, _) => {
                self.backward = *action != Action::Release;
            }

            WindowEvent::Key(Key::A, _, action, _) => {
                self.left = *action != Action::Release;
            }

            WindowEvent::Key(Key::D, _, action, _) => {
                self.right = *action != Action::Release;
            }

            WindowEvent::CursorPos(x, y) => {
                if self.first_mouse {
                    self.last_mouse_x = *x;
                    self.last_mouse_y = *y;
                    self.first_mouse = false;
                }

                self.mouse_dx = (*x - self.last_mouse_x) as f32;
                self.mouse_dy = (self.last_mouse_y - *y) as f32;

                self.last_mouse_x = *x;
                self.last_mouse_y = *y;
            }

            _ => {}
        }
    }

    pub fn reset_mouse_delta(&mut self) {
        self.mouse_dx = 0.0;
        self.mouse_dy = 0.0;
    }
}
