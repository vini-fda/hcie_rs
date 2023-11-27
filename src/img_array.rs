use image::GrayImage;
use ndarray::Array2;

/// Converts GrayScale image to array of pixels
pub fn img_to_array(img: &GrayImage) -> Array2<u8> {
    let (width, height) = img.dimensions();
    let mut array = Array2::<u8>::zeros((width as usize, height as usize));
    for (x, y, pixel) in img.enumerate_pixels() {
        array[[x as usize, y as usize]] = pixel[0];
    }
    array
}

pub fn array_to_img(array: &Array2<u8>) -> GrayImage {
    let (width, height) = array.dim();
    let mut img = GrayImage::new(width as u32, height as u32);
    for (x, y, pixel) in img.enumerate_pixels_mut() {
        *pixel = image::Luma([array[[x as usize, y as usize]]]);
    }
    img
}

/// Open an image from a filepath
pub fn open_grayscale(path: &str) -> GrayImage {
    image::open(path).unwrap().to_luma8()
}

/// Save an image to a filepath
pub fn save_grayscale(img: &GrayImage, path: &str) {
    img.save(path).unwrap();
}