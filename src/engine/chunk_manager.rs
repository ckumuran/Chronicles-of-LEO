use std::collections::{
    HashMap,
    HashSet,
};

use crossbeam_channel::{
    unbounded,
    Receiver,
    Sender,
};

use crate::engine::chunk::Chunk;
use crate::engine::chunk_render_data::ChunkRenderData;
use crate::engine::chunk_worker::{
    ChunkJob,
    ChunkResult,
    ChunkWorker,
};

pub const RENDER_DISTANCE: i32 = 6;

pub struct ChunkManager {

    pub chunks:
        HashMap<
            (i32, i32, i32),
            Chunk
        >,

    pub render_data:
        HashMap<
            (i32, i32, i32),
            ChunkRenderData
        >,

    generating:
        HashSet<(i32, i32)>,

    job_sender:
        Sender<ChunkJob>,

    result_receiver:
        Receiver<ChunkResult>,
}

impl ChunkManager {

    pub fn new() -> Self {

        let (
            job_sender,
            job_receiver,
        ) = unbounded();

        let (
            result_sender,
            result_receiver,
        ) = unbounded();

        ChunkWorker::start(
            job_receiver,
            result_sender,
        );

        Self {

            chunks: HashMap::new(),

            render_data: HashMap::new(),

            generating: HashSet::new(),

            job_sender,

            result_receiver,
        }
    }

    pub fn update(
        &mut self,

        player_chunk_x: i32,
        player_chunk_z: i32,
    ) {

        self.request_chunks(
            player_chunk_x,
            player_chunk_z,
        );

        self.receive_chunks();

        self.unload_chunks(
            player_chunk_x,
            player_chunk_z,
        );
    }

    fn request_chunks(
        &mut self,

        player_chunk_x: i32,
        player_chunk_z: i32,
    ) {

        for x in
            player_chunk_x - RENDER_DISTANCE
            ..
            =
            player_chunk_x + RENDER_DISTANCE
        {

            for z in
                player_chunk_z - RENDER_DISTANCE
                ..
                =
                player_chunk_z + RENDER_DISTANCE
            {

                let key = (x, z);

                if self.generating.contains(&key) {
                    continue;
                }

                if self.chunks.contains_key(
                    &(x, 0, z)
                ) {
                    continue;
                }

                self.generating.insert(key);

                self.job_sender.send(
                    ChunkJob {
                        x,
                        z,
                    }
                ).unwrap();
            }
        }
    }

    fn receive_chunks(
        &mut self,
    ) {

        while let Ok(result) =
            self.result_receiver.try_recv()
        {

            let key =
                (
                    result.x,
                    0,
                    result.z,
                );

            self.chunks.insert(
                key,
                result.chunk,
            );

            self.render_data.insert(
                key,

                ChunkRenderData {
                    mesh: None,
                }
            );

            self.generating.remove(
                &(result.x, result.z)
            );
        }
    }

    fn unload_chunks(
        &mut self,

        player_chunk_x: i32,
        player_chunk_z: i32,
    ) {

        self.chunks.retain(
            |&(x, _, z), _| {

                let dx =
                    (x - player_chunk_x).abs();

                let dz =
                    (z - player_chunk_z).abs();

                dx <= RENDER_DISTANCE &&
                dz <= RENDER_DISTANCE
            }
        );

        self.render_data.retain(
            |&(x, _, z), _| {

                let dx =
                    (x - player_chunk_x).abs();

                let dz =
                    (z - player_chunk_z).abs();

                dx <= RENDER_DISTANCE &&
                dz <= RENDER_DISTANCE
            }
        );
    }
}
