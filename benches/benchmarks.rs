use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use rand::{rngs::SmallRng, RngCore, SeedableRng};
use std::time::Duration;
use zbase32::encode;

fn encode_benchmark(c: &mut Criterion, gen: &mut SmallRng) {
    const JUST_SIZES: [usize; 8] = [5, 50, 100, 500, 1000, 10_000, 100_000, 1_000_000];
    const RAND_SIZES: [usize; 8] = [7, 51, 103, 503, 1009, 10_009, 100_003, 1_000_003];
    let mut group = c.benchmark_group("encode");
    group.measurement_time(Duration::from_secs(15));

    for (sizes, name) in [(JUST_SIZES, "just-sizes"), (RAND_SIZES, "random-sizes")] {
        for size in sizes {
            let mut input = vec![0; size];
            gen.fill_bytes(&mut input);
            group
                .throughput(Throughput::Bytes(size as u64))
                .bench_with_input(BenchmarkId::new(name, size), &input, |b, input| {
                    b.iter(|| {
                        let e = encode(black_box(&input));
                        black_box(&e);
                    });
                });
        }
    }
    group.finish();
}

fn benchmark(c: &mut Criterion) {
    let mut r = SmallRng::seed_from_u64(47);
    encode_benchmark(c, &mut r);
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
