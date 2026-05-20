use glam::{
    Vec3,
    Mat4,
};

use rand::Rng;

use crate::engine::shader::Shader;

pub struct Particle {

    pub position: Vec3,

    pub velocity: Vec3,

    pub lifetime: f32,

    pub size: f32,
}

pub struct ParticleSystem {

    pub particles:
        Vec<Particle>,
}

impl ParticleSystem {

    pub fn new() -> Self {

        Self {

            particles:
                Vec::new(),
        }
    }

    pub fn spawn_block_break(
        &mut self,

        position: Vec3,
    ) {

        let mut rng =
            rand::thread_rng();

        for _ in 0..20 {

            let velocity =
                Vec3::new(

                    rng.gen_range(
                        -2.0..2.0
                    ),

                    rng.gen_range(
                        1.0..5.0
                    ),

                    rng.gen_range(
                        -2.0..2.0
                    ),
                );

            self.particles.push(
                Particle {

                    position,

                    velocity,

                    lifetime: 1.0,

                    size: rng.gen_range(
                        0.05..0.15
                    ),
                }
            );
        }
    }

    pub fn update(
        &mut self,

        delta_time: f32,
    ) {

        for particle
        in
        &mut self.particles
        {

            // GRAVITY

            particle.velocity.y -=
                9.8 * delta_time;

            // MOVEMENT

            particle.position +=
                particle.velocity *
                delta_time;

            // LIFETIME

            particle.lifetime -=
                delta_time;
        }

        self.particles.retain(
            |p| p.lifetime > 0.0
        );
    }

    pub fn draw(
        &self,

        shader: &Shader,
    ) {

        shader.use_program();

        for particle
        in
        &self.particles
        {

            let model =
                Mat4::from_translation(
                    particle.position
                ) *
                Mat4::from_scale(
                    Vec3::splat(
                        particle.size
                    )
                );

            shader.set_mat4(
                "model",
                &model,
            );

            // TEMP:
            // cube mesh rendering later
        }
    }
}
