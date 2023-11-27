use ndarray::{ArrayViewMut1, ArrayViewMut2};

/// Rotates the `i`-th row of `f` by `p` positions
/// in the left or right direction (defined by `b`).
pub fn rolr(f: &mut ArrayViewMut2<u8>, i: usize, p: usize, b: u8) {
    let m = f.shape()[0]; // number of rows
    assert!(i < m);

    let mut g = f.row_mut(i);
    let n = g.len();
    let p = p % n;
    let g = g.as_slice_mut().unwrap();
    match b {
        0 => {
            g.rotate_left(p);
        }
        1 => {
            g.rotate_right(p);
        }
        _ => {
            panic!("Invalid direction");
        }
    }
}

/// Rotates the `j`-th column of `f` by `p` positions
/// in the up or down direction (defined by `b`).
pub fn roud(f: &mut ArrayViewMut2<u8>, j: usize, p: usize, b: u8) {
    let n = f.shape()[1];
    assert!(j < n);

    // g is not contiguous, because the array is row-major
    // therefore we cannot convert it to a slice trivially
    let g = f.column_mut(j);
    let m = g.len();
    let p = p % m;

    rotate(g, p, b);
}

/// Rotates all elements satisfying `i + j == k`,
/// in the lower left (when b = 0) or upper right (when b = 1) direction,
/// by `p` positions.
pub fn rour(f: &mut ArrayViewMut2<u8>, k: usize, p: usize, b: u8) {
    let m = f.shape()[0];
    let n = f.shape()[1];
    assert!(k <= m + n - 2);

    let g = f.as_slice_memory_order_mut().unwrap();

    let min_j = k.saturating_sub(m - 1);
    let max_j = k.min(n - 1);
    let min_i = k.saturating_sub(n - 1);
    let max_i = k.min(m - 1);
    let g_ptr = g.as_mut_ptr();
    let ptrs = (min_i..=max_i)
        .rev()
        .zip(min_j..=max_j)
        .map(|(i, j)| unsafe { g_ptr.add(i * n + j) })
        .collect::<Vec<_>>();

    rotate_ptrs(&ptrs, p, b);
}

/// Rotates all elements satisfying `i - j == l`,
/// in the upper left (when b = 0) or lower right (when b = 1) direction,
/// by `p` positions.
pub fn roul(f: &mut ArrayViewMut2<u8>, l: isize, p: usize, b: u8) {
    let m = f.shape()[0];
    let n = f.shape()[1];
    assert!(l >= 1 - n as isize, "l = {}, n = {}", l, n);
    assert!(l <= (m - 1) as isize);

    let g = f.as_slice_memory_order_mut().unwrap();
    let min_j = std::cmp::max(0, -l) as usize;
    let max_j = std::cmp::min(n - 1, (m as isize - 1 - l) as usize);
    let min_i = std::cmp::max(0, l) as usize;
    let max_i = std::cmp::min(m - 1, (l + n as isize - 1) as usize);

    let g_ptr = g.as_mut_ptr();
    let ptrs = (min_i..=max_i)
        .zip(min_j..=max_j)
        .map(|(i, j)| unsafe { g_ptr.add(i * n + j) })
        .collect::<Vec<_>>();

    rotate_ptrs(&ptrs, p, b);
}

fn rotate_ptrs<T>(ptrs: &[*mut T], p: usize, b: u8)
where
    T: Copy,
{
    let n = ptrs.len();
    let p = p % n;
    // println!("n = {}, p = {}", n, p);

    
    let p = if b == 0 { p } else { n - p };
    // if panic, print the following message

    let d = gcd(n, p);

    for i in 0..d {
        let temp = unsafe { *ptrs[i] };
        let mut j = i;

        loop {
            let k = (j + p) % n;
            if k == i {
                break;
            }

            unsafe {
                *ptrs[j] = *ptrs[k];
            }
            j = k;
        }

        unsafe {
            *ptrs[j] = temp;
        }
    }
}

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn rotate<T>(mut a: ArrayViewMut1<T>, p: usize, b: u8)
where
    T: Copy,
{
    let n = a.len();
    let p = if b == 0 { p } else { n - p };
    let d = gcd(n, p);

    for i in 0..d {
        let temp = a[i];
        let mut j = i;

        loop {
            let k = (j + p) % n;
            if k == i {
                break;
            }

            a[j] = a[k];
            j = k;
        }

        a[j] = temp;
    }
}
