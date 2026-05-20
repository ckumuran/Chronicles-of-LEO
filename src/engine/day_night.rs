use glam::Vec3;

pub struct DayNightCycle {

    pub time: f32,

    pub day_speed: f32,
}

impl DayNightCycle {

    pub fn new() -> Self {

        Self {

            time: 0.3,

            day_speed: 0.01,
        }
    }

    pub fn update(
        &mut self,

        delta_time: f32,
    ) {

        self.time +=
            delta_time *
            self.day_speed;

        if self.time >= 1.0 {

            self.time = 0.0;
        }
    }

    pub fn sunlight_strength(
        &self,
    ) -> f32 {

        let angle =
            self.time *
            std::f32::consts::TAU;

        angle.sin() * 0.5 + 0.5
    }

    pub fn ambient_color(
        &self,
    ) -> Vec3 {

        let sunlight =
            self.sunlight_strength();

        // NIGHT

        if sunlight < 0.15 {

            return Vec3::new(
                0.03,
                0.03,
                0.08,
            );
        }

        // SUNRISE/SUNSET

        if sunlight < 0.4 {

            return Vec3::new(
                0.9,
                0.45,
                0.25,
            );
        }

        // DAY

        Vec3::new(
            0.55,
            0.75,
            0.95,
        )
    }

    pub fn fog_color(
        &self,
    ) -> Vec3 {

        let sunlight =
            self.sunlight_strength();

        if sunlight < 0.2 {

            return Vec3::new(
                0.02,
                0.02,
                0.05,
            );
        }

        if sunlight < 0.4 {

            return Vec3::new(
                0.8,
                0.45,
                0.3,
            );
        }

        Vec3::new(
            0.6,
            0.75,
            0.95,
        )
    }
}
