use criterion::{
    BatchSize, BenchmarkId, Criterion, black_box, criterion_group, criterion_main,
};
use leet::max_total_value_3691::max_total_value;

const SEED: u64 = 0x1234_5678_9ABC_DEF0;

// (n, k) pairs chosen to stress heap pre-sizing in both directions. The heap's
// peak size is always `n` (seed `n` entries, then each pop pushes at most one),
// so `with_capacity(n)` is exact while `with_capacity(k)` is wrong whenever
// `k != n`:
//   (50_000,       5) — k << n: with_capacity(k) reallocates while seeding.
//   (50_000,  50_000) — k == n: control, both sizings are exact.
//   (   500, 100_000) — k >> n: with_capacity(k) over-allocates ~200x the peak.
const CASES: [(usize, i32); 3] = [(50_000, 5), (50_000, 50_000), (500, 100_000)];

/// Deterministic SplitMix64 generator producing values in the problem's
/// `0 <= nums[i] <= 1e9` range.
fn random_nums(len: usize, seed: u64) -> Vec<i32> {
    let mut state = seed;
    (0..len)
        .map(|_| {
            state = state.wrapping_add(0x9E37_79B9_7F4A_7C15);
            let mut z = state;
            z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
            z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
            z ^= z >> 31;
            (z % 1_000_000_001) as i32
        })
        .collect()
}

fn bench_heap_sizing(c: &mut Criterion) {
    let mut group = c.benchmark_group("max_total_value_3691");

    for (n, k) in CASES {
        let id = BenchmarkId::from_parameter(format!("n{n}_k{k}"));
        group.bench_with_input(id, &(n, k), |b, &(n, k)| {
            let mut seed = SEED;
            b.iter_batched(
                || {
                    seed = seed.wrapping_add(0x9E37_79B9_7F4A_7C15);
                    random_nums(n, seed)
                },
                |nums| max_total_value(black_box(nums), black_box(k)),
                BatchSize::LargeInput,
            );
        });
    }

    group.finish();
}

criterion_group!(benches, bench_heap_sizing);
criterion_main!(benches);
