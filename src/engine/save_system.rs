use std::fs;
use std::path::Path;

use crate::engine::chunk::Chunk;

pub struct SaveSystem;

impl SaveSystem {

    pub fn save_chunk(
        chunk: &Chunk,
    ) {

        let directory =
            "world/chunks";

        fs::create_dir_all(
            directory
        ).unwrap();

        let filename =
            format!(
                "{}/{}_{}_{}.json",

                directory,

                chunk.position.0,
                chunk.position.1,
                chunk.position.2,
            );

        let json =
            serde_json::to_string(
                chunk
            ).unwrap();

        fs::write(
            filename,
            json,
        ).unwrap();
    }

    pub fn load_chunk(
        x: i32,
        y: i32,
        z: i32,
    ) -> Option<Chunk> {

        let filename =
            format!(
                "world/chunks/{}_{}_{}.json",

                x,
                y,
                z,
            );

        if !Path::new(
            &filename
        ).exists()
        {
            return None;
        }

        let json =
            fs::read_to_string(
                filename
            ).ok()?;

        serde_json::from_str(
            &json
        ).ok()
    }
}
