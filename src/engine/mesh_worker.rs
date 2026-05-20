use std::thread;

use crossbeam_channel::{
    Receiver,
    Sender,
};

use crate::engine::chunk::Chunk;
use crate::engine::greedy_mesher::GreedyMesher;

pub struct MeshJob {

    pub position:
        (i32, i32, i32),

    pub chunk: Chunk,
}

pub struct MeshResult {

    pub position:
        (i32, i32, i32),

    pub vertices: Vec<f32>,
}

pub struct MeshWorker;

impl MeshWorker {

    pub fn start(
        receiver: Receiver<MeshJob>,

        sender: Sender<MeshResult>,
    ) {

        thread::spawn(move || {

            loop {

                let job =
                    match receiver.recv() {

                        Ok(job) => job,

                        Err(_) => break,
                    };

                let vertices =
                    GreedyMesher::build(
                        &job.chunk
                    );

                sender.send(
                    MeshResult {

                        position:
                            job.position,

                        vertices,
                    }
                ).unwrap();
            }
        });
    }
}
