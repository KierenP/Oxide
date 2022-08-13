use crate::board::Board;

use super::chess_move::{Move, MoveFlag};
use crate::definitions::*;

pub fn legal_moves(board: &Board, moves: &mut Vec<Move>) {
    quiet_moves(board, moves);
    loud_moves(board, moves);
}

pub fn quiet_moves(board: &Board, moves: &mut Vec<Move>) {
    pawn_pushes(board, moves);
    pawn_double_pushes(board, moves);
    castle_moves(board, moves);

    let pieces = board.occupied_squares();

    let mut knights = board.get_piece_bb(Piece::from_type(PieceType::Knight, board.stm));
    while knights != BB_EMPTY {
        let from = knights.poplsb();
        let mut targets = KNIGHT_ATTACKS[from as usize] & !pieces;
        while targets != BB_EMPTY {
            let to = targets.poplsb();
            let m = Move {
                from: from,
                to: to,
                flag: MoveFlag::Quiet,
            };

            if !move_puts_self_in_check(board, &m) {
                moves.push(m);
            }
        }
    }

    let mut kings = board.get_piece_bb(Piece::from_type(PieceType::King, board.stm));
    while kings != BB_EMPTY {
        let from = kings.poplsb();
        let mut targets = KING_ATTACKS[from as usize] & !pieces;
        while targets != BB_EMPTY {
            let to = targets.poplsb();
            let m = Move {
                from,
                to,
                flag: MoveFlag::Quiet,
            };

            if !move_puts_self_in_check(board, &m) {
                moves.push(m);
            }
        }
    }

    let mut bishops = board.get_piece_bb(Piece::from_type(PieceType::Bishop, board.stm));
    while bishops != BB_EMPTY {
        let from = bishops.poplsb();
        let mut targets = BISHOP_ATTACKS[from as usize] & !pieces;
        while targets != BB_EMPTY {
            let to = targets.poplsb();
            let m = Move {
                from,
                to,
                flag: MoveFlag::Quiet,
            };

            if in_between(from, to) & pieces == BB_EMPTY && !move_puts_self_in_check(board, &m) {
                moves.push(m);
            }
        }
    }

    let mut rooks = board.get_piece_bb(Piece::from_type(PieceType::Rook, board.stm));
    while rooks != BB_EMPTY {
        let from = rooks.poplsb();
        let mut targets = ROOK_ATTACKS[from as usize] & !pieces;
        while targets != BB_EMPTY {
            let to = targets.poplsb();
            let m = Move {
                from,
                to,
                flag: MoveFlag::Quiet,
            };

            if in_between(from, to) & pieces == BB_EMPTY && !move_puts_self_in_check(board, &m) {
                moves.push(m);
            }
        }
    }

    let mut queens = board.get_piece_bb(Piece::from_type(PieceType::Queen, board.stm));
    while queens != BB_EMPTY {
        let from = queens.poplsb();
        let mut targets = QUEEN_ATTACKS[from as usize] & !pieces;
        while targets != BB_EMPTY {
            let to = targets.poplsb();
            let m = Move {
                from,
                to,
                flag: MoveFlag::Quiet,
            };

            if in_between(from, to) & pieces == BB_EMPTY && !move_puts_self_in_check(board, &m) {
                moves.push(m);
            }
        }
    }
}

pub fn loud_moves(board: &Board, moves: &mut Vec<Move>) {
    pawn_captures(board, moves);
    pawn_promotions(board, moves);
    pawn_en_passant(board, moves);

    let pieces = board.occupied_squares();
    let targets = board.get_pieces(!board.stm);

    let mut knights = board.get_piece_bb(Piece::from_type(PieceType::Knight, board.stm));
    while knights != BB_EMPTY {
        let from = knights.poplsb();
        let mut targets = KNIGHT_ATTACKS[from as usize] & targets;
        while targets != BB_EMPTY {
            let to = targets.poplsb();
            let m = Move {
                from: from,
                to: to,
                flag: MoveFlag::Capture,
            };

            if !move_puts_self_in_check(board, &m) {
                moves.push(m);
            }
        }
    }

    let mut kings = board.get_piece_bb(Piece::from_type(PieceType::King, board.stm));
    while kings != BB_EMPTY {
        let from = kings.poplsb();
        let mut targets = KING_ATTACKS[from as usize] & targets;
        while targets != BB_EMPTY {
            let to = targets.poplsb();
            let m = Move {
                from,
                to,
                flag: MoveFlag::Capture,
            };

            if !move_puts_self_in_check(board, &m) {
                moves.push(m);
            }
        }
    }

    let mut bishops = board.get_piece_bb(Piece::from_type(PieceType::Bishop, board.stm));
    while bishops != BB_EMPTY {
        let from = bishops.poplsb();
        let mut targets = BISHOP_ATTACKS[from as usize] & targets;
        while targets != BB_EMPTY {
            let to = targets.poplsb();
            let m = Move {
                from,
                to,
                flag: MoveFlag::Capture,
            };

            if in_between(from, to) & pieces == BB_EMPTY && !move_puts_self_in_check(board, &m) {
                moves.push(m);
            }
        }
    }

    let mut rooks = board.get_piece_bb(Piece::from_type(PieceType::Rook, board.stm));
    while rooks != BB_EMPTY {
        let from = rooks.poplsb();
        let mut targets = ROOK_ATTACKS[from as usize] & targets;
        while targets != BB_EMPTY {
            let to = targets.poplsb();
            let m = Move {
                from,
                to,
                flag: MoveFlag::Capture,
            };

            if in_between(from, to) & pieces == BB_EMPTY && !move_puts_self_in_check(board, &m) {
                moves.push(m);
            }
        }
    }

    let mut queens = board.get_piece_bb(Piece::from_type(PieceType::Queen, board.stm));
    while queens != BB_EMPTY {
        let from = queens.poplsb();
        let mut targets = QUEEN_ATTACKS[from as usize] & targets;
        while targets != BB_EMPTY {
            let to = targets.poplsb();
            let m = Move {
                from,
                to,
                flag: MoveFlag::Capture,
            };

            if in_between(from, to) & pieces == BB_EMPTY && !move_puts_self_in_check(board, &m) {
                moves.push(m);
            }
        }
    }
}

