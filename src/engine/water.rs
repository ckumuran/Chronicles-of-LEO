use glam::Vec2;

pub struct WaterRenderer {

    pub time: f32,

    pub wave_strength: f32,

    pub wave_speed: f32,

    pub uv_scale: f32,
}

impl WaterRenderer {

    pub fn new() -> Self {

        Self {

            time: 0.0,

            wave_strength: 0.08,

            wave_speed: 1.5,

            uv_scale: 4.0,
        }
    }

    pub fn update(
        &mut self,

        delta_time: f32,
    ) {

        self.time +=
            delta_time;
    }

    pub fn wave_height(

        &self,

        x: f32,

        z: f32,
    ) -> f32 {

        let wave1 =

            (x * 0.12 +
             self.time *
             self.wave_speed)

            .sin();

        let wave2 =

            (z * 0.08 +
             self.time *
             self.wave_speed *
             0.7)

            .cos();

        (wave1 + wave2)
        *
        self.wave_strength
    }

    pub fn uv_offset(
        &self,
    ) -> Vec2 {

        Vec2::new(

            self.time * 0.05,

            self.time * 0.03,
        )
    }
}
