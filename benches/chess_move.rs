use criterion::{black_box, criterion_group, criterion_main, Criterion};
use oxide::{chess_move::MoveFlag, *};

fn bench_move(c: &mut Criterion) {
    let move_ = chess_move::Move {
        from: definitions::Square::E2,
        to: definitions::Square::E4,
        flag: MoveFlag::PawnDoubleMove,
    };

    c.bench_function("Move::is_promotion", |b| {
        b.iter(|| black_box(move_).is_promotion())
    });
    c.bench_function("Move::is_capture", |b| {
        b.iter(|| black_box(move_).is_capture())
    });
    c.bench_function("Move::is_castle", |b| {
        b.iter(|| black_box(move_).is_castle())
    });
}

criterion_group!(benches, bench_move);
criterion_main!(benches);
