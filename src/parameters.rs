use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Parameters {
    /// Gravitational strength factor.
    grav_strength: f32,

    /// Minimum calculation distance between massive particles.
    smoothing_length: f32,

    /// Colour map.
    cmap: Vec<String>,
}

impl Parameters {
    /// Load parameters from a JSON file.
    pub fn load(path: String) -> Self {
        json5::from_str(&path).unwrap()
    }
}
