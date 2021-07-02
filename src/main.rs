use nalgebra::base::DMatrix;
use rand::distributions::Standard;
use rand::prelude::*;
use rand_pcg::Pcg64;

fn random_matrix(r: usize, c: usize) -> DMatrix<f64> {
    let rng = Pcg64::seed_from_u64(2);
    DMatrix::from_iterator(r, c, rng.sample_iter(Standard).take(r * c))
}

fn main() {
    let mat = random_matrix(5, 5);
    println!("{:?}", mat);
}
