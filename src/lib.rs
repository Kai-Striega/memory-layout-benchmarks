pub fn linear(a: &Vec<f64>, lda: usize, b: &Vec<f64>, ldb: usize, c: &mut Vec<f64>, ldc: usize) -> () {
    for (ci, ai) in c.chunks_exact_mut(ldc).zip(a.chunks_exact(lda)) {
        for (aik, bk) in ai.iter().zip(b.chunks_exact(ldb)) {
            for (cij, bkj) in ci.iter_mut().zip(bk.iter()) {
                *cij += (*aik) * (*bkj);
            }
        }
    }
}

pub fn vec_of_vec(a: &Vec<Vec<f64>>, b: &Vec<Vec<f64>>, c: &mut Vec<Vec<f64>>) -> () {
    for (ci, ai) in c.iter_mut().zip(a.iter()) {
        for (aik, bk) in ai.iter().zip(b.iter()) {
            for (cij, bkj) in ci.iter_mut().zip(bk.iter()) {
                *cij += (*aik) * (*bkj);
            }
        }
    }
}

#[cfg(test)]
mod tests {}
