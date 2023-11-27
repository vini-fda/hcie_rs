use ndarray::ArrayViewMut2;

use crate::{rotate::{rolr, roud, rour, roul}, logistic::{SecretKey, logistic_bitsequence}};


pub struct SubHCIE {
    offset: usize,
    n_iter: usize,
    bit_sequence: Vec<u8>
}

impl SubHCIE {
    pub fn new(key: &SecretKey, n_iter: usize) -> Self {
        let bit_sequence = logistic_bitsequence(key, 100000);
        Self {
            offset: 0,
            n_iter,
            bit_sequence
        }
    }

    /// Given a pseudo-random bit sequence b
    /// this function is used to permute an s_m x s_n matrix
    pub fn apply(&mut self, f: &mut ArrayViewMut2<u8>) {
        let s_m = f.shape()[0];
        let s_n = f.shape()[1];
        const ALPHA: usize = 1;
        const BETA: usize = 1;
        const GAMMA: usize = 1;
        let b_len = self.bit_sequence.len();
        let bit = |i: usize| self.bit_sequence[i % b_len] as usize;
        for iter in 0..self.n_iter {
            let q = self.offset + (3*s_m + 3*s_n - 2) * iter;
            let p = ALPHA + BETA * bit(q) + GAMMA * bit(q + 1);
            for i in 0..s_m {
                rolr(f, i, p, bit(i + q) as u8);
            }
            for j in 0..s_n {
                roud(f, j, p, bit(j + q + s_m) as u8);
            }
            for k in 0..(s_m + s_n - 2) {
                rour(f, k, p, bit(k + q + s_m + s_n) as u8);
            }
            for l in (-(s_n as isize) + 1)..=(s_m as isize - 1) {
                let temp = q + 2*s_m + 3*s_n;
                let temp = temp as isize;
                let temp = temp + l - 2;
                roul(f, l, p, bit(temp as usize) as u8);
            }
        }
        self.offset += (3*s_m + 3*s_n - 2) * self.n_iter
    }
}