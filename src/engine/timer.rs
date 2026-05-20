use std::time::Instant;

pub struct Timer {

    last_frame: Instant,

    pub delta_time: f32,

    pub fps: u32,

    frame_counter: u32,

    fps_timer: f32,
}

impl Timer {

    pub fn new() -> Self {

        Self {

            last_frame:
                Instant::now(),

            delta_time: 0.0,

            fps: 0,

            frame_counter: 0,

            fps_timer: 0.0,
        }
    }

    pub fn update(
        &mut self,
    ) {

        let now =
            Instant::now();

        self.delta_time =
            now
                .duration_since(
                    self.last_frame
                )
                .as_secs_f32();

        self.last_frame = now;

        // FPS COUNTER

        self.frame_counter += 1;

        self.fps_timer +=
            self.delta_time;

        if self.fps_timer >= 1.0 {

            self.fps =
                self.frame_counter;

            self.frame_counter = 0;

            self.fps_timer = 0.0;
        }
    }
}
