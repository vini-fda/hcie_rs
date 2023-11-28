use std::{fs::File, io::BufRead};

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
    for filename in filenames.iter() {
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
    for n in 1..=3 {
        let (w, w_inv) = get_permutation_matrix(&original_imgs[0..1], &encrypted_imgs[0..1]);
        // let w_inv = read_matrix_from_txt(256, 256, "w_inv.txt");

        // print_matrix_to_txt(&w, "w.txt");
        // print_matrix_to_txt(&w_inv, "w_inv.txt");
        // let decrypted_first_img = apply_permutation_matrix(&w_inv, &encrypted_imgs[0]);
        // // save image
        // let img = img_array::array_to_img(&decrypted_first_img);
        // img.save("imgs_256_decrypted/baboon.png").unwrap();

        // decrypt all imgs
        for (i, filename) in filenames.iter().enumerate() {
            let decrypted = apply_permutation_matrix(&w_inv, &encrypted_imgs[i]);
            // save image
            let img = img_array::array_to_img(&decrypted);
            img.save(&format!("imgs_256_decrypted/{}_{}.png", filename, n)).unwrap();
        }
    }
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

pub fn read_matrix_from_txt(m: usize, n: usize, filename: &str) -> Array2<(usize, usize)> {
    let file = File::open(filename).expect("Error opening file");
    let reader = std::io::BufReader::new(file);
    let mut w = Array2::<(usize, usize)>::from_elem((m, n), (0, 0));

    for (i, line) in reader.lines().enumerate() {
        let line = line.expect("Error reading line from file");
        let mut numbers = line.split_whitespace().map(|s| s.parse::<usize>());
        
        for j in 0..n {
            let x = numbers.next().expect("Error parsing number");
            let y = numbers.next().expect("Error parsing number");

            w[(i, j)] = (x.unwrap(), y.unwrap());
        }
    }

    w
}