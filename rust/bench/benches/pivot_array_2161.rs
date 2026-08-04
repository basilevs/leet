use criterion::{
    BatchSize, BenchmarkId, Criterion, black_box, criterion_group, criterion_main,
};
use leet::pivot_array_2161::pivot_array;

const PIVOT: i32 = 0;
const SEED: u64 = 0x1234_5678_9ABC_DEF0;
const SIZES: [usize; 3] = [1_000, 10_000, 100_000];

/// Deterministic SplitMix64 generator so every run (and every branch baseline)
/// partitions the exact same data, keeping comparisons meaningful.
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

fn bench_random(c: &mut Criterion) {
    let mut group = c.benchmark_group("pivot_array_random");

    for len in SIZES {
        let nums = random_nums(len, SEED);
        group.bench_with_input(BenchmarkId::from_parameter(len), &nums, |b, nums| {
            b.iter_batched(
                || nums.clone(),
                |nums| pivot_array(black_box(nums), black_box(PIVOT)),
                BatchSize::SmallInput,
            );
        });
    }

    group.finish();
}

criterion_group!(benches, bench_random);
criterion_main!(benches);
