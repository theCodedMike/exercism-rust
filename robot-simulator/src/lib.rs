// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.
//
//       N
//       ↑
//  W <-   -> E
//       ↓
//       S
//

#[derive(PartialEq, Eq, Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn turn_right(&mut self) {
        match self {
            Direction::North => {
                *self = Direction::East;
            }
            Direction::East => {
                *self = Direction::South;
            }
            Direction::South => {
                *self = Direction::West;
            }
            Direction::West => {
                *self = Direction::North;
            }
        }
    }

    fn turn_left(&mut self) {
        match self {
            Direction::North => {
                *self = Direction::West;
            }
            Direction::East => {
                *self = Direction::North;
            }
            Direction::South => {
                *self = Direction::East;
            }
            Direction::West => {
                *self = Direction::South;
            }
        }
    }
}

pub struct Robot {
    x: i32,
    y: i32,
    d: Direction,
}

impl Robot {
    pub fn new(x: i32, y: i32, d: Direction) -> Self {
        Robot { x, y, d }
    }

    #[must_use]
    pub fn turn_right(mut self) -> Self {
        self.d.turn_right();
        self
    }

    #[must_use]
    pub fn turn_left(mut self) -> Self {
        self.d.turn_left();
        self
    }

    #[must_use]
    pub fn advance(mut self) -> Self {
        let (ad_x, ad_y) = match &self.d {
            Direction::North => (0, 1),
            Direction::East => (1, 0),
            Direction::South => (0, -1),
            Direction::West => (-1, 0),
        };
        self.x += ad_x;
        self.y += ad_y;

        self
    }

    #[must_use]
    pub fn instructions(mut self, instructions: &str) -> Self {
        for c in instructions.chars() {
            match c {
                'R' => {
                    self = self.turn_right();
                }
                'A' => {
                    self = self.advance();
                }
                'L' => {
                    self = self.turn_left();
                }
                _ => panic!("unsupported instruction: {c}"),
            }
        }

        self
    }

    pub fn position(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    pub fn direction(&self) -> &Direction {
        &self.d
    }
}
