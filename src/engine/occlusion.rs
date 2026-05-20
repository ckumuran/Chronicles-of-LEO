use glam::Vec3;

pub struct OcclusionCulling;

impl OcclusionCulling {

    pub fn chunk_visible(
        camera_position: Vec3,

        camera_front: Vec3,

        chunk_center: Vec3,
    ) -> bool {

        let to_chunk =
            (chunk_center - camera_position)
                .normalize();

        let dot =
            camera_front.dot(to_chunk);

        // Behind camera

        if dot < 0.15 {
            return false;
        }

        true
    }
}
