use std::thread;

use crossbeam_channel::{
    Receiver,
    Sender,
};

use crate::engine::chunk::Chunk;
use crate::engine::terrain::TerrainGenerator;

pub struct ChunkJob {

    pub x: i32,
    pub z: i32,
}

pub struct ChunkResult {

    pub x: i32,
    pub z: i32,

    pub chunk: Chunk,
}

pub struct ChunkWorker;

impl ChunkWorker {

    pub fn start(
        receiver: Receiver<ChunkJob>,

        sender: Sender<ChunkResult>,
    ) {

        thread::spawn(move || {

            let terrain =
                TerrainGenerator::new();

            loop {

                let job =
                    match receiver.recv() {

                        Ok(job) => job,

                        Err(_) => break,
                    };

                let chunk =
                    terrain.generate_chunk(
                        job.x,
                        0,
                        job.z,
                    );

                sender.send(
                    ChunkResult {

                        x: job.x,
                        z: job.z,

                        chunk,
                    }
                ).unwrap();
            }
        });
    }
}
