use criterion::{
    BatchSize, BenchmarkId, Criterion, black_box, criterion_group, criterion_main,
};
use leet::find_missing_elements_3731::{
    find_missing_elements, find_missing_elements_bitset, find_missing_elements_bool,
    find_missing_elements_hashset, find_missing_elements_loop, find_missing_elements_u128,
};

const SEED: u64 = 0x1234_5678_9ABC_DEF0;
/// Full-range spans to exercise. The range-scan variants (bitset / bool array)
/// pay `O(span)`, while the sort-based baseline pays `O(n log n)` on the present
/// values, so widening the span stresses the scanners specifically.
const SPANS: [usize; 5] = [100, 1_000, 10_000, 100_000, 1_000_000];

/// Deterministic SplitMix64 step.
fn split_mix(state: &mut u64) -> u64 {
    *state = state.wrapping_add(0x9E37_79B9_7F4A_7C15);
    let mut z = *state;
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    z ^ (z >> 31)
}

/// Build a present-values vector over the range `[1, span]`. The endpoints `1`
/// and `span` are always present (as the problem guarantees), and each interior
/// value is kept with ~50% probability, so roughly half the range is missing.
/// The returned order is shuffled-ish (kept in ascending order here; the
/// baseline sorts anyway, and the scanners are order-insensitive).
fn present_values(span: usize, seed: u64) -> Vec<i32> {
    let mut state = seed;
    let mut nums = Vec::with_capacity(span);
    nums.push(1);
    for v in 2..span as i32 {
        if split_mix(&mut state) & 1 == 0 {
            nums.push(v);
        }
    }
    if span >= 2 {
        nums.push(span as i32);
    }
    nums
}

fn bench_variant(
    c: &mut Criterion,
    name: &str,
    spans: &[usize],
    f: fn(Vec<i32>) -> Vec<i32>,
) {
    let mut group = c.benchmark_group(name);
    for &span in spans {
        group.bench_with_input(BenchmarkId::from_parameter(span), &span, |b, &span| {
            let mut seed = SEED;
            b.iter_batched(
                || {
                    seed = seed.wrapping_add(0x9E37_79B9_7F4A_7C15);
                    present_values(span, seed)
                },
                |nums| f(black_box(nums)),
                BatchSize::LargeInput,
            );
        });
    }
    group.finish();
}

fn bench_all(c: &mut Criterion) {
    bench_variant(c, "find_missing_sort", &SPANS, find_missing_elements);
    bench_variant(c, "find_missing_loop", &SPANS, find_missing_elements_loop);
    bench_variant(c, "find_missing_bitset", &SPANS, find_missing_elements_bitset);
    bench_variant(c, "find_missing_bool", &SPANS, find_missing_elements_bool);
    bench_variant(c, "find_missing_hashset", &SPANS, find_missing_elements_hashset);
    // The single-u128 mask only holds values < 128, so it is valid only at the
    // problem's real ceiling (span = 100); larger spans would overflow the shift.
    bench_variant(c, "find_missing_u128", &[100], find_missing_elements_u128);
}

criterion_group!(benches, bench_all);
criterion_main!(benches);
