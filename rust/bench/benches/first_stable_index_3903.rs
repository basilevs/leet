use criterion::{
    BatchSize, BenchmarkId, Criterion, black_box, criterion_group, criterion_main,
};

const SEED: u64 = 0x1234_5678_9ABC_DEF0;
const SIZES: [usize; 3] = [1_000, 10_000, 100_000];

/// Deterministic SplitMix64 so every variant sees the exact same input.
fn build_nums(len: usize, seed: u64) -> Vec<i32> {
    let mut state = seed;
    (0..len)
        .map(|_| {
            state = state.wrapping_add(0x9E37_79B9_7F4A_7C15);
            let mut z = state;
            z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
            z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
            z ^= z >> 31;
            (z % 1_000_000_000) as i32
        })
        .collect()
}

/// First version: suffix minima written through `mins[i]`, read back
/// through `mins[i]` while `enumerate` tracks the index.
fn indexed(nums: Vec<i32>, k: i32) -> i32 {
    let mut mins = vec![i32::MAX; nums.len()];
    let mut min = i32::MAX;
    for (i, &num) in nums.iter().enumerate().rev() {
        min = min.min(num);
        mins[i] = min;
    }
    let mut max = i32::MIN;
    for (i, &num) in nums.iter().enumerate() {
        max = max.max(num);
        if max - mins[i] <= k {
            return i as i32;
        }
    }
    -1
}

/// Second version: both loops paired with `zip`; `enumerate` survives only
/// to name the answer.
fn zipped(nums: Vec<i32>, k: i32) -> i32 {
    let mut mins = vec![i32::MAX; nums.len()];
    let mut min = i32::MAX;
    for (slot, &num) in mins.iter_mut().zip(&nums).rev() {
        min = min.min(num);
        *slot = min;
    }
    let mut max = i32::MIN;
    for (i, (&num, &min)) in nums.iter().zip(&mins).enumerate() {
        max = max.max(num);
        if max - min <= k {
            return i as i32;
        }
    }
    -1
}

/// `zip` plus `position`, so no index is tracked by hand at all.
fn positioned(nums: Vec<i32>, k: i32) -> i32 {
    let mut mins = vec![i32::MAX; nums.len()];
    let mut min = i32::MAX;
    for (slot, &num) in mins.iter_mut().zip(&nums).rev() {
        min = min.min(num);
        *slot = min;
    }
    let mut max = i32::MIN;
    nums.iter()
        .zip(&mins)
        .position(|(&num, &min)| {
            max = max.max(num);
            max - min <= k
        })
        .map_or(-1, |i| i as i32)
}

fn bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("first_stable_index_3903");

    for len in SIZES {
        let nums = build_nums(len, SEED);
        // k = 0 on pseudo-random values: no stable index exists, so every
        // variant walks the whole array instead of returning early.
        assert_eq!(-1, indexed(nums.clone(), 0));
        assert_eq!(-1, zipped(nums.clone(), 0));
        assert_eq!(-1, positioned(nums.clone(), 0));

        group.bench_with_input(BenchmarkId::new("indexed", len), &nums, |b, nums| {
            b.iter_batched(
                || nums.clone(),
                |nums| indexed(black_box(nums), black_box(0)),
                BatchSize::LargeInput,
            );
        });
        group.bench_with_input(BenchmarkId::new("zipped", len), &nums, |b, nums| {
            b.iter_batched(
                || nums.clone(),
                |nums| zipped(black_box(nums), black_box(0)),
                BatchSize::LargeInput,
            );
        });
        group.bench_with_input(BenchmarkId::new("positioned", len), &nums, |b, nums| {
            b.iter_batched(
                || nums.clone(),
                |nums| positioned(black_box(nums), black_box(0)),
                BatchSize::LargeInput,
            );
        });
    }

    group.finish();
}

criterion_group!(benches, bench);
criterion_main!(benches);
