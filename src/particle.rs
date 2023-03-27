use nalgebra::{Point3, Vector3};

/// Massive particle.
#[derive(PartialEq)]
pub struct Particle {
    /// Mass.
    pub mass: f32,
    /// Position.
    pub pos: Point3<f32>,
    /// Velocity.
    pub vel: Vector3<f32>,
}

impl Particle {
    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(mass: f32, pos: Point3<f32>, vel: Vector3<f32>) -> Self {
        Self { mass, pos, vel }
    }
}
