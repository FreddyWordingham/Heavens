//! Camera setup.

use nalgebra::Point2;
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
}
