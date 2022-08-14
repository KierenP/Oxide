use crate::board::Board;

use super::chess_move::{Move, MoveFlag};
use crate::definitions::*;

pub trait MoveContainer {
    fn push(&mut self, value: Move);
}

impl MoveContainer for Vec<Move> {
    fn push(&mut self, value: Move) {
        self.push(value);
    }
}

impl MoveContainer for arrayvec::ArrayVec<Move, 256> {
    fn push(&mut self, value: Move) {
        self.push(value);
    }
}

pub fn legal_moves<T: MoveContainer>(board: &Board, moves: &mut T) {
    let pinned = pinned_mask(board);
    quiet_moves(board, moves, &pinned);
    loud_moves(board, moves, &pinned);
}

pub fn quiet_moves<T: MoveContainer>(board: &Board, moves: &mut T, pinned: &BB) {
    pawn_pushes(board, moves, pinned);
    pawn_double_pushes(board, moves, pinned);
    castle_moves(board, moves);

    let pieces = board.occupied_squares();

    let mut generate_moves = |mut targets: BB, from| {
        while targets != BB_EMPTY {
            let to = targets.poplsb();
            let m = Move {
                from,
                to,
                flag: MoveFlag::Quiet,
            };

            if in_between(from, to) & pieces == BB_EMPTY
                && (*pinned & SQUARE_BB[from as usize] == BB_EMPTY
                    || !move_puts_self_in_check(board, &m))
            {
                moves.push(m);
            }
        }
    };

    let mut knights = board.get_piece_bb(Piece::from_type(PieceType::Knight, board.stm));
    while knights != BB_EMPTY {
        let from = knights.poplsb();
        generate_moves(KNIGHT_ATTACKS[from as usize] & !pieces, from);
    }

    let mut kings = board.get_piece_bb(Piece::from_type(PieceType::King, board.stm));
    while kings != BB_EMPTY {
        let from = kings.poplsb();
        generate_moves(KING_ATTACKS[from as usize] & !pieces, from);
    }

    let mut bishops = board.get_piece_bb(Piece::from_type(PieceType::Bishop, board.stm));
    while bishops != BB_EMPTY {
        let from = bishops.poplsb();
        generate_moves(BISHOP_ATTACKS[from as usize] & !pieces, from);
    }

    let mut rooks = board.get_piece_bb(Piece::from_type(PieceType::Rook, board.stm));
    while rooks != BB_EMPTY {
        let from = rooks.poplsb();
        generate_moves(ROOK_ATTACKS[from as usize] & !pieces, from);
    }

    let mut queens = board.get_piece_bb(Piece::from_type(PieceType::Queen, board.stm));
    while queens != BB_EMPTY {
        let from = queens.poplsb();
        generate_moves(QUEEN_ATTACKS[from as usize] & !pieces, from);
    }
}

pub fn loud_moves<T: MoveContainer>(board: &Board, moves: &mut T, pinned: &BB) {
    pawn_captures(board, moves, pinned);
    pawn_promotions(board, moves, pinned);
    pawn_en_passant(board, moves);

    let pieces = board.occupied_squares();
    let targets = board.get_pieces(!board.stm);

    let mut generate_captures = |mut targets: BB, from| {
        while targets != BB_EMPTY {
            let to = targets.poplsb();
            let m = Move {
                from,
                to,
                flag: MoveFlag::Capture,
            };

            if in_between(from, to) & pieces == BB_EMPTY
                && (*pinned & SQUARE_BB[from as usize] == BB_EMPTY
                    || !move_puts_self_in_check(board, &m))
            {
                moves.push(m);
            }
        }
    };

    let mut knights = board.get_piece_bb(Piece::from_type(PieceType::Knight, board.stm));
    while knights != BB_EMPTY {
        let from = knights.poplsb();
        generate_captures(KNIGHT_ATTACKS[from as usize] & targets, from);
    }

    let mut kings = board.get_piece_bb(Piece::from_type(PieceType::King, board.stm));
    while kings != BB_EMPTY {
        let from = kings.poplsb();
        generate_captures(KING_ATTACKS[from as usize] & targets, from);
    }

    let mut bishops = board.get_piece_bb(Piece::from_type(PieceType::Bishop, board.stm));
    while bishops != BB_EMPTY {
        let from = bishops.poplsb();
        generate_captures(BISHOP_ATTACKS[from as usize] & targets, from);
    }

    let mut rooks = board.get_piece_bb(Piece::from_type(PieceType::Rook, board.stm));
    while rooks != BB_EMPTY {
        let from = rooks.poplsb();
        generate_captures(ROOK_ATTACKS[from as usize] & targets, from);
    }

    let mut queens = board.get_piece_bb(Piece::from_type(PieceType::Queen, board.stm));
    while queens != BB_EMPTY {
        let from = queens.poplsb();
        generate_captures(QUEEN_ATTACKS[from as usize] & targets, from);
    }
}

