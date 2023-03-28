//! Galaxy input parameters.

use nalgebra::Vector3;
use serde::Deserialize;

/// Complete information to construct an initial galaxy.
#[derive(Debug, Deserialize)]
pub struct Galaxy {
    /// Centre.
    pub pos: Vector3<f32>,
}
