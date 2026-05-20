use glam::{Mat4, Vec3};

use crate::engine::input::Input;

pub struct Camera {
    pub position: Vec3,

    pub yaw: f32,
    pub pitch: f32,

    pub front: Vec3,
    pub up: Vec3,

    speed: f32,
    sensitivity: f32,
}

impl Camera {
    pub fn new() -> Self {
        Self {
            position: Vec3::new(0.0, 10.0, 3.0),

            yaw: -90.0,
            pitch: 0.0,

            front: Vec3::new(0.0, 0.0, -1.0),
            up: Vec3::Y,

            speed: 10.0,
            sensitivity: 0.1,
        }
    }

    pub fn update(&mut self, input: &Input, delta_time: f32) {
        let velocity = self.speed * delta_time;

        let right = self.front.cross(self.up).normalize();

        if input.forward {
            self.position += self.front * velocity;
        }

        if input.backward {
            self.position -= self.front * velocity;
        }

        if input.left {
            self.position -= right * velocity;
        }

        if input.right {
            self.position += right * velocity;
        }

        self.yaw += input.mouse_dx * self.sensitivity;
        self.pitch += input.mouse_dy * self.sensitivity;

        self.pitch = self.pitch.clamp(-89.0, 89.0);

        self.update_vectors();
    }

    fn update_vectors(&mut self) {
        let yaw_radians = self.yaw.to_radians();
        let pitch_radians = self.pitch.to_radians();

        let front = Vec3 {
            x: yaw_radians.cos() * pitch_radians.cos(),
            y: pitch_radians.sin(),
            z: yaw_radians.sin() * pitch_radians.cos(),
        };

        self.front = front.normalize();
    }

    pub fn get_view_matrix(&self) -> Mat4 {
        Mat4::look_at_rh(
            self.position,
            self.position + self.front,
            self.up,
        )
    }
}
