use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId};
use rand::{thread_rng, Rng};
use rand::distributions::Uniform;

use layout::{linear, vec_of_vec};


fn memory_layout(c: &mut Criterion) {
    let uniform = Uniform::new(-1.0, 1.0);
    let mut group = c.benchmark_group("Memory Layout");

    for size in [256usize, 512usize, 1024usize, 2048usize, 4096usize].iter() {
        let n_elements = size * size;

        let a_linear: Vec<f64> = thread_rng().sample_iter(uniform).take(n_elements).collect();
        let b_linear: Vec<f64> = thread_rng().sample_iter(uniform).take(n_elements).collect();
        let mut c_linear: Vec<f64> = thread_rng().sample_iter(uniform).take(n_elements).collect();

        let a_vec_of_vec: Vec<Vec<f64>> = a_linear.clone().chunks_exact(*size).map(|x| x.to_vec()).collect();
        let b_vec_of_vec: Vec<Vec<f64>> = b_linear.clone().chunks_exact(*size).map(|x| x.to_vec()).collect();
        let mut c_vec_of_vec: Vec<Vec<f64>> = c_linear.clone().chunks_exact(*size).map(|x| x.to_vec()).collect();

        group.bench_with_input(
            BenchmarkId::new("linear", size), size,
            |b, &size| {
                b.iter(|| linear(&a_linear, size, &b_linear, size, &mut c_linear, size));
            },
        );
        group.bench_with_input(
            BenchmarkId::new("vec_of_vec", size), size,
            |b, &_size| {
                b.iter(|| vec_of_vec(&a_vec_of_vec, &b_vec_of_vec, &mut c_vec_of_vec));
            },
        );
    }
    group.finish();
}

criterion_group!(benches, memory_layout);
criterion_main!(benches);
