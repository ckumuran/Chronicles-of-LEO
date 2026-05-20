use std::fs::File;
use std::io::BufReader;

use rodio::{
    Decoder,
    OutputStream,
    OutputStreamHandle,
    Sink,
};

pub struct AudioEngine {

    _stream: OutputStream,

    stream_handle:
        OutputStreamHandle,
}

impl AudioEngine {

    pub fn new() -> Self {

        let (
            stream,
            stream_handle
        ) =
            OutputStream::try_default()
                .unwrap();

        Self {

            _stream: stream,

            stream_handle,
        }
    }

    pub fn play_sound(
        &self,

        path: &str,
    ) {

        let file =
            match File::open(path)
        {

            Ok(file) => file,

            Err(_) => return,
        };

        let source =
            Decoder::new(
                BufReader::new(file)
            )
            .unwrap();

        let sink =
            Sink::try_new(
                &self.stream_handle
            )
            .unwrap();

        sink.append(source);

        sink.detach();
    }

    pub fn play_block_break(
        &self,
    ) {

        self.play_sound(
            "assets/audio/block_break.wav"
        );
    }

    pub fn play_footstep(
        &self,
    ) {

        self.play_sound(
            "assets/audio/footstep.wav"
        );
    }

    pub fn play_jump(
        &self,
    ) {

        self.play_sound(
            "assets/audio/jump.wav"
        );
    }

    pub fn play_ambient(
        &self,
    ) {

        self.play_sound(
            "assets/audio/ambient.wav"
        );
    }
}
