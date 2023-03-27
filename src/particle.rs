use nalgebra::{Point3, Vector3};

/// Massive particle.
#[derive(PartialEq)]
pub struct Particle {
    /// Mass.
    pub mass: f64,
    /// Position.
    pub pos: Point3<f64>,
    /// Velocity.
    pub vel: Vector3<f64>,
}

impl Particle {
    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(mass: f64, pos: Point3<f64>, vel: Vector3<f64>) -> Self {
        Self { mass, pos, vel }
    }
}
