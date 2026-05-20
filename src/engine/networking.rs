use std::collections::HashMap;
use std::net::SocketAddr;

use glam::Vec3;

use serde::{
    Serialize,
    Deserialize,
};

use tokio::net::UdpSocket;

#[derive(
    Serialize,
    Deserialize,
    Clone,
)]
pub enum Packet {

    PlayerPosition {

        id: u32,

        position:
            (f32, f32, f32),
    },

    BlockUpdate {

        x: i32,
        y: i32,
        z: i32,

        block: u8,
    },

    PlayerJoin {

        id: u32,
    },

    PlayerLeave {

        id: u32,
    },
}

pub struct NetworkPlayer {

    pub id: u32,

    pub position: Vec3,
}

pub struct Server {

    socket: UdpSocket,

    pub players:
        HashMap<
            SocketAddr,
            NetworkPlayer
        >,
}

impl Server {

    pub async fn new(
        address: &str,
    ) -> Self {

        let socket =
            UdpSocket::bind(address)
                .await
                .unwrap();

        println!(
            "Server running on {}",
            address
        );

        Self {

            socket,

            players:
                HashMap::new(),
        }
    }

    pub async fn run(
        &mut self,
    ) {

        let mut buffer =
            [0u8; 2048];

        loop {

            let (
                size,
                address
            ) =
                self.socket
                    .recv_from(
                        &mut buffer
                    )
                    .await
                    .unwrap();

            let packet:
                Packet =
                bincode::deserialize(
                    &buffer[..size]
                ).unwrap();

            self.handle_packet(
                packet,
                address,
            ).await;
        }
    }

    async fn handle_packet(
        &mut self,

        packet: Packet,

        address: SocketAddr,
    ) {

        match packet {

            Packet::PlayerPosition {

                id,
                position,
            } => {

                let player =
                    self.players
                        .entry(address)
                        .or_insert(
                            NetworkPlayer {

                                id,

                                position:
                                    Vec3::ZERO,
                            }
                        );

                player.position =
                    Vec3::new(
                        position.0,
                        position.1,
                        position.2,
                    );

                self.broadcast(
                    &Packet::PlayerPosition {

                        id,

                        position,
                    }
                ).await;
            }

            Packet::BlockUpdate {

                x,
                y,
                z,
                block,
            } => {

                self.broadcast(
                    &Packet::BlockUpdate {

                        x,
                        y,
                        z,
                        block,
                    }
                ).await;
            }

            _ => {}
        }
    }

    async fn broadcast(
        &self,

        packet: &Packet,
    ) {

        let bytes =
            bincode::serialize(
                packet
            ).unwrap();

        for address
        in self.players.keys()
        {

            self.socket
                .send_to(
                    &bytes,
                    address,
                )
                .await
                .unwrap();
        }
    }
}
