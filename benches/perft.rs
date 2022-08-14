use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use oxide::*;

fn bench_perft(c: &mut Criterion) {
    let fen = "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1";
    let board = fen::fen_to_board(fen).unwrap();
    let depth = 4;

    c.bench_with_input(BenchmarkId::new("perft", depth), &depth, |b, &depth| {
        b.iter(|| perft::perft(&board, depth))
    });
}

criterion_group! {
    name = benches;
    config = Criterion::default().sample_size(10);
    targets = bench_perft
}
criterion_main!(benches);
