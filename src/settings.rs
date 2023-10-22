#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Settings {
    pub display_width: f32,
    pub display_height: f32,
    pub pixel_size: f32,
    pub zoom: f32,

    pub gravitational_constant: f32,
    pub time_step: f32,
    pub smoothing_length: f32,
}

impl Settings {
    pub fn is_valid(&self) -> bool {
        self.display_width > 0.0
            && self.display_height > 0.0
            && self.pixel_size > 0.0
            && self.zoom > 0.0
            && self.gravitational_constant > 0.0
            && self.time_step > 0.0
            && self.smoothing_length > 0.0
    }
}
