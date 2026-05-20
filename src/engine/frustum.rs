use glam::{Mat4, Vec3, Vec4};

pub struct Frustum {

    planes: [Vec4; 6],
}

impl Frustum {

    pub fn new(
        projection: Mat4,
        view: Mat4,
    ) -> Self {

        let matrix =
            projection * view;

        let m =
            matrix.to_cols_array();

        let planes = [

            // LEFT

            Vec4::new(
                m[3] + m[0],
                m[7] + m[4],
                m[11] + m[8],
                m[15] + m[12],
            ),

            // RIGHT

            Vec4::new(
                m[3] - m[0],
                m[7] - m[4],
                m[11] - m[8],
                m[15] - m[12],
            ),

            // BOTTOM

            Vec4::new(
                m[3] + m[1],
                m[7] + m[5],
                m[11] + m[9],
                m[15] + m[13],
            ),

            // TOP

            Vec4::new(
                m[3] - m[1],
                m[7] - m[5],
                m[11] - m[9],
                m[15] - m[13],
            ),

            // NEAR

            Vec4::new(
                m[3] + m[2],
                m[7] + m[6],
                m[11] + m[10],
                m[15] + m[14],
            ),

            // FAR

            Vec4::new(
                m[3] - m[2],
                m[7] - m[6],
                m[11] - m[10],
                m[15] - m[14],
            ),
        ];

        Self {
            planes
        }
    }

    pub fn chunk_visible(
        &self,

        chunk_x: f32,
        chunk_y: f32,
        chunk_z: f32,
    ) -> bool {

        let min =
            Vec3::new(
                chunk_x,
                chunk_y,
                chunk_z,
            );

        let max =
            min + Vec3::splat(16.0);

        for plane in &self.planes {

            let px =
                if plane.x > 0.0 {
                    max.x
                } else {
                    min.x
                };

            let py =
                if plane.y > 0.0 {
                    max.y
                } else {
                    min.y
                };

            let pz =
                if plane.z > 0.0 {
                    max.z
                } else {
                    min.z
                };

            let distance =
                plane.x * px +
                plane.y * py +
                plane.z * pz +
                plane.w;

            if distance < 0.0 {
                return false;
            }
        }

        true
    }
}
