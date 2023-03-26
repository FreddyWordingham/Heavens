use nalgebra::Vector3;

/// Massive particle.
pub struct Particle {
    /// Position.
    pub pos: Vector3<f64>,
}

impl Particle {
    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(pos: Vector3<f64>) -> Self {
        Self { pos }
    }
}
