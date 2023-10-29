use nalgebra::{Matrix4, Unit, Vector3};

pub struct Camera {
    eye_position: Vector3<f32>,
    target_position: Vector3<f32>,
    upward_direction: Unit<Vector3<f32>>,

    fov_y: f32,
    aspect_ratio: f32, // Width / height
    near_clip: f32,
    far_clip: f32,
}

impl Camera {
    pub fn new(eye_position: [f32; 3], target_position: [f32; 3], fov_x: f32) -> Self {
        debug_assert!(fov_x > 0.0);

        let aspect_ratio = 16.0 / 16.0;
        let fov_y = fov_x * aspect_ratio;
        let near_clip = 0.1;
        let far_clip = 1.0e9;

        debug_assert!(near_clip < far_clip);

        Self {
            eye_position: Vector3::from(eye_position),
            target_position: Vector3::from(target_position),
            upward_direction: Vector3::z_axis(),
            fov_y,
            aspect_ratio,
            near_clip,
            far_clip,
        }
    }

    pub fn mvp(&self) -> Matrix4<f32> {
        self.look_at() * self.perspective()
    }

    fn look_at(&self) -> Matrix4<f32> {
        let forward = (self.target_position - self.eye_position).normalize();
        let right = self.upward_direction.cross(&forward).normalize();
        let actual_up = forward.cross(&right).normalize();

        let rotation = Matrix4::new(
            right.x,
            actual_up.x,
            forward.x,
            0.0,
            right.y,
            actual_up.y,
            forward.y,
            0.0,
            right.z,
            actual_up.z,
            forward.z,
            0.0,
            0.0,
            0.0,
            0.0,
            1.0,
        );

        let translation = Matrix4::new(
            1.0,
            0.0,
            0.0,
            -self.eye_position.x,
            0.0,
            1.0,
            0.0,
            -self.eye_position.y,
            0.0,
            0.0,
            1.0,
            -self.eye_position.z,
            0.0,
            0.0,
            0.0,
            1.0,
        );

        rotation * translation
    }

    fn perspective(&self) -> Matrix4<f32> {
        let f = 1.0 / (self.fov_y / 2.0).tan();
        let nf = 1.0 / (self.near_clip - self.far_clip);

        Matrix4::new(
            f / self.aspect_ratio,
            0.0,
            0.0,
            0.0,
            0.0,
            f,
            0.0,
            0.0,
            0.0,
            0.0,
            (self.far_clip + self.near_clip) * nf,
            2.0 * self.far_clip * self.near_clip * nf,
            0.0,
            0.0,
            -1.0,
            0.0,
        )
    }
}
