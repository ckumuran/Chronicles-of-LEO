use std::collections::HashMap;

use glam::Vec3;

pub type Entity = u32;

#[derive(Clone)]
pub struct Transform {

    pub position: Vec3,

    pub rotation: Vec3,

    pub scale: Vec3,
}

#[derive(Clone)]
pub struct Velocity {

    pub velocity: Vec3,
}

#[derive(Clone)]
pub struct Renderable {

    pub texture_id: u32,
}

pub struct ECS {

    next_entity: Entity,

    pub transforms:
        HashMap<Entity, Transform>,

    pub velocities:
        HashMap<Entity, Velocity>,

    pub renderables:
        HashMap<Entity, Renderable>,
}

impl ECS {

    pub fn new() -> Self {

        Self {

            next_entity: 0,

            transforms:
                HashMap::new(),

            velocities:
                HashMap::new(),

            renderables:
                HashMap::new(),
        }
    }

    pub fn create_entity(
        &mut self
    ) -> Entity {

        let entity =
            self.next_entity;

        self.next_entity += 1;

        entity
    }

    pub fn add_transform(
        &mut self,

        entity: Entity,

        transform: Transform,
    ) {

        self.transforms.insert(
            entity,
            transform,
        );
    }

    pub fn add_velocity(
        &mut self,

        entity: Entity,

        velocity: Velocity,
    ) {

        self.velocities.insert(
            entity,
            velocity,
        );
    }

    pub fn add_renderable(
        &mut self,

        entity: Entity,

        renderable: Renderable,
    ) {

        self.renderables.insert(
            entity,
            renderable,
        );
    }

    pub fn update(
        &mut self,

        delta_time: f32,
    ) {

        let entities:
            Vec<Entity> =
            self.velocities
                .keys()
                .cloned()
                .collect();

        for entity in entities {

            let velocity =
                match self.velocities
                    .get(&entity)
            {
                Some(v) => v.clone(),

                None => continue,
            };

            let transform =
                match self.transforms
                    .get_mut(&entity)
            {
                Some(t) => t,

                None => continue,
            };

            transform.position +=
                velocity.velocity *
                delta_time;
        }
    }
}
