use glam::Vec3;

pub struct Player {

    pub position: Vec3,

    pub velocity: Vec3,

    pub rotation: Vec3,

    pub on_ground: bool,

    pub flying: bool,

    pub speed: f32,

    pub sprint_multiplier: f32,

    pub selected_block: u8,
}

impl Player {

    pub fn new() -> Self {

        Self {

            position:
                Vec3::new(
                    8.0,
                    20.0,
                    8.0,
                ),

            velocity:
                Vec3::ZERO,

            rotation:
                Vec3::ZERO,

            on_ground: false,

            flying: false,

            speed: 6.0,

            sprint_multiplier: 1.8,

            selected_block: 1,
        }
    }

    pub fn update(
        &mut self,

        delta_time: f32,
    ) {

        // GRAVITY

        if !self.flying {

            self.velocity.y -=
                20.0 * delta_time;
        }

        // APPLY VELOCITY

        self.position +=
            self.velocity *
            delta_time;

        // SIMPLE FLOOR

        if self.position.y < 2.0 {

            self.position.y = 2.0;

            self.velocity.y = 0.0;

            self.on_ground = true;
        }
    }

    pub fn jump(
        &mut self,
    ) {

        if self.on_ground {

            self.velocity.y = 8.0;

            self.on_ground = false;
        }
    }

    pub fn toggle_flight(
        &mut self,
    ) {

        self.flying =
            !self.flying;
    }
}
