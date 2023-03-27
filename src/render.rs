//! Create images from data.

use image::RgbImage;
use ndarray::{arr1, s, Array2, Array3};

/// Convert a 2D array of integers to an RGB image array.
pub fn image(data: Array2<u8>, max: u8) -> Array3<u8> {
    let mut cols = Array3::<u8>::zeros((data.shape()[0], data.shape()[1], 3));
    let max_inv = 1.0 / max as f64;
    let (width, height) = data.dim();
    for yi in 0..height {
        for xi in 0..width {
            let x = data[(xi, yi)] as f64 * max_inv;
            let col = [(x * 255.0) as u8, (x * 255.0) as u8, (x * 255.0) as u8];
            cols.slice_mut(s![xi, yi, ..]).assign(&arr1(&col));
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
