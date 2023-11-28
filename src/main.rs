mod rotate;
mod sub_hcie;
mod encrypt;
mod logistic;
mod img_array;

fn main() {
    let secret_key = logistic::SecretKey::new(0.1, 3.9999);
    
    // create array with file names
    let filenames = vec!["baboon", "female", "earth", "house", "peppers", "splash"];
    let s_m = 32;
    let s_n = 32;
    for filename in filenames {
        // open image
        let img = img_array::open_grayscale(&format!("imgs_256/{}.png", filename));
        // convert to array
        let array = img_array::img_to_array(&img);
        // encrypt
        let encrypted = encrypt::encrypt(&array, s_m, s_n, &secret_key);
        // convert to image
        let img = img_array::array_to_img(&encrypted);
        // save image
        img.save(&format!("imgs_256_encrypted/{}_encrypted.png", filename)).unwrap();
    }
}
