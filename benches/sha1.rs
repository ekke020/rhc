use criterion::{black_box, criterion_group, criterion_main, Criterion};
use hash::sha1;

fn sha1_benchmark(c: &mut Criterion) {
    c.bench_function("sha1", |b| {
        b.iter(|| {
            let mut sha = sha1::Sha160::new();
            sha.load(black_box(b"Benchmark"));
            sha.run();
            sha.extract();
        })
    });
}

criterion_group!(benches, sha1_benchmark);
criterion_main!(benches);
