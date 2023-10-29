#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Settings {
    pub display_width: f32,
    pub display_height: f32,
    pub pixel_size: f32,

    pub zoom: f32,
    pub camera_x: f32,
    pub camera_y: f32,

    pub gravitational_constant: f32,
    pub time_step: f32,
    pub smoothing_length: f32,

    pub ghost_mass: f32,
    pub ghost_stack_visible_limit: f32,

    pub blur_radius: f32,

    pub mvp_xx: f32,
    pub mvp_xy: f32,
    pub mvp_xz: f32,
    pub mvp_xw: f32,
    pub mvp_yx: f32,
    pub mvp_yy: f32,
    pub mvp_yz: f32,
    pub mvp_yw: f32,
    pub mvp_zx: f32,
    pub mvp_zy: f32,
    pub mvp_zz: f32,
    pub mvp_zw: f32,
    pub mvp_wx: f32,
    pub mvp_wy: f32,
    pub mvp_wz: f32,
    pub mvp_ww: f32,
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
            && self.ghost_mass > 0.0
            && self.ghost_stack_visible_limit >= 1.0
            && self.blur_radius >= 0.0
    }

    pub fn as_slice(&self) -> &[f32] {
        unsafe { std::slice::from_raw_parts(self as *const _ as *const f32, 12 + 16) }
    }
}
