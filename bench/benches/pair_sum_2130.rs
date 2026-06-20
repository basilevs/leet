use std::cell::RefCell;
use std::iter::from_fn;

use criterion::{
    BatchSize, BenchmarkId, Criterion, black_box, criterion_group, criterion_main,
};
use leet::pair_sum_2130::ListNode;

const SEED: u64 = 0x1234_5678_9ABC_DEF0;
// All even, matching the problem guarantee of an even node count.
const SIZES: [usize; 3] = [1_000, 10_000, 100_000];

/// Deterministic SplitMix64 so every variant and every baseline traverses the
/// exact same list, keeping the A/B comparison meaningful.
fn build_list(len: usize, seed: u64) -> Option<Box<ListNode>> {
    let mut state = seed;
    let mut next_val = || {
        state = state.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = state;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^= z >> 31;
        // Map into [1, 100_000] to match the problem constraints.
        (z % 100_000) as i32 + 1
    };

    let mut head = None;
    for _ in 0..len {
        head = Some(Box::new(ListNode {
            val: next_val(),
            next: head,
        }));
    }
    head
}

thread_local! {
    static COPY_WHILE: RefCell<Vec<i32>> = RefCell::new(Vec::with_capacity(100_000));
    static COPY_FROM_FN: RefCell<Vec<i32>> = RefCell::new(Vec::with_capacity(100_000));
}

/// Collect via an explicit `while let` move loop, then fold the twin sums.
/// Mirrors the committed `pair_sum` exactly (thread_local scratch + RefCell).
fn pair_sum_while(mut head: Option<Box<ListNode>>) -> i32 {
    COPY_WHILE.with_borrow_mut(|copy| {
        copy.clear();
        while let Some(node) = head {
            copy.push(node.val);
            head = node.next;
        }

        let n = copy.len();
        copy.iter()
            .take(n / 2)
            .enumerate()
            .map(|(i, val)| val + copy[n - 1 - i])
            .max()
            .unwrap()
    })
}

/// Collect via `Vec::extend` over a `from_fn` iterator, then fold the twin sums.
/// Mirrors the commented-out variant exactly (thread_local scratch + RefCell).
fn pair_sum_from_fn(mut head: Option<Box<ListNode>>) -> i32 {
    COPY_FROM_FN.with_borrow_mut(|copy| {
        copy.clear();
        copy.extend(from_fn(|| {
            let result = head.as_ref().map(|t| t.val);
            head = head.take().and_then(|t| t.next);
            result
        }));

        let n = copy.len();
        copy.iter()
            .take(n / 2)
            .enumerate()
            .map(|(i, val)| val + copy[n - 1 - i])
            .max()
            .unwrap()
    })
}

fn bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("pair_sum_2130");

    for len in SIZES {
        group.bench_with_input(BenchmarkId::new("while", len), &len, |b, &len| {
            b.iter_batched(
                || build_list(len, SEED),
                |list| pair_sum_while(black_box(list)),
                BatchSize::LargeInput,
            );
        });

        group.bench_with_input(BenchmarkId::new("from_fn", len), &len, |b, &len| {
            b.iter_batched(
                || build_list(len, SEED),
                |list| pair_sum_from_fn(black_box(list)),
                BatchSize::LargeInput,
            );
        });
    }

    group.finish();
}

criterion_group!(benches, bench);
criterion_main!(benches);
