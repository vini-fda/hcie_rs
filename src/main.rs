use std::fmt::Debug;

use crate::rotate::roul;

mod rotate;

fn main() {
    println!("Hello, world!");
    // create a 2-dimensional array from closure
    let mut f = ndarray::Array::from_shape_fn((4, 5), |(i, j)| (i*5 + j) as u8);
    println!("f: {:?}", f);
    // rolr(f.view_mut(), 1, 1, 1);
    // println!("f: {:?}", f);
    // roud(f.view_mut(), 1, 1, 1);
    // println!("f: {:?}", f);
    roul(f.view_mut(), 2, 1, 1);
    println!("f: {:?}", f);
}
