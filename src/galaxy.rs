use nalgebra::Vector3;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Galaxy {
    /// Centre.
    pub pos: [f32; 3],
}
