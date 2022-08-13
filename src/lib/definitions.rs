use strum::{EnumCount, EnumIter};

#[derive(EnumCount, Copy, Clone, PartialEq)]
pub enum Side {
    White,
    Black,
}

impl std::ops::Not for Side {
    type Output = Self;

    fn not(self) -> Self {
        match self {
            Side::White => Side::Black,
            Side::Black => Side::White,
        }
    }
}

#[derive(EnumIter, EnumCount, Copy, Clone, PartialEq)]
pub enum Piece {
    WhitePawn,
    WhiteKnight,
    WhiteBishop,
    WhiteRook,
    WhiteQueen,
    WhiteKing,
    BlackPawn,
    BlackKnight,
    BlackBishop,
    BlackRook,
    BlackQueen,
    BlackKing,
}

impl Piece {
    pub fn from_type(t: PieceType, s: Side) -> Piece {
        match (t, s) {
            (PieceType::Pawn, Side::White) => Piece::WhitePawn,
            (PieceType::Knight, Side::White) => Piece::WhiteKnight,
            (PieceType::Bishop, Side::White) => Piece::WhiteBishop,
            (PieceType::Rook, Side::White) => Piece::WhiteRook,
            (PieceType::Queen, Side::White) => Piece::WhiteQueen,
            (PieceType::King, Side::White) => Piece::WhiteKing,
            (PieceType::Pawn, Side::Black) => Piece::BlackPawn,
            (PieceType::Knight, Side::Black) => Piece::BlackKnight,
            (PieceType::Bishop, Side::Black) => Piece::BlackBishop,
            (PieceType::Rook, Side::Black) => Piece::BlackRook,
            (PieceType::Queen, Side::Black) => Piece::BlackQueen,
            (PieceType::King, Side::Black) => Piece::BlackKing,
        }
    }
}

pub enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

#[rustfmt::skip]
#[derive(EnumCount, Copy, Clone, Debug, PartialEq)]
pub enum Square {
    A1, B1, C1, D1, E1, F1, G1, H1,
    A2, B2, C2, D2, E2, F2, G2, H2,
    A3, B3, C3, D3, E3, F3, G3, H3,
    A4, B4, C4, D4, E4, F4, G4, H4,
    A5, B5, C5, D5, E5, F5, G5, H5,
    A6, B6, C6, D6, E6, F6, G6, H6,
    A7, B7, C7, D7, E7, F7, G7, H7,
    A8, B8, C8, D8, E8, F8, G8, H8,
}

impl Square {
    pub fn from_index(x: usize) -> Result<Square, String> {
        match x {
            0 => Ok(Square::A1),
            1 => Ok(Square::B1),
            2 => Ok(Square::C1),
            3 => Ok(Square::D1),
            4 => Ok(Square::E1),
            5 => Ok(Square::F1),
            6 => Ok(Square::G1),
            7 => Ok(Square::H1),
            8 => Ok(Square::A2),
            9 => Ok(Square::B2),
            10 => Ok(Square::C2),
            11 => Ok(Square::D2),
            12 => Ok(Square::E2),
            13 => Ok(Square::F2),
            14 => Ok(Square::G2),
            15 => Ok(Square::H2),
            16 => Ok(Square::A3),
            17 => Ok(Square::B3),
            18 => Ok(Square::C3),
            19 => Ok(Square::D3),
            20 => Ok(Square::E3),
            21 => Ok(Square::F3),
            22 => Ok(Square::G3),
            23 => Ok(Square::H3),
            24 => Ok(Square::A4),
            25 => Ok(Square::B4),
            26 => Ok(Square::C4),
            27 => Ok(Square::D4),
            28 => Ok(Square::E4),
            29 => Ok(Square::F4),
            30 => Ok(Square::G4),
            31 => Ok(Square::H4),
            32 => Ok(Square::A5),
            33 => Ok(Square::B5),
            34 => Ok(Square::C5),
            35 => Ok(Square::D5),
            36 => Ok(Square::E5),
            37 => Ok(Square::F5),
            38 => Ok(Square::G5),
            39 => Ok(Square::H5),
            40 => Ok(Square::A6),
            41 => Ok(Square::B6),
            42 => Ok(Square::C6),
            43 => Ok(Square::D6),
            44 => Ok(Square::E6),
            45 => Ok(Square::F6),
            46 => Ok(Square::G6),
            47 => Ok(Square::H6),
            48 => Ok(Square::A7),
            49 => Ok(Square::B7),
            50 => Ok(Square::C7),
            51 => Ok(Square::D7),
            52 => Ok(Square::E7),
            53 => Ok(Square::F7),
            54 => Ok(Square::G7),
            55 => Ok(Square::H7),
            56 => Ok(Square::A8),
            57 => Ok(Square::B8),
            58 => Ok(Square::C8),
            59 => Ok(Square::D8),
            60 => Ok(Square::E8),
            61 => Ok(Square::F8),
            62 => Ok(Square::G8),
            63 => Ok(Square::H8),
            _ => Err(format!("Invalid square: {}", x)),
        }
    }