pub fn is_in_check(board: &Board, s: Side) -> bool {
    is_square_threatened(board, board.get_king(s), s)
}

fn pawn_pushes(board: &Board, moves: &mut Vec<Move>) {
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
        let start = Square::from_index((end as i8 - forward) as usize).unwrap();

        let m = Move {
            from: start,
            to: end,
            flag: MoveFlag::Quiet,
        };

        if !move_puts_self_in_check(board, &m) {
            moves.push(m);
        }
    }
}

fn pawn_promotions(board: &Board, moves: &mut Vec<Move>) {
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
        let start = Square::from_index((end as i8 - forward) as usize).unwrap();

        let m = Move {
            from: start,
            to: end,
            flag: MoveFlag::KnightPromotion,
        };

        if !move_puts_self_in_check(board, &m) {
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

fn pawn_double_pushes(board: &Board, moves: &mut Vec<Move>) {
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
        let start = Square::from_index((end as i8 - forward) as usize).unwrap();

        let m = Move {
            from: start,
            to: end,
            flag: MoveFlag::PawnDoubleMove,
        };

        if !move_puts_self_in_check(board, &m) {
            moves.push(m);
        }
    }
}

fn pawn_en_passant(board: &Board, moves: &mut Vec<Move>) {
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

fn pawn_captures(board: &Board, moves: &mut Vec<Move>) {
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
        let start = Square::from_index((end as i8 - forward_left) as usize).unwrap();

        let m = Move {
            from: start,
            to: end,
            flag: MoveFlag::Capture,
        };

        if move_puts_self_in_check(board, &m) {
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
        let start = Square::from_index((end as i8 - forward_right) as usize).unwrap();

        let m = Move {
            from: start,
            to: end,
            flag: MoveFlag::Capture,
        };

        if move_puts_self_in_check(board, &m) {
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

fn castle_moves(board: &Board, moves: &mut Vec<Move>) {
    let pieces = board.occupied_squares();

    if board.stm == Side::White
        && board.white_king_castle
        && in_between(Square::E1, Square::H1) & pieces == BB_EMPTY
        && !is_square_threatened(board, Square::E1, board.stm)
        && !is_square_threatened(board, Square::F1, board.stm)
        && !is_square_threatened(board, Square::G1, board.stm)
    {
        moves.push(Move {
            from: Square::E1,
            to: Square::G1,
            flag: MoveFlag::KingCastle,
        });
    }

    if board.stm == Side::White
        && board.white_queen_castle
        && in_between(Square::E1, Square::A1) & pieces == BB_EMPTY
        && !is_square_threatened(board, Square::E1, board.stm)
        && !is_square_threatened(board, Square::D1, board.stm)
        && !is_square_threatened(board, Square::C1, board.stm)
    {
        moves.push(Move {
            from: Square::E1,
            to: Square::C1,
            flag: MoveFlag::QueenCastle,
        });
    }

    if board.stm == Side::Black
        && board.black_king_castle
        && in_between(Square::E8, Square::H8) & pieces == BB_EMPTY
        && !is_square_threatened(board, Square::E8, board.stm)
        && !is_square_threatened(board, Square::F8, board.stm)
        && !is_square_threatened(board, Square::G8, board.stm)
    {
        moves.push(Move {
            from: Square::E8,
            to: Square::G8,
            flag: MoveFlag::KingCastle,
        });
    }

    if board.stm == Side::Black
        && board.black_queen_castle
        && in_between(Square::E8, Square::A8) & pieces == BB_EMPTY
        && !is_square_threatened(board, Square::E8, board.stm)
        && !is_square_threatened(board, Square::D8, board.stm)
        && !is_square_threatened(board, Square::C8, board.stm)
    {
        moves.push(Move {
            from: Square::E8,
            to: Square::C8,
            flag: MoveFlag::QueenCastle,
        });
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
