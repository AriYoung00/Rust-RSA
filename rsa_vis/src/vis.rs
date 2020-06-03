use crate::rand;
use bmp::{Image, Pixel, px};

pub fn generate_rng_bitmap(img_size: u32) -> Image {
    let mut img = Image::new(img_size, img_size);

    let mut rng = rand::new();
    for (x, y) in img.coordinates() {
        let res = if rng.next_int(0, 2) == 0 { 0 } else { 255 };
        img.set_pixel(x, y, px!(res, res, res));
    }
    return img;
}