use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId};
use oxide::{*, chess_move::MoveFlag};

fn bench_perft(c: &mut Criterion) {
    let board =
        fen::fen_to_board("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1")
            .unwrap();

    c.bench_function("Perft", |b| b.iter(|| perft::perft(&board, 3)));
}

fn bench_move(c: &mut Criterion) {
    let move_ = chess_move::Move { from: definitions::Square::E2, to: definitions::Square::E4, flag: MoveFlag::PawnDoubleMove };

    c.bench_with_input(BenchmarkId::new("Move::is_promotion", move_), &move_, |b, move_| b.iter(|| move_.is_promotion()));
    c.bench_with_input(BenchmarkId::new("Move::is_capture", move_), &move_, |b, move_| b.iter(|| move_.is_capture()));
    c.bench_with_input(BenchmarkId::new("Move::is_castle", move_), &move_, |b, move_| b.iter(|| move_.is_castle()));

}

criterion_group!(benches, bench_move, bench_perft);
criterion_main!(benches);
