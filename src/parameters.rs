use serde::Deserialize;
use std::path::Path;

#[derive(Debug, Deserialize)]
pub struct Parameters {
    /// Gravitational strength factor.
    gravitational_strength: f32,

    /// Minimum calculation distance between massive particles.
    smoothing_length: f32,

    /// Colour map.
    cmap: Vec<String>,
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
