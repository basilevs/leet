use criterion::{Criterion, criterion_group, criterion_main, BatchSize};
use leet::robot_sim_874::{hashed, naive, naive_factored, winner};
const COMMANDS:[i32; 100] = [5, -1, 6, 8, 3, 2, 8, 8, 5, 4, 6, 2, -2, 4, 7, 7, 3, -1, 5, 9, -1, 8, 1, 7, 5, 6, 8, 5, 4, 4, 2, -2, 4, 6, 3, 2, 4, 4, 7, 9, -1, 9, -2, 7, 5, 2, 8, 5, 3, 5, 1, -1, 9, -1, 8, -1, 9, 1, -2, -1, 8, -1, 5, 8, 6, 7, 6, 3, -2, -2, 6, 7, -2, 5, 6, -2, -1, 7, 6, -1, 2, -1, 3, 4, 9, 5, 7, 7, 8, -1, 8, 2, -2, 3, 2, 1, -2, 4, 7, 3];


const OBSTACLES:[[i32; 2]; 34] = [[30, -38], [-64, -38], [-48, 11], [-70, 9], [-21, -96], [89, -25], [-40, 96], [-29, -37], [49, -78], [-10, -94], [81, -53], [-77, 6], [-17, 97], [-42, 73], [-49, 75], [-72, 98], [-88, -6], [-82, -97], [53, -71], [-18, -62], [-22, 31], [-23, -58], [77, -96], [-4, 61], [9, -13], [24, -40], [35, -69], [-98, 42], [67, -61], [-91, -87], [-10, -28], [63, -84], [-55, -52], [-21, -49]];

fn bench_path(c: &mut Criterion) {
    let commands = COMMANDS.to_vec();
    let obstacles:Vec<Vec<i32>> = OBSTACLES.iter().map(|c| c.to_vec()).collect();

    c.bench_function("naive", |b| {
        b.iter_batched(|| (commands.clone(), obstacles.clone()), |d| naive(d.0, d.1), BatchSize::SmallInput);
    } );

    c.bench_function("naive_factored", |b| {
        b.iter_batched(|| (commands.clone(), obstacles.clone()), |d| naive_factored(d.0, d.1), BatchSize::SmallInput);
    } );

    c.bench_function("hashed", |b| {
        b.iter_batched(|| (commands.clone(), obstacles.clone()), |d| hashed(d.0, d.1), BatchSize::SmallInput);
    } );

    c.bench_function("winner", |b| {
        b.iter_batched(|| (commands.clone(), obstacles.clone()), |d| winner(d.0, d.1), BatchSize::SmallInput);
    } );
}

criterion_group!(benches, bench_path);
criterion_main!(benches);