pub fn is_in_check(board: &Board, s: Side) -> bool {
    is_square_threatened(board, board.get_king(s), s)
}

fn pawn_pushes<T: MoveContainer>(board: &Board, moves: &mut T, pinned: &BB) {
    let forward;
    let mut targets;

    if board.stm == Side::White {
        forward = 8;
        targets = (board.get_piece_bb(Piece::WhitePawn) << 8) & board.empty_squares();
    } else {
        forward = -8;
        targets = (board.get_piece_bb(Piece::BlackPawn) >> 8) & board.empty_squares();
    };

    // ignore pushes to back ranks
    targets &= !(RANK_BB[Rank::One as usize] | RANK_BB[Rank::Eight as usize]);

    while targets != BB_EMPTY {
        let end = targets.poplsb();
        let start = Square::from_index((end as i8 - forward) as usize);

        let m = Move {
            from: start,
            to: end,
            flag: MoveFlag::Quiet,
        };

        if *pinned & SQUARE_BB[start as usize] == BB_EMPTY || !move_puts_self_in_check(board, &m) {
            moves.push(m);
        }
    }
}

fn pawn_promotions<T: MoveContainer>(board: &Board, moves: &mut T, pinned: &BB) {
    let forward;
    let mut targets;

    if board.stm == Side::White {
        forward = 8;
        targets = (board.get_piece_bb(Piece::WhitePawn) << 8) & board.empty_squares();
    } else {
        forward = -8;
        targets = (board.get_piece_bb(Piece::BlackPawn) >> 8) & board.empty_squares();
    };

    // only pushes to back ranks
    targets &= RANK_BB[Rank::One as usize] | RANK_BB[Rank::Eight as usize];

    while targets != BB_EMPTY {
        let end = targets.poplsb();
        let start = Square::from_index((end as i8 - forward) as usize);

        let m = Move {
            from: start,
            to: end,
            flag: MoveFlag::KnightPromotion,
        };

        if *pinned & SQUARE_BB[start as usize] == BB_EMPTY || !move_puts_self_in_check(board, &m) {
            moves.push(m);
            moves.push(Move {
                from: start,
                to: end,
                flag: MoveFlag::BishopPromotion,
            });

            moves.push(Move {
                from: start,
                to: end,
                flag: MoveFlag::RookPromotion,
            });

            moves.push(Move {
                from: start,
                to: end,
                flag: MoveFlag::QueenPromotion,
            });
        }
    }
}

fn pawn_double_pushes<T: MoveContainer>(board: &Board, moves: &mut T, pinned: &BB) {
    let forward;
    let mut targets;

    if board.stm == Side::White {
        forward = 16;
        targets = board.get_piece_bb(Piece::WhitePawn) & RANK_BB[Rank::Two as usize];
        targets = (targets << 8) & board.empty_squares();
        targets = (targets << 8) & board.empty_squares();
    } else {
        forward = -16;
        targets = board.get_piece_bb(Piece::BlackPawn) & RANK_BB[Rank::Seven as usize];
        targets = (targets >> 8) & board.empty_squares();
        targets = (targets >> 8) & board.empty_squares();
    };

    while targets != BB_EMPTY {
        let end = targets.poplsb();
        let start = Square::from_index((end as i8 - forward) as usize);

        let m = Move {
            from: start,
            to: end,
            flag: MoveFlag::PawnDoubleMove,
        };

        if *pinned & SQUARE_BB[start as usize] == BB_EMPTY || !move_puts_self_in_check(board, &m) {
            moves.push(m);
        }
    }
}

fn pawn_en_passant<T: MoveContainer>(board: &Board, moves: &mut T) {
    if board.en_passant.is_none() {
        return;
    }

    let mut attackers = PAWN_ATTACKS[!board.stm as usize][board.en_passant.unwrap() as usize]
        & board.get_piece_bb(Piece::from_type(PieceType::Pawn, board.stm));

    while attackers != BB_EMPTY {
        let start = attackers.poplsb();
        let end = board.en_passant.unwrap();

        let m = Move {
            from: start,
            to: end,
            flag: MoveFlag::EnPassant,
        };

        if !move_puts_self_in_check(board, &m) {
            moves.push(m);
        }
    }
}

