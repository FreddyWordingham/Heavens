//! Camera setup.

use nalgebra::Point2;
use ndarray::Array2;
use serde::Deserialize;

/// Complete information to construct an rendering object.
#[derive(Clone, Debug, Deserialize)]
pub struct Camera {
    /// Centre [x, y].
    pub centre: Point2<f32>,
    /// Scale.
    pub scale: f32,
    /// Resolution.
    pub res: usize,
    /// Normalisation.
    pub norm: f32,
}

impl Camera {
    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn render(&self, pos: &[f32]) -> Array2<f32> {
        let delta = self.scale / self.res as f32;
        let num_particles = pos.len() / 3;

        let mut count = Array2::zeros((self.res, self.res));
        for i in 0..num_particles {
            let x = pos[(3 * i) + 0];
            let y = pos[(3 * i) + 1];
            // let z = pos[(3 * i) + 2];

            let xi = ((x - self.centre.x) / delta) as usize;
            let yi = ((y - self.centre.y) / delta) as usize;

            if xi < self.res && yi < self.res {
                count[(xi, yi)] += 1.0;
            }
        }

        let norm_inv = 1.0 / self.norm;
        count *= norm_inv;
        count.mapv(|v| v.max(1.0));

        count
    }

    /// Count the number of particles in the field of view.
    #[inline]
    #[must_use]
    pub fn count(&self, pos: &[f32]) -> Array2<u32> {
        let delta = self.scale / self.res as f32;
        let num_particles = pos.len() / 3;

        let mut count = Array2::zeros((self.res, self.res));
        for i in 0..num_particles {
            let x = pos[(3 * i) + 0];
            let y = pos[(3 * i) + 1];
            // let z = pos[(3 * i) + 2];

            let xi = ((x - self.centre.x) / delta) as usize;
            let yi = ((y - self.centre.y) / delta) as usize;

            if xi < self.res && yi < self.res {
                count[(xi, yi)] += 1;
            }
        }
        count
    }
}
