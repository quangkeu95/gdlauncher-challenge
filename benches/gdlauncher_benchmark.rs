use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use gdlauncher_challenge::{find_unsecure_numbers, utils::FileUtils};

pub fn criterion_benchmark(c: &mut Criterion) {
    let vector: Vec<u128> = FileUtils::parse_text_file("challenge_input.txt").unwrap();

    c.bench_with_input(
        BenchmarkId::new("Find unsecure number", "challenge_input.txt"),
        &vector,
        |b, v| {
            b.iter(|| find_unsecure_numbers(&v, 100));
        },
    );
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