fn pawn_captures<T: MoveContainer>(board: &Board, moves: &mut T, pinned: &BB) {
    let forward_left;
    let forward_right;
    let mut left_attackers;
    let mut right_attackers;
    let pawns = board.get_piece_bb(Piece::from_type(PieceType::Pawn, board.stm));

    if board.stm == Side::White {
        forward_left = 7;
        forward_right = 9;
        left_attackers =
            ((pawns & !(FILE_BB[File::A as usize])) << 7) & board.get_pieces(Side::Black);
        right_attackers =
            ((pawns & !(FILE_BB[File::H as usize])) << 9) & board.get_pieces(Side::Black);
    } else {
        forward_left = -9;
        forward_right = -7;
        left_attackers =
            ((pawns & !(FILE_BB[File::A as usize])) >> 9) & board.get_pieces(Side::White);
        right_attackers =
            ((pawns & !(FILE_BB[File::H as usize])) >> 7) & board.get_pieces(Side::White);
    };

    while left_attackers != BB_EMPTY {
        let end = left_attackers.poplsb();
        let start = Square::from_index((end as i8 - forward_left) as usize);

        let m = Move {
            from: start,
            to: end,
            flag: MoveFlag::Capture,
        };

        if *pinned & SQUARE_BB[start as usize] != BB_EMPTY && move_puts_self_in_check(board, &m) {
            continue;
        }

        if end.rank() == Rank::Eight || end.rank() == Rank::One {
            moves.push(Move {
                from: start,
                to: end,
                flag: MoveFlag::KnightPromotion,
            });

            moves.push(Move {
                from: start,
                to: end,
                flag: MoveFlag::BishopPromotion,
            });

            moves.push(Move {
                from: start,
                to: end,
                flag: MoveFlag::RookPromotion,
            });

            moves.push(Move {
                from: start,
                to: end,
                flag: MoveFlag::QueenPromotion,
            });
        } else {
            moves.push(m);
        }
    }

    while right_attackers != BB_EMPTY {
        let end = right_attackers.poplsb();
        let start = Square::from_index((end as i8 - forward_right) as usize);

        let m = Move {
            from: start,
            to: end,
            flag: MoveFlag::Capture,
        };

        if *pinned & SQUARE_BB[start as usize] != BB_EMPTY && move_puts_self_in_check(board, &m) {
            continue;
        }

        if end.rank() == Rank::Eight || end.rank() == Rank::One {
            moves.push(Move {
                from: start,
                to: end,
                flag: MoveFlag::KnightPromotion,
            });

            moves.push(Move {
                from: start,
                to: end,
                flag: MoveFlag::BishopPromotion,
            });

            moves.push(Move {
                from: start,
                to: end,
                flag: MoveFlag::RookPromotion,
            });

            moves.push(Move {
                from: start,
                to: end,
                flag: MoveFlag::QueenPromotion,
            });
        } else {
            moves.push(m);
        }
    }
}

fn castle_moves<T: MoveContainer>(board: &Board, moves: &mut T) {
    let pieces = board.occupied_squares();

    // path1..3 are the squares the king will traverse to castle
    let mut can_castle = |move_flag, rook, path_1, path_2, path_3| {
        if in_between(path_1, rook) & pieces == BB_EMPTY
            && !is_square_threatened(board, path_1, board.stm)
            && !is_square_threatened(board, path_2, board.stm)
            && !is_square_threatened(board, path_3, board.stm)
        {
            moves.push(Move {
                from: path_1,
                to: path_3,
                flag: move_flag,
            });
        }
    };

    if board.stm == Side::White && board.white_king_castle {
        can_castle(
            MoveFlag::KingCastle,
            Square::H1,
            Square::E1,
            Square::F1,
            Square::G1,
        );
    }
    if board.stm == Side::White && board.white_queen_castle {
        can_castle(
            MoveFlag::QueenCastle,
            Square::A1,
            Square::E1,
            Square::D1,
            Square::C1,
        );
    }
    if board.stm == Side::Black && board.black_king_castle {
        can_castle(
            MoveFlag::KingCastle,
            Square::H8,
            Square::E8,
            Square::F8,
            Square::G8,
        );
    }
    if board.stm == Side::Black && board.black_queen_castle {
        can_castle(
            MoveFlag::QueenCastle,
            Square::A8,
            Square::E8,
            Square::D8,
            Square::C8,
        );
    }
}

