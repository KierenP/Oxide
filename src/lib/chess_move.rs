use crate::definitions::*;

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum MoveFlag {
    Quiet,                  //0000
    PawnDoubleMove,         //0001
    KingCastle,             //0010
    QueenCastle,            //0011
    Capture,                //0100
    EnPassant = 5,          //0101
    KnightPromotion = 8,    //1000
    BishopPromotion,        //1001
    RookPromotion,          //1010
    QueenPromotion,         //1011
    KnightPromotionCapture, //1100
    BishopPromotionCapture, //1101
    RookPromotionCapture,   //1110
    QueenPromotionCapture,  //1111
}

#[derive(Debug, Copy, Clone)]
pub struct Move {
    pub from: Square,
    pub to: Square,
    pub flag: MoveFlag,
}

impl Move {
    pub const fn is_promotion(&self) -> bool {
        return self.flag as u8 & 0b1000 != 0;
    }

    pub const fn is_capture(&self) -> bool {
        return self.flag as u8 & 0b0100 != 0;
    }

    pub const fn is_castle(&self) -> bool {
        match self.flag {
            MoveFlag::KingCastle | MoveFlag::QueenCastle => true,
            _ => false,
        }
    }
}

impl std::fmt::Display for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}{}", self.from, self.to)
    }
}
