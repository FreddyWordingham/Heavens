//! Human readable parameters.

use serde::Deserialize;
use std::path::Path;

use crate::{Camera, Galaxy};

#[derive(Debug, Deserialize)]
pub struct Parameters {
    /// Output directory.
    pub output_dir: String,
    /// Number of steps.
    pub num_steps: usize,
    /// Sub-steps
    pub sub_steps: usize,
    /// Time step in seconds.
    pub dt: f32,
    /// Gravitational strength factor.
    pub grav_strength: f32,
    /// Minimum calculation distance between massive particles.
    pub smooth_length: f32,
    /// Galaxies.
    pub galaxies: Vec<Galaxy>,
    /// Cameras.
    pub cameras: Vec<Camera>,
}

impl Parameters {
    /// Read parameters from a JSON string.
    pub fn read(s: &str) -> Self {
        json5::from_str(s).expect("Failed to load parameters file, or could not construct Parameters struct from the JSON.")
    }

    /// Load parameters from a JSON file.
    pub fn load(path: &Path) -> Self {
        let s = std::fs::read_to_string(path).expect(&format!(
            "Failed to open parameters file at {}",
            path.display()
        ));
        Self::read(&s)
    }
}
