use ndarray::Array2;

pub fn mean_block(x: &Array2<u8>, s_m: usize, s_n: usize) -> Array2<u8> {
    let m = x.shape()[0];
    let n = x.shape()[1];
    // create a m/s_m x n/s_n matrix with the mean value of each block
    let mut y = Array2::<u8>::zeros((m/s_m, n/s_n));
    for i in 0..m/s_m {
        for j in 0..n/s_n {
            let mut sum = 0.0;
            for k in 0..s_m {
                for l in 0..s_n {
                    sum += x[(i*s_m + k, j*s_n + l)] as f64;
                }
            }
            y[(i,j)] = (sum / (s_m * s_n) as f64) as u8;
        }
    }
    y
}