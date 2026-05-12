use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn bench_encode(c: &mut Criterion) {
    // BCJ2 encode is not implemented in Phase 1 (decoder-only).
    // Placeholder for Phase 2 when native BCJ2 encoder lands.
    let mut group = c.benchmark_group("encode");
    group.bench_function("bcj2_placeholder", |b| {
        b.iter(|| {
            black_box(0u8);
        });
    });
    group.finish();
}

criterion_group!(benches, bench_encode);
criterion_main!(benches);
