use criterion::{black_box, criterion_group, criterion_main, Criterion};
use circadian_tools;

fn circadian_benchmark(c: &mut Criterion) {
    c.bench_function("circadian", |b| {
        b.iter(|| {
            let one_thousands: Vec<f32> = vec![black_box(1000.0); 1_000_000];
            let two_thousands: Vec<f32> = vec![black_box(2000.0); 1_000_000];
            let data = one_thousands.into_iter().chain(two_thousands.into_iter());
            circadian_tools::circadian_average(4000.0, data.into_iter())
        })
    });
    c.bench_function("safe_circadian", |b| {
        b.iter(|| {
            let one_thousands: Vec<f32> = vec![black_box(1000.0); 1_000_000];
            let two_thousands: Vec<f32> = vec![black_box(2000.0); 1_000_000];
            let data = one_thousands.into_iter().chain(two_thousands.into_iter());
            circadian_tools::safe_circadian_average(4000.0, data.into_iter())
        })
    });
}

criterion_group!(benches, circadian_benchmark);
criterion_main!(benches);
