use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn bench_decode(c: &mut Criterion) {
    // BCJ2 decode benchmark placeholder.
    // A real benchmark would need pre-split 4-stream data from a 7z BCJ2 archive.
    let mut group = c.benchmark_group("decode");
    group.bench_function("bcj2_placeholder", |b| {
        b.iter(|| {
            black_box(0u8);
        });
    });
    group.finish();
}

criterion_group!(benches, bench_decode);
criterion_main!(benches);
