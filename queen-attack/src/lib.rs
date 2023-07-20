const MAX_CHESS_LEN: i32 = 8;

#[derive(Debug)]
pub struct ChessPosition {
    row: i32,
    col: i32,
}

#[derive(Debug)]
pub struct Queen {
    position: ChessPosition,
}

impl ChessPosition {
    pub fn new(rank: i32, file: i32) -> Option<Self> {
        if rank < 0 || file < 0 || rank >= MAX_CHESS_LEN || file >= MAX_CHESS_LEN {
            None
        } else {
            Some(ChessPosition {
                row: rank,
                col: file,
            })
        }
    }

    fn meet_each_other(&self, other: &ChessPosition) -> bool {
        self.row == other.row
            || self.col == other.col
            || (self.row - other.row).abs() == (self.col - other.col).abs()
    }
}

impl Queen {
    pub fn new(position: ChessPosition) -> Self {
        Queen { position }
    }

    pub fn can_attack(&self, other: &Queen) -> bool {
        self.position.meet_each_other(&other.position)
    }
}
