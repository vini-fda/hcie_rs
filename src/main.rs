use std::fs::File;

use get_permutation_matrix::{apply_permutation_matrix, get_permutation_matrix};
use ndarray::Array2;

mod rotate;
mod sub_hcie;
mod encrypt;
mod logistic;
mod img_array;
mod get_permutation_matrix;

fn main() {
    let secret_key = logistic::SecretKey::new(0.1, 3.9999);
    
    // create array with file names
    let filenames = vec!["baboon", "female", "earth", "house", "peppers", "splash"];
    let s_m = 32;
    let s_n = 32;
    let mut original_imgs = vec![];
    let mut encrypted_imgs = vec![];
    for filename in filenames {
        // open image
        let img = img_array::open_grayscale(&format!("imgs_256/{}.png", filename));
        // convert to array
        let array = img_array::img_to_array(&img);
        original_imgs.push(array.clone());
        // encrypt
        let encrypted = encrypt::encrypt(&array, s_m, s_n, &secret_key);
        encrypted_imgs.push(encrypted.clone());
        assert!(is_permutation(&array, &encrypted));
        // convert to image
        let img = img_array::array_to_img(&encrypted);
        // save image
        img.save(&format!("imgs_256_encrypted/{}.png", filename)).unwrap();
    }
    println!("done encrypting");

    // use get_permutation_matrix::get_permutation_matrix;
    let (w, w_inv) = get_permutation_matrix(&original_imgs, &encrypted_imgs);
    print_matrix_to_txt(&w, "w.txt");
    print_matrix_to_txt(&w_inv, "w_inv.txt");
    let decrypted_first_img = apply_permutation_matrix(&w_inv, &encrypted_imgs[0]);
    // save image
    let img = img_array::array_to_img(&decrypted_first_img);
    img.save("imgs_256_decrypted/baboon.png").unwrap();
}

//when sorted, x and its permutation y must be equal
pub fn is_permutation(x: &Array2<u8>, y: &Array2<u8>) -> bool {
    // convert to Vec<u8>
    let mut x = x.iter().cloned().collect::<Vec<_>>();
    let mut y = y.iter().cloned().collect::<Vec<_>>();
    // sort
    x.sort();
    y.sort();
    // compare
    x == y
}

pub fn print_matrix_to_txt(m: &Array2<(usize, usize)>, filename: &str) {
    use std::io::Write;
    let mut file = File::create(filename).unwrap();
    //do not write as binary
    for i in 0..m.shape()[0] {
        for j in 0..m.shape()[1] {
            let (a, b) = m[[i,j]];
            write!(file, "{} {} ", a, b).unwrap();
        }
        writeln!(file).unwrap();
    }
}