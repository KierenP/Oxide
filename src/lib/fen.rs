use crate::board::*;
use crate::definitions::*;

pub fn fen_to_board(fen: &str) -> Result<Board, String> {
    let mut board = Board::new();
    let mut fen_iter = fen.split_whitespace();
    let mut i = 0;

    // board
    for c in fen_iter.next().ok_or("Fen cannot be empty")?.chars() {
        match c {
            '1'..='8' => i += c.to_digit(10).unwrap() as usize,
            '/' => (),
            _ => {
                let square = Square::from_index((7 - (i / 8)) * 8 + i % 8)?;
                let piece = char_to_piece(c)?;
                board.set_square(square, piece);
                i += 1;
            }
        }
    }

    // stm
    let stm = fen_iter.next().ok_or("Fen is too short")?;
    board.stm = str_to_side(stm)?;

    // casteling rights
    let castling = fen_iter.next().ok_or("Fen is too short")?;
    board.white_king_castle = castling.contains('K');
    board.white_queen_castle = castling.contains('Q');
    board.black_king_castle = castling.contains('k');
    board.black_queen_castle = castling.contains('q');

    // en passant
    board.en_passant = match fen_iter.next().ok_or("Fen is too short")? {
        "-" => None,
        ep => Some(str_to_square(ep)?),
    };

    // halfmove clock
    let str = fen_iter.next().ok_or("Fen is too short")?;
    board.halfmove_clock = str.parse().expect("Got invalid halfmove clock {str}");

    // fullmove number
    let str = fen_iter.next().ok_or("Fen is too short")?;
    board.fullmove_number = str.parse().expect("Got invalid fullmove number {str}");

    Ok(board)
}

pub fn board_to_fen(board: &Board) -> String {
    let mut fen = String::new();

    // iterate over the board, from top to bottom, left to right
    for rank in (0..8).rev() {
        let mut empty = 0;
        for file in 0..8 {
            let square = Square::from_index(rank * 8 + file).expect("Invalid square");
            let piece = board.get_square(square);
            if piece.is_some() {
                if empty > 0 {
                    fen.push_str(&empty.to_string());
                    empty = 0;
                }
                fen.push(piece_to_char(piece.unwrap()));
            } else {
                empty += 1;
            }
        }
        if empty > 0 {
            fen.push_str(&empty.to_string());
        }
        if rank != 0 {
            fen.push('/');
        }
    }

    fen.push_str(format!(" {}", side_to_char(board.stm)).as_str());
    fen.push_str(
        format!(
            " {}",
            castling_to_str(
                board.white_king_castle,
                board.white_queen_castle,
                board.black_king_castle,
                board.black_queen_castle
            )
        )
        .as_str(),
    );
    fen.push_str(format!(" {}", ep_to_str(board.en_passant)).as_str());
    fen.push_str(format!(" {}", board.halfmove_clock).as_str());
    fen.push_str(format!(" {}", board.fullmove_number).as_str());
    fen
}

fn str_to_side(s: &str) -> Result<Side, String> {
    match s {
        "w" => Ok(Side::White),
        "b" => Ok(Side::Black),
        _ => Err(format!("Invalid side {s}")),
    }
}

fn side_to_char(s: Side) -> char {
    match s {
        Side::White => 'w',
        Side::Black => 'b',
    }
}

fn char_to_piece(c: char) -> Result<Piece, String> {
    match c {
        'P' => Ok(Piece::WhitePawn),
        'N' => Ok(Piece::WhiteKnight),
        'B' => Ok(Piece::WhiteBishop),
        'R' => Ok(Piece::WhiteRook),
        'Q' => Ok(Piece::WhiteQueen),
        'K' => Ok(Piece::WhiteKing),
        'p' => Ok(Piece::BlackPawn),
        'n' => Ok(Piece::BlackKnight),
        'b' => Ok(Piece::BlackBishop),
        'r' => Ok(Piece::BlackRook),
        'q' => Ok(Piece::BlackQueen),
        'k' => Ok(Piece::BlackKing),
        _ => Err(format!("Invalid piece {c}")),
    }
}

fn piece_to_char(p: Piece) -> char {
    match p {
        Piece::WhitePawn => 'P',
        Piece::WhiteKnight => 'N',
        Piece::WhiteBishop => 'B',
        Piece::WhiteRook => 'R',
        Piece::WhiteQueen => 'Q',
        Piece::WhiteKing => 'K',
        Piece::BlackPawn => 'p',
        Piece::BlackKnight => 'n',
        Piece::BlackBishop => 'b',
        Piece::BlackRook => 'r',
        Piece::BlackQueen => 'q',
        Piece::BlackKing => 'k',
    }
}

fn str_to_square(s: &str) -> Result<Square, String> {
    let file = s
        .chars()
        .nth(0)
        .ok_or(format!("Square string too short {s}"))?;
    let rank = s
        .chars()
        .nth(1)
        .ok_or(format!("Square string too short {s}"))?;

    if rank < '1' || rank > '8' || file < 'a' || file > 'h' {
        return Err(format!("Invalid square string {s}"));
    }

    Square::from_index((rank as usize - '1' as usize) * 8 + (file as usize - 'a' as usize))
}

fn ep_to_str(square: Option<Square>) -> String {
    if square.is_none() {
        return "-".to_string();
    }

    let square = square.unwrap();
    let file = square as u8 % 8;
    let rank = square as u8 / 8;

    format!(
        "{}{}",
        (file + 'a' as u8) as char,
        (rank + '1' as u8) as char
    )
}

