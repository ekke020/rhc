use criterion::{black_box, criterion_group, criterion_main, Criterion};
use hash::sha2;

fn sha2_224_benchmark(c: &mut Criterion) {
    c.bench_function("sha2_224", |b| {
        b.iter(|| {
            let mut sha = sha2::Sha224::new();
            sha.load(black_box(b"Benchmark"));
            sha.run();
            sha.extract();
        })
    });
}

fn sha2_256_benchmark(c: &mut Criterion) {
    c.bench_function("sha2_256", |b| {
        b.iter(|| {
            let mut sha = sha2::Sha256::new();
            sha.load(black_box(b"Benchmark"));
            sha.run();
            sha.extract();
        })
    });
}

fn sha2_384_benchmark(c: &mut Criterion) {
    c.bench_function("sha2_384", |b| {
        b.iter(|| {
            let mut sha = sha2::Sha384::new();
            sha.load(black_box(b"Benchmark"));
            sha.run();
            sha.extract();
        })
    });
}

fn sha2_512_benchmark(c: &mut Criterion) {
    c.bench_function("sha2_512", |b| {
        b.iter(|| {
            let mut sha = sha2::Sha512::new();
            sha.load(black_box(b"Benchmark"));
            sha.run();
            sha.extract();
        })
    });
}

fn sha2_512_224_benchmark(c: &mut Criterion) {
    c.bench_function("sha2_512_224", |b| {
        b.iter(|| {
            let mut sha = sha2::Sha512_224::new();
            sha.load(black_box(b"Benchmark"));
            sha.run();
            sha.extract();
        })
    });
}

fn sha2_512_256_benchmark(c: &mut Criterion) {
    c.bench_function("sha2_512_256", |b| {
        b.iter(|| {
            let mut sha = sha2::Sha512_256::new();
            sha.load(black_box(b"Benchmark"));
            sha.run();
            sha.extract();
        })
    });
}

criterion_group!(
    benches,
    sha2_224_benchmark,
    sha2_256_benchmark,
    sha2_384_benchmark,
    sha2_512_benchmark,
    sha2_512_224_benchmark,
    sha2_512_256_benchmark
);
criterion_main!(benches);
