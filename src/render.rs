//! Create images from data.

use image::RgbImage;
use ndarray::{arr1, s, Array2, Array3};
use palette::{Gradient, LinSrgb, Pixel};

/// Convert a 2D array of integers to an RGB image array.
pub fn image(data: &Array2<f32>, max: f32, cmap: &Gradient<LinSrgb>) -> Array3<u8> {
    let mut cols = Array3::<u8>::zeros((data.shape()[0], data.shape()[1], 3));

    let max_inv = 1.0 / max;
    let (width, height) = data.dim();
    for yi in 0..height {
        for xi in 0..width {
            let x = (data[(xi, yi)] as f32 * max_inv).max(1.0);
            let col = cmap.get(x);
            let u8s: [u8; 3] = col.into_format().into_raw();
            cols.slice_mut(s![xi, yi, ..]).assign(&arr1(&u8s));
        }
    }

    cols
}

/// Encode an RGB image array as an image.
pub fn encode(arr: &Array3<u8>) -> RgbImage {
    let (width, height, _) = arr.dim();

    RgbImage::from_vec(
        height as u32,
        width as u32,
        arr.view().as_slice().unwrap().to_vec(),
    )
    .expect("Container should have the right size for the image dimensions.")
}