fn castling_to_str(wk: bool, wq: bool, bk: bool, bq: bool) -> String {
    let mut s = String::new();
    if wk {
        s.push_str("K");
    }
    if wq {
        s.push_str("Q");
    }
    if bk {
        s.push_str("k");
    }
    if bq {
        s.push_str("q");
    }
    if !wk && !wq && !bk && !bq {
        s.push_str("-");
    }
    s
}

#[cfg(test)]
mod tests {
    use crate::fen::*;

    const POSITIONS: [&str; 36] = [
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
        "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 10",
        "8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 11",
        "4rrk1/pp1n3p/3q2pQ/2p1pb2/2PP4/2P3N1/P2B2PP/4RRK1 b - - 7 19",
        "rq3rk1/ppp2ppp/1bnpb3/3N2B1/3NP3/7P/PPPQ1PP1/2KR3R w - - 7 14",
        "r1bq1r1k/1pp1n1pp/1p1p4/4p2Q/4Pp2/1BNP4/PPP2PPP/3R1RK1 w - - 2 14",
        "r3r1k1/2p2ppp/p1p1bn2/8/1q2P3/2NPQN2/PPP3PP/R4RK1 b - - 2 15",
        "r1bbk1nr/pp3p1p/2n5/1N4p1/2Np1B2/8/PPP2PPP/2KR1B1R w kq - 0 13",
        "r1bq1rk1/ppp1nppp/4n3/3p3Q/3P4/1BP1B3/PP1N2PP/R4RK1 w - - 1 16",
        "4r1k1/r1q2ppp/ppp2n2/4P3/5Rb1/1N1BQ3/PPP3PP/R5K1 w - - 1 17",
        "2rqkb1r/ppp2p2/2npb1p1/1N1Nn2p/2P1PP2/8/PP2B1PP/R1BQK2R b KQ - 0 11",
        "r1bq1r1k/b1p1npp1/p2p3p/1p6/3PP3/1B2NN2/PP3PPP/R2Q1RK1 w - - 1 16",
        "3r1rk1/p5pp/bpp1pp2/8/q1PP1P2/b3P3/P2NQRPP/1R2B1K1 b - - 6 22",
        "r1q2rk1/2p1bppp/2Pp4/p6b/Q1PNp3/4B3/PP1R1PPP/2K4R w - - 2 18",
        "4k2r/1pb2ppp/1p2p3/1R1p4/3P4/2r1PN2/P4PPP/1R4K1 b - - 3 22",
        "3q2k1/pb3p1p/4pbp1/2r5/PpN2N2/1P2P2P/5PP1/Q2R2K1 b - - 4 26",
        "6k1/6p1/6Pp/ppp5/3pn2P/1P3K2/1PP2P2/3N4 b - - 0 1",
        "3b4/5kp1/1p1p1p1p/pP1PpP1P/P1P1P3/3KN3/8/8 w - - 0 1",
        "2K5/p7/7P/5pR1/8/5k2/r7/8 w - - 0 1",
        "8/6pk/1p6/8/PP3p1p/5P2/4KP1q/3Q4 w - - 0 1",
        "7k/3p2pp/4q3/8/4Q3/5Kp1/P6b/8 w - - 0 1",
        "8/2p5/8/2kPKp1p/2p4P/2P5/3P4/8 w - - 0 1",
        "8/1p3pp1/7p/5P1P/2k3P1/8/2K2P2/8 w - - 0 1",
        "8/pp2r1k1/2p1p3/3pP2p/1P1P1P1P/P5KR/8/8 w - - 0 1",
        "8/3p4/p1bk3p/Pp6/1Kp1PpPp/2P2P1P/2P5/5B2 b - - 0 1",
        "5k2/7R/4P2p/5K2/p1r2P1p/8/8/8 b - - 0 1",
        "6k1/6p1/P6p/r1N5/5p2/7P/1b3PP1/4R1K1 w - - 0 1",
        "1r3k2/4q3/2Pp3b/3Bp3/2Q2p2/1p1P2P1/1P2KP2/3N4 w - - 0 1",
        "6k1/4pp1p/3p2p1/P1pPb3/R7/1r2P1PP/3B1P2/6K1 w - - 0 1",
        "8/3p3B/5p2/5P2/p7/PP5b/k7/6K1 w - - 0 1",
        "5rk1/q6p/2p3bR/1pPp1rP1/1P1Pp3/P3B1Q1/1K3P2/R7 w - - 93 90",
        "4rrk1/1p1nq3/p7/2p1P1pp/3P2bp/3Q1Bn1/PPPB4/1K2R1NR w - - 40 21",
        "r3k2r/3nnpbp/q2pp1p1/p7/Pp1PPPP1/4BNN1/1P5P/R2Q1RK1 w kq - 0 16",
        "3Qb1k1/1r2ppb1/pN1n2q1/Pp1Pp1Pr/4P2p/4BP2/4B1R1/1R5K b - - 11 40",
        "4k3/3q1r2/1N2r1b1/3ppN2/2nPP3/1B1R2n1/2R1Q3/3K4 w - - 5 1",
        "r3k2r/p1ppqpb1/bn2pnp1/3PN3/Pp2P3/2N2Q1p/1PPBBPPP/R3K2R b KQkq g3 0 1",
    ];

    #[test]
    fn test_fen_round_trip() {
        for fen in POSITIONS.iter() {
            let board = fen_to_board(fen).unwrap();
            let output = board_to_fen(&board);
            assert_eq!(*fen, output);
        }
    }
}