    pub fn file(&self) -> File {
        File::from_index(*self as usize % 8).unwrap()
    }

    /// Returns the rank of the square.
    ///
    /// # Examples
    /// ```
    /// # use oxide::definitions::*;
    /// assert_eq!(Square::G2.rank(), Rank::Two);
    /// ```
    pub fn rank(&self) -> Rank {
        Rank::from_index(*self as usize / 8).unwrap()
    }

    pub fn from_coord(file: File, rank: Rank) -> Square {
        Square::from_index(rank as usize * 8 + file as usize).unwrap()
    }

    pub fn to_bb(&self) -> BB {
        BB(1) << *self as i32
    }
}

impl std::fmt::Display for Square {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let file = *self as u8 % 8;
        let rank = *self as u8 / 8;
        write!(
            f,
            "{}{}",
            (file + 'a' as u8) as char,
            (rank + '1' as u8) as char
        )
    }
}

#[derive(EnumCount, PartialEq, Debug)]
pub enum Rank {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
}

impl Rank {
    pub fn from_index(x: usize) -> Result<Rank, String> {
        match x {
            0 => Ok(Rank::One),
            1 => Ok(Rank::Two),
            2 => Ok(Rank::Three),
            3 => Ok(Rank::Four),
            4 => Ok(Rank::Five),
            5 => Ok(Rank::Six),
            6 => Ok(Rank::Seven),
            7 => Ok(Rank::Eight),
            _ => Err(format!("Invalid rank: {}", x)),
        }
    }
}

#[derive(EnumCount)]
pub enum File {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
}

impl File {
    pub fn from_index(x: usize) -> Result<File, String> {
        match x {
            0 => Ok(File::A),
            1 => Ok(File::B),
            2 => Ok(File::C),
            3 => Ok(File::D),
            4 => Ok(File::E),
            5 => Ok(File::F),
            6 => Ok(File::G),
            7 => Ok(File::H),
            _ => Err(format!("Invalid file: {}", x)),
        }
    }
}

#[derive(EnumCount)]
pub enum Diagonal {
    A8A8,
    A7B8,
    A6C8,
    A5D8,
    A4E8,
    A3F8,
    A2G8,
    A1H8,
    B1H7,
    C1H6,
    D1H5,
    E1H4,
    F1H3,
    G1H2,
    H1H1,
}

#[derive(EnumCount)]
pub enum AntiDiagonal {
    H8H8,
    G8H7,
    F8H6,
    E8H5,
    D8H4,
    C8H3,
    B8H2,
    A8H1,
    A7G1,
    A6F1,
    A5E1,
    A4D1,
    A3C1,
    A2B1,
    A1A1,
}

//------------------------------------------------------------------------------

#[derive(Copy, Clone, PartialEq)]
pub struct BB(u64);

impl std::ops::Not for BB {
    type Output = Self;

    fn not(self) -> Self {
        Self(!self.0)
    }
}

impl std::ops::BitAndAssign for BB {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0 &= rhs.0
    }
}

impl std::ops::BitOrAssign for BB {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0
    }
}

impl std::ops::BitAnd for BB {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self(self.0 & rhs.0)
    }
}

impl std::ops::BitOr for BB {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl std::ops::BitXor for BB {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        Self(self.0 ^ rhs.0)
    }
}

impl std::ops::Shl<i32> for BB {
    type Output = Self;

    fn shl(self, rhs: i32) -> Self::Output {
        Self(self.0 << rhs)
    }
}

impl std::ops::Shr<i32> for BB {
    type Output = Self;

    fn shr(self, rhs: i32) -> Self::Output {
        Self(self.0 >> rhs)
    }
}

impl core::fmt::Debug for BB {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        for i in (0..8).rev() {
            write!(f, "\n")?;
            for j in 0..8 {
                if *self & SQUARE_BB[i * 8 + j] == BB_EMPTY {
                    write!(f, ".")?;
                } else {
                    write!(f, "#")?;
                }
            }
        }
        write!(f, "\n")?;
        Ok(())
    }
}

pub const BB_EMPTY: BB = BB(0);
pub const BB_FULL: BB = BB(u64::MAX);

impl BB {
    pub fn lsb(&self) -> Square {
        assert!(*self != BB_EMPTY);

        let mut bb = self.0;
        let mut i = 0;
        while bb & 1 == 0 {
            bb = bb >> 1;
            i += 1;
        }
        Square::from_index(i as usize).unwrap()
    }

