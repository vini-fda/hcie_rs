

// Secret key: (x_0, mu)
// function: f(x) = mu * x * (1 - x)
pub struct SecretKey {
    pub x_0: f64,
    pub mu: f64,
}

impl SecretKey {
    pub fn new(x_0: f64, mu: f64) -> Self {
        Self { x_0, mu }
    }
}

/// Generates a binary sequence from iterating the logistic map.
/// 
/// We take the floating point values with finite binary precision
/// and convert them to a binary sequence
pub fn logistic_bitsequence(key: &SecretKey, n_iter: usize) -> Vec<u8> {
    let mut x = key.x_0;
    let mut b = Vec::new();
    for _ in 0..n_iter {
        x = key.mu * x * (1.0 - x);
        binary_add(&mut b, x);
    }
    b
}

// extracts 8 bits following the decimal point
// and appends them to the binary sequence
#[inline(always)]
fn binary_add(b: &mut Vec<u8>, x: f64) {
    let mut x = x.fract();
    for _ in 0..8 {
        x *= 2.0;
        b.push(x.floor() as u8);
        x = x.fract();
    }
}