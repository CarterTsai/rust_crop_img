extern crate image;

use image::{imageops};
use walkdir::WalkDir;

fn get_all_img() {
    for entry in WalkDir::new("./demo/src_img") {
        let entry = entry.unwrap();
        println!("{}", entry.path().display());
    }
}

fn main() {
    get_all_img();
    // Use the open function to load an image from a Path.
    // `open` returns a `DynamicImage` on success.
    let mut img = image::open("./demo/src_img/04wn01-21n_1.jpg").unwrap();

    let subimg = imageops::crop(&mut img, 0, 0, 400, 600);

    // Write the contents of this image to the Writer in PNG format.
    subimg.to_image().save("./demo/result_img/resize_test.png").unwrap();
}