    pub fn poplsb(&mut self) -> Square {
        assert!(*self != BB_EMPTY);

        let index = self.lsb();
        self.0 &= self.0 - 1;
        index
    }
}

//------------------------------------------------------------------------------

pub const SQUARE_BB: [BB; Square::COUNT] = {
    let mut squares = [BB_EMPTY; Square::COUNT];
    let mut i = 0;
    while i < Square::COUNT {
        squares[i] = BB(1 << i);
        i += 1;
    }
    squares
};

pub const RANK_BB: [BB; Rank::COUNT] = {
    let mut bb = [BB_EMPTY; Rank::COUNT];
    let mut i = 0;
    while i < Rank::COUNT {
        bb[i] = BB(0xff << (8 * i));
        i += 1;
    }
    bb
};

pub const FILE_BB: [BB; File::COUNT] = {
    let mut bb = [BB_EMPTY; File::COUNT];
    let mut i = 0;
    while i < File::COUNT {
        bb[i] = BB(0x101010101010101 << i);
        i += 1;
    }
    bb
};

pub const DIAGONAL_BB: [BB; Diagonal::COUNT] = {
    let mut bb = [BB_EMPTY; Diagonal::COUNT];
    bb[0] = SQUARE_BB[Square::A8 as usize];
    let mut i = 1;
    while i < Diagonal::COUNT {
        if i > Diagonal::COUNT / 2 {
            bb[i] = BB((bb[i - 1].0) >> 8);
        } else {
            bb[i] = BB((bb[i - 1].0 << 1) | (bb[i - 1].0 >> 8));
        }
        i += 1;
    }
    bb
};

pub const ANTI_DIAGONAL_BB: [BB; AntiDiagonal::COUNT] = {
    let mut bb = [BB_EMPTY; AntiDiagonal::COUNT];
    bb[0] = SQUARE_BB[Square::H8 as usize];
    let mut i = 1;
    while i < AntiDiagonal::COUNT {
        if i > AntiDiagonal::COUNT / 2 {
            bb[i] = BB((bb[i - 1].0) >> 8);
        } else {
            bb[i] = BB((bb[i - 1].0 >> 1) | (bb[i - 1].0 >> 8));
        }
        i += 1;
    }
    bb
};

pub const KNIGHT_ATTACKS: [BB; Square::COUNT] = {
    let mut bb = [BB_EMPTY; Square::COUNT];
    let mut i = 0;
    while i < Square::COUNT {
        let mut j = 0;
        while j < Square::COUNT {
            let file_diff = ((i % 8) as i32 - (j % 8) as i32).abs();
            let rank_diff = ((i / 8) as i32 - (j / 8) as i32).abs();
            if (file_diff == 2 && rank_diff == 1) || (file_diff == 1 && rank_diff == 2) {
                bb[i].0 |= SQUARE_BB[j].0;
            }
            j += 1;
        }
        i += 1;
    }
    bb
};

pub const ROOK_ATTACKS: [BB; Square::COUNT] = {
    let mut bb = [BB_EMPTY; Square::COUNT];
    let mut i = 0;
    while i < Square::COUNT {
        bb[i].0 = (RANK_BB[i / 8].0 | FILE_BB[i % 8].0) ^ SQUARE_BB[i].0;
        i += 1;
    }
    bb
};

pub const BISHOP_ATTACKS: [BB; Square::COUNT] = {
    let mut bb = [BB_EMPTY; Square::COUNT];
    let mut i = 0;
    while i < Square::COUNT {
        bb[i].0 = (DIAGONAL_BB[7 - i / 8 + i % 8].0 | ANTI_DIAGONAL_BB[14 - i / 8 - i % 8].0)
            ^ SQUARE_BB[i].0;
        i += 1;
    }
    bb
};

pub const KING_ATTACKS: [BB; Square::COUNT] = {
    let mut bb = [BB_EMPTY; Square::COUNT];
    let mut i = 0;
    while i < Square::COUNT {
        let mut j = 0;
        while j < Square::COUNT {
            let file_diff = ((i % 8) as i32 - (j % 8) as i32).abs();
            let rank_diff = ((i / 8) as i32 - (j / 8) as i32).abs();
            if i != j && file_diff <= 1 && rank_diff <= 1 {
                bb[i].0 |= SQUARE_BB[j].0;
            }
            j += 1;
        }
        i += 1;
    }
    bb
};

pub const QUEEN_ATTACKS: [BB; Square::COUNT] = {
    let mut bb = [BB_EMPTY; Square::COUNT];
    let mut i = 0;
    while i < Square::COUNT {
        bb[i].0 = ROOK_ATTACKS[i].0 | BISHOP_ATTACKS[i].0;
        i += 1;
    }
    bb
};

