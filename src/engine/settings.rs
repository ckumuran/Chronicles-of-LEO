#[derive(Clone)]
pub struct GraphicsSettings {

    pub render_distance: i32,

    pub fog_distance: f32,

    pub vsync: bool,

    pub fullscreen: bool,

    pub shadows: bool,
}

#[derive(Clone)]
pub struct AudioSettings {

    pub master_volume: f32,

    pub ambient_volume: f32,

    pub effects_volume: f32,
}

#[derive(Clone)]
pub struct GameplaySettings {

    pub mouse_sensitivity: f32,

    pub invert_y: bool,

    pub flying_enabled: bool,
}

#[derive(Clone)]
pub struct Settings {

    pub graphics:
        GraphicsSettings,

    pub audio:
        AudioSettings,

    pub gameplay:
        GameplaySettings,
}

impl Settings {

    pub fn new() -> Self {

        Self {

            graphics:
                GraphicsSettings {

                    render_distance: 8,

                    fog_distance: 140.0,

                    vsync: true,

                    fullscreen: false,

                    shadows: false,
                },

            audio:
                AudioSettings {

                    master_volume: 1.0,

                    ambient_volume: 0.5,

                    effects_volume: 1.0,
                },

            gameplay:
                GameplaySettings {

                    mouse_sensitivity: 0.12,

                    invert_y: false,

                    flying_enabled: true,
                },
        }
    }
}
