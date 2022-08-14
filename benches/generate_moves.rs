use std::vec;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use oxide::*;

fn bench_legal_moves(c: &mut Criterion) {
    let fen = "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1";
    let board = fen::fen_to_board(fen).unwrap();

    c.bench_function("generate_moves::legal_moves", |b| {
        b.iter(|| {
            let mut moves = vec![];
            generate_moves::legal_moves(&board, black_box(&mut moves))
        })
    });
}

criterion_group!(benches, bench_legal_moves);
criterion_main!(benches);
