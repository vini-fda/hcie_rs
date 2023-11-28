use ndarray::ArrayViewMut2;

use crate::{rotate::{rolr, roud, rour, roul}, logistic::{SecretKey, logistic_bitsequence}};

#[derive(Clone, Copy)]
pub enum Operation {
    Encrypt,
    Decrypt
}

pub struct SubHCIE {
    offset: usize,
    n_iter: usize,
    bit_sequence: Vec<u8>,
    alpha: usize,
    beta: usize,
    gamma: usize,
    op: Operation
}

impl SubHCIE {
    pub fn new(key: &SecretKey, n_iter: usize, op: Operation, alpha: usize, beta: usize, gamma: usize, bit_sequence: Vec<u8>, init_offset: usize) -> Self {
        Self {
            offset: init_offset,
            n_iter,
            bit_sequence,
            op,
            alpha,
            beta,
            gamma
        }
    }

    pub fn apply(&mut self, f: &mut ArrayViewMut2<u8>) {
        match self.op {
            Operation::Encrypt => self.encrypt(f),
            Operation::Decrypt => self.decrypt(f)
        }
    }

    /// Given a pseudo-random bit sequence b
    /// this function is used to permute an s_m x s_n matrix
    fn encrypt(&mut self, f: &mut ArrayViewMut2<u8>) {
        let s_m = f.shape()[0];
        let s_n = f.shape()[1];
        let b_len = self.bit_sequence.len();
        let bit = |i: usize| self.bit_sequence[i % b_len];
        for iter in 0..self.n_iter {
            let q = self.offset + (3*s_m + 3*s_n - 2) * iter;
            let p = self.alpha + self.beta * bit(q) as usize + self.gamma * bit(q + 1) as usize;
            for i in 0..s_m {
                rolr(f, i, p, bit(i + q));
            }
            for j in 0..s_n {
                roud(f, j, p, bit(j + q + s_m));
            }
            for k in 0..=(s_m + s_n - 2) {
                rour(f, k, p, bit(k + q + s_m + s_n));
            }
            for l in (-(s_n as isize) + 1)..=(s_m as isize - 1) {
                let temp = q + 2*s_m + 3*s_n;
                let temp = temp as isize;
                let temp = temp + l - 2;
                roul(f, l, p, bit(temp as usize));
            }
        }
        self.offset += (3*s_m + 3*s_n - 2) * self.n_iter
    }

    pub fn set_op(&mut self, op: Operation) {
        self.op = op;
    }

    pub fn set_offset(&mut self, offset: usize) {
        self.offset = offset;
    }

    fn decrypt(&mut self, f: &mut ArrayViewMut2<u8>) {
        let s_m = f.shape()[0];
        let s_n = f.shape()[1];
        let b_len = self.bit_sequence.len();
        let bit = |i: usize| self.bit_sequence[i % b_len];
        for iter in (0..self.n_iter).rev() {
            let q = self.offset + (3*s_m + 3*s_n - 2) * iter;
            let p = self.alpha + self.beta * bit(q) as usize + self.gamma * bit(q + 1) as usize;
            
            for l in ((-(s_n as isize) + 1)..=(s_m as isize - 1)).rev() {
                let temp = q + 2*s_m + 3*s_n;
                let temp = temp as isize;
                let temp = temp + l - 2;
                roul(f, l, p, bit(temp as usize) ^ 1);
            }
            for k in (0..=(s_m + s_n - 2)).rev() {
                rour(f, k, p, bit(k + q + s_m + s_n) ^ 1);
            }
            for j in (0..s_n).rev() {
                roud(f, j, p, bit(j + q + s_m) ^ 1);
            }
            for i in (0..s_m).rev() {
                rolr(f, i, p, bit(i + q) ^ 1);
            }
        }
        self.offset -= (3*s_m + 3*s_n - 2) * self.n_iter
    }
}