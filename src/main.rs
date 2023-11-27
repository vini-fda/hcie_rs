mod rotate;
mod sub_hcie;
mod encrypt;
mod logistic;
mod img_array;

fn main() {
    // open imgs/peppers.tif
    let img = img_array::open_grayscale("imgs/peppers.tif");
    // convert to array
    let array = img_array::img_to_array(&img);

    let secret_key = logistic::SecretKey::new(0.1, 3.9999);
    // encrypt
    let encrypted = encrypt::encrypt(&array, 64, 64, &secret_key);
    // convert to img
    let encrypted_img = img_array::array_to_img(&encrypted);
    // save to peppers_encrypted.tif
    img_array::save_grayscale(&encrypted_img, "peppers_encrypted.png");

    // decryptio
    let decrypted = encrypt::decrypt(&encrypted, 64, 64, &secret_key);
    // convert to img
    let decrypted_img = img_array::array_to_img(&decrypted);
    // save to peppers_decrypted.tif
    img_array::save_grayscale(&decrypted_img, "peppers_decrypted.png");
}