fn move_puts_self_in_check(board: &Board, m: &Move) -> bool {
    let mut board = board.clone();
    board.make_move(m);
    is_in_check(&board, !board.stm) // the STM has changed after we made the move
}

fn is_square_threatened(board: &Board, square: Square, side: Side) -> bool {
    if KNIGHT_ATTACKS[square as usize]
        & board.get_piece_bb(Piece::from_type(PieceType::Knight, !side))
        != BB_EMPTY
    {
        return true;
    }

    if PAWN_ATTACKS[side as usize][square as usize]
        & board.get_piece_bb(Piece::from_type(PieceType::Pawn, !side))
        != BB_EMPTY
    {
        return true;
    }

    if KING_ATTACKS[square as usize] & board.get_piece_bb(Piece::from_type(PieceType::King, !side))
        != BB_EMPTY
    {
        return true;
    }

    let pieces = board.occupied_squares();

    let mut queens = QUEEN_ATTACKS[square as usize]
        & board.get_piece_bb(Piece::from_type(PieceType::Queen, !side));
    while queens != BB_EMPTY {
        let start = queens.poplsb();
        if in_between(start, square) & pieces == BB_EMPTY {
            return true;
        }
    }

    let mut bishops = BISHOP_ATTACKS[square as usize]
        & board.get_piece_bb(Piece::from_type(PieceType::Bishop, !side));
    while bishops != BB_EMPTY {
        let start = bishops.poplsb();
        if in_between(start, square) & pieces == BB_EMPTY {
            return true;
        }
    }

    let mut rooks = ROOK_ATTACKS[square as usize]
        & board.get_piece_bb(Piece::from_type(PieceType::Rook, !side));
    while rooks != BB_EMPTY {
        let start = rooks.poplsb();
        if in_between(start, square) & pieces == BB_EMPTY {
            return true;
        }
    }

    return false;
}

/// Return a mask of all potentially pinned pieces. This is an optimization, any
/// pieces that move from a non-pinned square will not be checked for putting the
/// king in check. If the king itself is in check, then we must check all moves
/// to see if they put the king in check so we return BB_FULL.
fn pinned_mask(board: &Board) -> BB {
    let king = board.get_king(board.stm);

    if is_in_check(board, board.stm) {
        return BB_FULL;
    }

    let pieces = board.occupied_squares();
    let our_pieces = board.get_pieces(board.stm);
    let mut pinned = BB_EMPTY;

    let enemy_queen = board.get_piece_bb(Piece::from_type(PieceType::Queen, !board.stm));
    let enemy_bishop = board.get_piece_bb(Piece::from_type(PieceType::Bishop, !board.stm));
    let enemy_rook = board.get_piece_bb(Piece::from_type(PieceType::Rook, !board.stm));
    let queen_rook = enemy_queen | enemy_rook;
    let queen_bishop = enemy_queen | enemy_bishop;

    let scan_for_pinned = |mut possible_pins: BB, all_pieces: BB| -> BB {
        let mut pinned = BB_EMPTY;
        while possible_pins != BB_EMPTY {
            let start = possible_pins.poplsb();
            if in_between(king, start) & all_pieces == BB_EMPTY {
                pinned |= SQUARE_BB[start as usize];
            }
        }
        pinned
    };

    // firstly check if there is an enemy bishop or queen in the same diagonal as our king,
    // because if not, then theres no pins. If there is, then we need to go through each of
    // our pieces on the diagonal and see if there's anything between the king and the piece.
    if DIAGONAL_BB[king.diagonal() as usize] & queen_bishop != BB_EMPTY {
        pinned |= scan_for_pinned(DIAGONAL_BB[king.diagonal() as usize] & our_pieces, pieces);
    }

    if ANTI_DIAGONAL_BB[king.antidiagonal() as usize] & queen_bishop != BB_EMPTY {
        pinned |= scan_for_pinned(
            ANTI_DIAGONAL_BB[king.antidiagonal() as usize] & our_pieces,
            pieces,
        );
    }
    if RANK_BB[king.rank() as usize] & queen_rook != BB_EMPTY {
        pinned |= scan_for_pinned(RANK_BB[king.rank() as usize] & our_pieces, pieces);
    }

    if FILE_BB[king.file() as usize] & queen_rook != BB_EMPTY {
        pinned |= scan_for_pinned(FILE_BB[king.file() as usize] & our_pieces, pieces);
    }

    // consider the king itself pinned, so we always check for legality for king moves
    pinned |= SQUARE_BB[king as usize];
    pinned
}
