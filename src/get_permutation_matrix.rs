use std::collections::{HashSet, BTreeMap, BTreeSet};

use ndarray::Array2;
use sprs::CsMat;

/// Returns a permutation matrix W such that `y = W * x`.
/// 
/// We assume that all x's and y's have the same dimensions m x n,
/// and that every y is a permutation of x. In case this assumption
/// does not hold, the function will panic.
pub fn get_permutation_matrix(xs: &[Array2<u8>], ys: &[Array2<u8>]) -> (Array2::<(usize, usize)>, Array2::<(usize, usize)>) {
    let m = xs[0].shape()[0];
    let n = xs[0].shape()[1];
    let p = xs.len();
    // Lambda_k(l) = {(i,j) \in {0,1,... m-1} \times {0, 1, ..., n-1} | x_k(i,j) = l}
    // Lambda_k(l) is the set of indices where the value l appears in x_k
    // we will have p such sets, one for each x_k

    // we can use HashSet for every Lambda_k(l) to get O(1) lookup time
    // create lambdas as a closure (lazy evaluation)
    let lambda = |k: usize, l: u8| {
        (0..m).flat_map(|i| {
            (0..n).filter_map(move |j| {
                if ys[k][(i,j)] == l {
                    Some((i,j))
                } else {
                    None
                }
            })
        }).collect::<HashSet<_>>()
    };


    // permutation function W_ast is a m x n matrix of multiple values
    // W_ast(i,j) = intersection of all Lambda_k(x_k(i, j)) for all k and l
    // build w_ast as a Vec<((usize, usize), HashSet<(usize, usize)>)>
    let mut w_ast = (0..m).flat_map(|i| {
        (0..n).map(move |j| {
            let w_ast_ij = xs.iter().enumerate().take(p).fold(lambda(0, xs[0][(i,j)]), |w_ast_ij, (k, xs_k)| {
                w_ast_ij.intersection(&lambda(k, xs_k[(i,j)])).copied().collect()
            });
            ((i,j), w_ast_ij)
        })
    }).collect::<Vec<_>>();
    w_ast.sort_by(|(_, w_0), (_, w_1)| w_0.len().cmp(&w_1.len()));

    // determine a unique-valued matrix W from W_ast
    // such that W(i,j) \in W_ast(i,j) for all i,j
    // also, W(i,j) \neq W(i',j') for all (i,j) \neq (i',j')

    // we must sort the elements of W_ast in increasing order of their cardinality
    // because, if for example, W_ast(i,j) = {(0,0)}, then we must choose W(i,j) = (0,0)
    // and then remove that possibility from all other W_ast(i',j') where (i',j') \neq (i,j)
    let mut w = Array2::<(usize, usize)>::from_elem((m, n), (0,0));
    let mut w_inv = Array2::<(usize, usize)>::from_elem((m, n), (0,0));
    for mid in 1..w_ast.len() {
        let (prev, current_onwards) = w_ast.split_at_mut(mid);
        let ((i,j), w_ast_ij) = &current_onwards[0];
        let (i, j) = (*i, *j);
        // pick first element of w_ast_ij
        let idx = *w_ast_ij.iter().next().unwrap();
        w[(i, j)] = idx;
        w_inv[idx] = (i, j);
        // remove the current element from all other w_ast
        for (_, w_ast_ij) in prev.iter_mut() {
            w_ast_ij.remove(&idx);
        }
        // remove from onwards
        for (_, w_ast_ij) in current_onwards[1..].iter_mut() {
            w_ast_ij.remove(&idx);
        }
    }

    (w, w_inv)
}

/// Applies a permutation matrix W to a matrix x.
pub fn apply_permutation_matrix(w: &Array2::<(usize, usize)>, x: &Array2::<u8>) -> Array2::<u8> {
    let m = x.shape()[0];
    let n = x.shape()[1];
    let mut y = Array2::<u8>::zeros((m, n));
    for i in 0..m {
        for j in 0..n {
            let (i_prime, j_prime) = w[(i,j)];
            y[(i_prime, j_prime)] = x[(i,j)];
        }
    }
    y
}
