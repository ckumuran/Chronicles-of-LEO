pub struct DebugSettings {

    pub wireframe: bool,

    pub show_chunk_borders: bool,

    pub show_fps: bool,

    pub show_chunk_count: bool,

    pub show_player_position: bool,
}

impl DebugSettings {

    pub fn new() -> Self {

        Self {

            wireframe: false,

            show_chunk_borders: false,

            show_fps: true,

            show_chunk_count: true,

            show_player_position: true,
        }
    }

    pub fn toggle_wireframe(
        &mut self,
    ) {

        self.wireframe =
            !self.wireframe;

        unsafe {

            if self.wireframe {

                gl::PolygonMode(
                    gl::FRONT_AND_BACK,

                    gl::LINE,
                );

            } else {

                gl::PolygonMode(
                    gl::FRONT_AND_BACK,

                    gl::FILL,
                );
            }
        }
    }

    pub fn toggle_chunk_borders(
        &mut self,
    ) {

        self.show_chunk_borders =
            !self.show_chunk_borders;
    }
}
