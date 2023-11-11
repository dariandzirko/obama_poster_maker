mod otsu;

use otsu::{otsu, otsu_filter_background};

fn main() {
    let img = image::open("src/disorderly_start.png").unwrap().to_luma8();

    let otsu_img = otsu_filter_background(&img);
    otsu_img.save("background_removed.png");
    let otsu_img = otsu(&img);
    otsu_img.save("otsu.png");
}
