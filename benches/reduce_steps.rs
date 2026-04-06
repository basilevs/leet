use criterion::{BenchmarkId, Criterion, black_box, criterion_group, criterion_main};
use leet::reduce_steps::{
    number_of_steps, number_of_steps_declarative, number_of_steps_from_leet,
    number_of_steps_imperative, number_of_steps_naive,
};

const INPUTS: &[i32] = &[0, 1, 14, 1234, 65_535, 1_000_000, i32::MAX];

fn bench_single_value(c: &mut Criterion) {
    let mut group = c.benchmark_group("single_value");

    for input in INPUTS {
        group.bench_with_input(BenchmarkId::new("optimized", input), input, |b, &n| {
            b.iter(|| number_of_steps(black_box(n)));
        });

        group.bench_with_input(BenchmarkId::new("naive", input), input, |b, &n| {
            b.iter(|| number_of_steps_naive(black_box(n)));
        });
    }

    group.finish();
}

fn bench_batch(c: &mut Criterion) {
    let mut group = c.benchmark_group("batch");
    group.bench_function("optimized_table", |b| {
        b.iter(|| {
            let mut total = 0;
            for &n in INPUTS {
                total += number_of_steps(black_box(n));
            }
            black_box(total)
        });
    });

    group.bench_function("number_of_steps_declarative", |b| {
        b.iter(|| {
            let mut total = 0;
            for &n in INPUTS {
                total += number_of_steps_declarative(black_box(n));
            }
            black_box(total)
        });
    });

    group.bench_function("number_of_steps_from_leet", |b| {
        b.iter(|| {
            let mut total = 0;
            for &n in INPUTS {
                total += number_of_steps_from_leet(black_box(n));
            }
            black_box(total)
        });
    });

    group.bench_function("number_of_steps_imperative", |b| {
        b.iter(|| {
            let mut total = 0;
            for &n in INPUTS {
                total += number_of_steps_imperative(black_box(n));
            }
            black_box(total)
        });
    });

    group.bench_function("naive", |b| {
        b.iter(|| {
            let mut total = 0;
            for &n in INPUTS {
                total += number_of_steps_naive(black_box(n));
            }
            black_box(total)
        });
    });
    group.finish();
}

criterion_group!(benches, bench_single_value, bench_batch);
criterion_main!(benches);
