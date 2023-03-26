use crate::particle::Particle;

/// Collection of matter.
pub struct Galaxy {
    /// Massive particles.
    pub particles: Vec<Particle>,
}

impl Galaxy {
    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(n: usize) -> Self {
        let particles = Vec::with_capacity(n);

        Self { particles }
    }
}
