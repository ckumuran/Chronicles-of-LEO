use glam::Vec3;

#[derive(
    Copy,
    Clone,
    PartialEq
)]
pub enum WeatherType {

    Clear,

    Rain,

    Storm,
}

pub struct WeatherSystem {

    pub current:
        WeatherType,

    pub intensity: f32,

    transition_timer: f32,
}

impl WeatherSystem {

    pub fn new() -> Self {

        Self {

            current:
                WeatherType::Clear,

            intensity: 0.0,

            transition_timer: 0.0,
        }
    }

    pub fn update(
        &mut self,

        delta_time: f32,
    ) {

        self.transition_timer +=
            delta_time;

        // CHANGE WEATHER
        // every ~60 seconds

        if self.transition_timer >
           60.0
        {

            self.transition_timer = 0.0;

            self.next_weather();
        }

        // SMOOTH INTENSITY

        let target =
            match self.current {

                WeatherType::Clear => 0.0,

                WeatherType::Rain => 0.6,

                WeatherType::Storm => 1.0,
            };

        self.intensity +=
            (target - self.intensity)
            *
            delta_time;
    }

    fn next_weather(
        &mut self,
    ) {

        self.current =
            match self.current {

                WeatherType::Clear =>
                    WeatherType::Rain,

                WeatherType::Rain =>
                    WeatherType::Storm,

                WeatherType::Storm =>
                    WeatherType::Clear,
            };
    }

    pub fn fog_multiplier(
        &self,
    ) -> f32 {

        1.0 -
        self.intensity * 0.4
    }

    pub fn ambient_tint(
        &self,
    ) -> Vec3 {

        match self.current {

            WeatherType::Clear =>
                Vec3::new(
                    1.0,
                    1.0,
                    1.0,
                ),

            WeatherType::Rain =>
                Vec3::new(
                    0.75,
                    0.8,
                    0.9,
                ),

            WeatherType::Storm =>
                Vec3::new(
                    0.45,
                    0.45,
                    0.55,
                ),
        }
    }
}
