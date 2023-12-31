use ndarray::Array2;

use crate::{sub_hcie::{SubHCIE, Operation}, logistic::{SecretKey, logistic_bitsequence}};

fn hcie_apply(f_hcie: &mut Array2<u8>, f_table: &Array2<u8>, f: &Array2<u8>, sub_hcie: &mut SubHCIE) {
    let m = f_hcie.shape()[0];
    let n = f_hcie.shape()[1];
    let s_m = f_table.shape()[0];
    let s_n = f_table.shape()[1];
    let b_n = n / s_n;
    assert!(m % s_m == 0);
    assert!(n % s_n == 0);
    let mut order = 0;
    // define f_sub here as all zeros
    let mut f_sub = ndarray::Array::from_elem((s_m, s_n), 0);
    for i in 0..s_m {
        for j in 0..s_n {
            if f_table[[i,j]] != 0 {
                let dividend = (f_table[[i,j]] - 1) as usize;
                let p = dividend / b_n;
                let q = dividend % b_n;
                for x in 0..s_m {
                    for y in 0..s_n {
                        f_sub[[x, y]] = f[[s_m*p + x, s_n * q + y]];
                    }
                }
                sub_hcie.apply(&mut f_sub.view_mut()); 
                let r = order / b_n;
                let s = order % b_n;
                for x in 0..s_m {
                    for y in 0..s_n {
                        f_hcie[[s_m*r + x, s_n*s + y]] = f_sub[[x, y]];
                    }
                }
                order += 1;
            }
        }
    }
}

fn hcie_apply_rev(f: &mut Array2<u8>, f_table: &Array2<u8>, f_hcie: &Array2<u8>, sub_hcie: &mut SubHCIE) {
    let m = f.shape()[0];
    let n = f.shape()[1];
    let s_m = f_table.shape()[0];
    let s_n = f_table.shape()[1];
    let b_m = m / s_m;
    let b_n = n / s_n;
    assert!(m % s_m == 0);
    assert!(n % s_n == 0);
    let mut order = b_m * b_n;
    // define f_sub here as all zeros
    let mut f_sub = ndarray::Array::from_elem((s_m, s_n), 0);
    for i in (0..s_m).rev() {
        for j in (0..s_n).rev() {
            if f_table[[i,j]] != 0 {
                order -= 1;
                let dividend = (f_table[[i,j]] - 1) as usize;
                let p = dividend / b_n;
                let q = dividend % b_n;
                for x in 0..s_m {
                    for y in 0..s_n {
                        f_sub[[x, y]] = f_hcie[[s_m*p + x, s_n * q + y]];
                    }
                }
                sub_hcie.apply(&mut f_sub.view_mut()); 
                let r = order / b_n;
                let s = order % b_n;
                for x in 0..s_m {
                    for y in 0..s_n {
                        f[[s_m*r + x, s_n*s + y]] = f_sub[[x, y]];
                    }
                }
            }
        }
    }
}

fn pseudoimage(m: usize, n: usize, s_m: usize, s_n: usize) -> Array2<u8> {
    assert!(m % s_m == 0);
    assert!(n % s_n == 0);
    let func = |x: usize, y: usize| {
        if x < m/s_m && y < n/s_n {
            x * (n / s_n) + y + 1
        } else {
            0
        }
    };
    ndarray::Array::from_shape_fn((s_m, s_n), |(i, j)| func(i, j) as u8)
}

pub fn encrypt(f: &Array2<u8>, s_m: usize, s_n: usize, key: &SecretKey) -> Array2<u8> {
    let m = f.shape()[0];
    let n = f.shape()[1];
    let n_iter = 2;
    let l_b = (1+ m/s_m * n/s_n) * n_iter * (3*s_m + 3*s_n - 2);
    let bit_sequence = logistic_bitsequence(key, l_b);
    let mut sub_hcie = SubHCIE::new(key, n_iter, Operation::Encrypt, 4, 2, 1, bit_sequence, 0);
    let mut f_table = pseudoimage(m, n, s_m, s_n);
    sub_hcie.apply(&mut f_table.view_mut());

    let mut f_hcie = ndarray::Array::from_elem((m, n), 0);
    hcie_apply(&mut f_hcie, &f_table, f, &mut sub_hcie);
    f_hcie
}

pub fn decrypt(f_hcie: &Array2<u8>, s_m: usize, s_n: usize, key: &SecretKey) -> Array2<u8> {
    let m = f_hcie.shape()[0];
    let n = f_hcie.shape()[1];
    let n_iter = 2;
    let l_b = (1+ m/s_m * n/s_n) * n_iter * (3*s_m + 3*s_n - 2);
    let bit_sequence = logistic_bitsequence(key, l_b);
    let mut sub_hcie = SubHCIE::new(key, n_iter, Operation::Encrypt, 1, 1, 1, bit_sequence, 0);
    let mut f_table = pseudoimage(m, n, s_m, s_n);
    sub_hcie.apply(&mut f_table.view_mut());
    let init_offset = (n/s_n * m/s_m + 1) * n_iter * (3*s_m + 3*s_n - 2);
    sub_hcie.set_offset(init_offset);
    sub_hcie.set_op(Operation::Decrypt);

    let mut f = ndarray::Array::from_elem((m, n), 0);
    hcie_apply_rev(&mut f, &f_table, f_hcie, &mut sub_hcie);
    f
}