/// A precomputed array of pawn attack masks.
/// the first index is the color, the second index is the square.
///
/// # Examples
/// ```
/// # use oxide::definitions::*;
/// assert_eq!(PAWN_ATTACKS[Side::White as usize][Square::B3 as usize], SQUARE_BB[Square::A4 as usize] | SQUARE_BB[Square::C4 as usize]);
/// ```
pub const PAWN_ATTACKS: [[BB; Square::COUNT]; Side::COUNT] = {
    let mut bb = [[BB_EMPTY; Square::COUNT]; Side::COUNT];
    let mut i = 0;
    while i < Square::COUNT {
        let mut j = 0;
        while j < Square::COUNT {
            let file_diff = ((i % 8) as i32 - (j % 8) as i32).abs();
            let rank_diff = (i / 8) as i32 - (j / 8) as i32;
            if file_diff == 1 && rank_diff == -1 {
                bb[Side::White as usize][i].0 |= SQUARE_BB[j].0;
            }
            if file_diff == 1 && rank_diff == 1 {
                bb[Side::Black as usize][i].0 |= SQUARE_BB[j].0;
            }
            j += 1;
        }
        i += 1;
    }
    bb
};

//------------------------------------------------------------------------------
// https://www.chessprogramming.org/Square_Attacked_By

/*pub const IN_BETWEEN: [[BB; Square::COUNT]; Square::COUNT] = {
    let mut ret = [[BB_EMPTY; Square::COUNT]; Square::COUNT];
    let mut i = 0;
    while i < Square::COUNT {
        let mut j = 0;
        while j < Square::COUNT {
            ret[i][j] = in_between(i, j);
            j += 1;
        }
        i += 1;
    }
    ret
};*/

/// Get the bitboard of the squares in between the two squares. If the two
/// squares do not share the same rank, file or diagonal, return BB_EMPTY.
///
/// # Examples
/// ```
/// # use oxide::definitions::*;
/// let mask = in_between(Square::C1, Square::C4);
/// assert_eq!(mask, SQUARE_BB[Square::C2 as usize] | SQUARE_BB[Square::C3 as usize]);
/// ```
pub fn in_between(sq1: Square, sq2: Square) -> BB {
    let sq1 = sq1 as u64;
    let sq2 = sq2 as u64;
    let m1 = u64::MAX;
    let a2a7 = 0x0001010101010100;
    let b2g7 = 0x0040201008040200;
    let h1b7 = 0x0002040810204080;
    let btwn;
    let rank;
    let file;
    let mut line;

    btwn = (m1 << sq1) ^ (m1 << sq2);
    file = (sq2 & 7).wrapping_sub(sq1 & 7);
    rank = ((sq2 | 7).wrapping_sub(sq1)) >> 3;
    line = ((file & 7).wrapping_sub(1)) & a2a7; /* a2a7 if same file */
    line += 2 * (((rank & 7).wrapping_sub(1)) >> 58); /* b1g1 if same rank */
    line += (((rank.wrapping_sub(file)) & 15).wrapping_sub(1)) & b2g7; /* b2g7 if same diagonal */
    line += (((rank.wrapping_add(file)) & 15).wrapping_sub(1)) & h1b7; /* h1b7 if same antidiag */
    line = line.wrapping_mul(btwn & btwn.wrapping_neg()); /* mul acts like shift by smaller square */
    return BB(line & btwn); /* return the bits on that line in-between */
}

#[cfg(test)]
mod tests {
    use crate::definitions::*;

    #[test]
    fn test_in_between() {
        assert_eq!(
            in_between(Square::C1, Square::C4),
            SQUARE_BB[Square::C2 as usize] | SQUARE_BB[Square::C3 as usize]
        );
        assert_eq!(
            in_between(Square::D3, Square::H3),
            SQUARE_BB[Square::E3 as usize]
                | SQUARE_BB[Square::F3 as usize]
                | SQUARE_BB[Square::G3 as usize]
        );
        assert_eq!(
            in_between(Square::A1, Square::C3),
            SQUARE_BB[Square::B2 as usize]
        );
        assert_eq!(
            in_between(Square::D5, Square::G2),
            SQUARE_BB[Square::E4 as usize] | SQUARE_BB[Square::F3 as usize]
        );

        assert_eq!(in_between(Square::F6, Square::F6), BB_EMPTY);
        assert_eq!(in_between(Square::F6, Square::G6), BB_EMPTY);
        assert_eq!(in_between(Square::F6, Square::G4), BB_EMPTY);

        for i in 0..Square::COUNT {
            for j in 0..Square::COUNT {
                test_in_between_symmetry(i, j);
            }
        }
    }

    fn test_in_between_symmetry(sq1: usize, sq2: usize) {
        let sq1 = Square::from_index(sq1).unwrap();
        let sq2 = Square::from_index(sq2).unwrap();
        assert_eq!(in_between(sq1, sq2), in_between(sq2, sq1));
    }
}
