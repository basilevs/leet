use criterion::{
    BatchSize, BenchmarkId, Criterion, black_box, criterion_group, criterion_main,
};
use leet::max_total_value_3689::max_total_value;

const K: i32 = 5;
const SEED: u64 = 0x1234_5678_9ABC_DEF0;
const SIZES: [usize; 4] = [1_000, 10_000, 100_000, 1_000_000];

/// Deterministic SplitMix64 generator. The caller varies `seed` per iteration
/// so that data is fresh every timed call (defeating branch-predictor
/// memorization), while a fixed seed sequence keeps the whole run reproducible.
fn random_nums(len: usize, seed: u64) -> Vec<i32> {
    let mut state = seed;
    (0..len)
        .map(|_| {
            state = state.wrapping_add(0x9E37_79B9_7F4A_7C15);
            let mut z = state;
            z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
            z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
            z ^= z >> 31;
            // Map into [-1_000_000, 1_000_000] to match the problem constraints.
            (z % 2_000_001) as i32 - 1_000_000
        })
        .collect()
}

fn bench_fresh(c: &mut Criterion) {
    let mut group = c.benchmark_group("max_total_value_fresh");

    for len in SIZES {
        group.bench_with_input(BenchmarkId::from_parameter(len), &len, |b, &len| {
            // Fresh data for every timed call: each input gets a new seed, so the
            // branch predictor can never memorize the sequence. This exposes the
            // true random-data cost (including mispredictions) at every size,
            // instead of letting the predictor learn one reused array at small N.
            // LargeInput keeps timer overhead low while bounding memory; the
            // generated batch holds many *different* arrays, not clones.
            let mut seed = SEED;
            b.iter_batched(
                || {
                    seed = seed.wrapping_add(0x9E37_79B9_7F4A_7C15);
                    random_nums(len, seed)
                },
                |nums| max_total_value(black_box(nums), black_box(K)),
                BatchSize::LargeInput,
            );
        });
    }

    group.finish();
}

criterion_group!(benches, bench_fresh);
criterion_main!(benches);
