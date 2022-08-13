use crate::{
    chess_move::{Move, MoveFlag},
    definitions::*,
};

use strum::{EnumCount, IntoEnumIterator};

#[derive(Copy, Clone, PartialEq)]
pub struct Board {
    pieces: [BB; Piece::COUNT],

    pub en_passant: Option<Square>,
    pub halfmove_clock: u32,
    pub fullmove_number: u32,
    pub stm: Side,

    pub white_king_castle: bool,
    pub white_queen_castle: bool,
    pub black_king_castle: bool,
    pub black_queen_castle: bool,
}

impl Board {
    pub fn new() -> Board {
        Board {
            pieces: [BB_EMPTY; Piece::COUNT],
            en_passant: None,
            halfmove_clock: 0,
            fullmove_number: 1,
            stm: Side::White,
            white_king_castle: true,
            white_queen_castle: true,
            black_king_castle: true,
            black_queen_castle: true,
        }
    }

    pub fn clear_square(&mut self, square: Square) {
        for piece in self.pieces.iter_mut() {
            *piece &= !square.to_bb();
        }
    }

    pub fn set_square(&mut self, square: Square, piece: Piece) {
        self.clear_square(square);
        self.pieces[piece as usize] |= square.to_bb();
    }

    pub fn get_square(&self, square: Square) -> Option<Piece> {
        for piece in Piece::iter() {
            if self.pieces[piece as usize] & square.to_bb() != BB_EMPTY {
                return Some(piece);
            }
        }
        None
    }

    pub fn empty_squares(&self) -> BB {
        BB_FULL ^ self.occupied_squares()
    }

    pub fn occupied_squares(&self) -> BB {
        let mut bb = BB_EMPTY;
        for piece in Piece::iter() {
            bb |= self.pieces[piece as usize];
        }
        bb
    }

    pub fn get_piece_bb(&self, piece: Piece) -> BB {
        self.pieces[piece as usize]
    }

    pub fn make_move(&mut self, m: &Move) {
        let from_piece = self
            .get_square(m.from)
            .unwrap_or_else(|| panic!("No piece at {:?}", m.from));

        if m.is_capture() || from_piece == Piece::from_type(PieceType::Pawn, self.stm) {
            self.halfmove_clock = 0;
        } else {
            self.halfmove_clock += 1;
        }

        // set the 'to' square with the 'from' piece, except if it's a promotion
        if !m.is_promotion() {
            self.set_square(m.to, from_piece);
        }

        // for castle moves, make sure we move the rook
        if m.flag == MoveFlag::KingCastle && self.stm == Side::White {
            self.set_square(Square::F1, Piece::WhiteRook);
            self.clear_square(Square::H1);
        }

        if m.flag == MoveFlag::QueenCastle && self.stm == Side::White {
            self.set_square(Square::D1, Piece::WhiteRook);
            self.clear_square(Square::A1);
        }

        if m.flag == MoveFlag::KingCastle && self.stm == Side::Black {
            self.set_square(Square::F8, Piece::BlackRook);
            self.clear_square(Square::H8);
        }

        if m.flag == MoveFlag::QueenCastle && self.stm == Side::Black {
            self.set_square(Square::D8, Piece::BlackRook);
            self.clear_square(Square::A8);
        }

        if m.flag == MoveFlag::PawnDoubleMove {
            // For the context of the 3-fold repetition rule, an ep square only should be set
            // if there is a pawn that could legally do the ep capture. This is somewhat of a
            // strange quirk in the chess rules and is a pain to implement so we ignore this.
            self.en_passant =
                Some(Square::from_index((m.to as usize + m.from as usize) / 2).unwrap());
        } else {
            self.en_passant = None;
        }

        if m.flag == MoveFlag::EnPassant {
            // This works for white or black. The ep capture happens on the file of the target
            // square, but on the rank of the from square.
            self.clear_square(Square::from_coord(m.to.file(), m.from.rank()));
        }

        if m.flag == MoveFlag::KnightPromotion || m.flag == MoveFlag::KnightPromotionCapture {
            self.set_square(m.to, Piece::from_type(PieceType::Knight, self.stm));
        }

        if m.flag == MoveFlag::BishopPromotion || m.flag == MoveFlag::BishopPromotionCapture {
            self.set_square(m.to, Piece::from_type(PieceType::Bishop, self.stm));
        }

        if m.flag == MoveFlag::RookPromotion || m.flag == MoveFlag::RookPromotionCapture {
            self.set_square(m.to, Piece::from_type(PieceType::Rook, self.stm));
        }

        if m.flag == MoveFlag::QueenPromotion || m.flag == MoveFlag::QueenPromotionCapture {
            self.set_square(m.to, Piece::from_type(PieceType::Queen, self.stm));
        }

        if self.stm == Side::White {
            self.fullmove_number += 1;
        }

        self.stm = !self.stm;
        self.clear_square(m.from);
        self.update_castle_rights(m);
    }

    pub fn get_king(&self, s: Side) -> Square {
        self.get_piece_bb(Piece::from_type(PieceType::King, s))
            .lsb()
    }

    pub fn get_pieces(&self, s: Side) -> BB {
        self.get_piece_bb(Piece::from_type(PieceType::Pawn, s))
            | self.get_piece_bb(Piece::from_type(PieceType::Knight, s))
            | self.get_piece_bb(Piece::from_type(PieceType::Bishop, s))
            | self.get_piece_bb(Piece::from_type(PieceType::Rook, s))
            | self.get_piece_bb(Piece::from_type(PieceType::Queen, s))
            | self.get_piece_bb(Piece::from_type(PieceType::King, s))
    }

    fn update_castle_rights(&mut self, m: &Move) {
        if m.from == Square::E1 {
            self.white_king_castle = false;
            self.white_queen_castle = false;
        }

        if m.from == Square::E8 {
            self.black_king_castle = false;
            self.black_queen_castle = false;
        }

        if m.from == Square::A1 || m.to == Square::A1 {
            self.white_queen_castle = false;
        }

        if m.from == Square::A8 || m.to == Square::A8 {
            self.black_queen_castle = false;
        }

        if m.from == Square::H1 || m.to == Square::H1 {
            self.white_king_castle = false;
        }

        if m.from == Square::H8 || m.to == Square::H8 {
            self.black_king_castle = false;
        }
    }
}
