use crate::helper::{file_parse_lazy, file_to_vec_transform};
use std::str::FromStr;
// use std::string::ParseError;
use std::num::ParseIntError;

#[derive(Copy, Clone)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Copy, Clone)]
pub enum RotationAmount {
    Quarter,       // 90 deg
    Half,          // 180 deg
    ThreeQuarters, // 270 deg.
}

#[derive(Copy, Clone)]
pub enum RotationDir {
    Left,
    Right,
}

#[derive(Copy, Clone)]
pub enum ShipInstruction {
    Movement {
        dir: Direction,
        amount: isize,
    },
    Rotation {
        dir: RotationDir,
        amount: RotationAmount,
    },
    Forwards {
        amount: isize,
    },
}

impl FromStr for ShipInstruction {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ShipInstruction::*;

        let action = s.chars().next().unwrap();
        let amount: isize = s[1..].parse::<isize>().unwrap();

        let rotation_amt = match amount {
            90 => Some(RotationAmount::Quarter),
            180 => Some(RotationAmount::Half),
            270 => Some(RotationAmount::ThreeQuarters),
            _ => None,
        };

        match action {
            'N' => Ok(Movement {
                dir: Direction::North,
                amount,
            }),
            'S' => Ok(Movement {
                dir: Direction::South,
                amount,
            }),
            'E' => Ok(Movement {
                dir: Direction::East,
                amount,
            }),
            'W' => Ok(Movement {
                dir: Direction::West,
                amount,
            }),
            'L' => Ok(Rotation {
                dir: RotationDir::Left,
                amount: rotation_amt.unwrap(),
            }),
            'R' => Ok(Rotation {
                dir: RotationDir::Right,
                amount: rotation_amt.unwrap(),
            }),
            'F' => Ok(Forwards { amount }),
            _ => unreachable!(),
        }
    }
}

type Position = (isize, isize);

// instructions: Vec<ShipInstruction>,

struct Ship {
    pos: Position,
    facing: isize,
}

impl Ship {
    fn move_in_direction(&self, dir: Direction, (x, y): Position, amount: isize) -> Position {
        use Direction::*;
        let (mut m_x, mut m_y) = (x, y);
        match dir {
            North => m_y += amount,
            South => m_y -= amount,
            East => m_x += amount,
            West => m_x -= amount,
        };
        (m_x, m_y)
    }

    fn advance(&mut self, instruction: &ShipInstruction) {
        use Direction::*;

        use RotationAmount::*;
        use ShipInstruction::*;

        let directions = [North, East, South, West];

        match instruction {
            Movement { dir, amount } => {
                self.pos = self.move_in_direction(*dir, self.pos, *amount);
            }
            Rotation { dir, amount } => {
                let offset_direction: isize = match dir {
                    RotationDir::Left => -1,
                    RotationDir::Right => 1,
                };

                let offset_amount: isize = match amount {
                    Quarter => 1,
                    Half => 2,
                    ThreeQuarters => 3,
                };

                self.facing += offset_direction * offset_amount;
                self.facing = ((self.facing as usize) % 4) as isize;
            }
            Forwards { amount } => {
                let curr_dir = directions[self.facing as usize];
                self.pos = self.move_in_direction(curr_dir, self.pos, *amount);
            }
        };
    }
}

struct NavigationInfo {
    ship_position: (isize, isize),
    waypoint_position: (isize, isize), // Offset to the ship
}

impl NavigationInfo {
    fn move_in_direction(&self, dir: Direction, (x, y): Position, amount: isize) -> Position {
        use Direction::*;
        let (mut m_x, mut m_y) = (x, y);
        match dir {
            North => m_y += amount,
            South => m_y -= amount,
            East => m_x += amount,
            West => m_x -= amount,
        };
        (m_x, m_y)
    }

    fn advance(&mut self, instruction: &ShipInstruction) {
        use RotationAmount::*;
        use ShipInstruction::*;

        match instruction {
            // Waypoint moves.
            Movement { dir, amount } => {
                self.waypoint_position =
                    self.move_in_direction(*dir, self.waypoint_position, *amount);
            }
            // Rotate waypoint.
            Rotation { dir, amount } => {
                let wp = self.waypoint_position;
                let new_point = [
                    (wp.0, wp.1),   // 0 deg.
                    (wp.1, -wp.0),  // 90 deg.
                    (-wp.0, -wp.1), // 180 deg.
                    (-wp.1, wp.0),
                ]; // 270 deg.

                let index = match dir {
                    RotationDir::Left => match amount {
                        Quarter => 3,
                        Half => 2,
                        ThreeQuarters => 1,
                    },
                    RotationDir::Right => match amount {
                        Quarter => 1,
                        Half => 2,
                        ThreeQuarters => 3,
                    },
                };

                self.waypoint_position = new_point[index];
            }
            Forwards { amount: times } => {
                let (x_offset, y_offset) = (
                    self.waypoint_position.0 * times,
                    self.waypoint_position.1 * times,
                );
                self.ship_position.0 += x_offset;
                self.ship_position.1 += y_offset;
            }
        };
    }
}

fn parse(s: &str) -> ShipInstruction {
    use ShipInstruction::*;

    let action = s.chars().next().unwrap();
    let amount: isize = s[1..].parse::<isize>().unwrap();

    let rotation_amt = match amount {
        90 => Some(RotationAmount::Quarter),
        180 => Some(RotationAmount::Half),
        270 => Some(RotationAmount::ThreeQuarters),
        _ => None,
    };

    match action {
        'N' => Movement {
            dir: Direction::North,
            amount,
        },
        'S' => Movement {
            dir: Direction::South,
            amount,
        },
        'E' => Movement {
            dir: Direction::East,
            amount,
        },
        'W' => Movement {
            dir: Direction::West,
            amount,
        },
        'L' => Rotation {
            dir: RotationDir::Left,
            amount: rotation_amt.unwrap(),
        },
        'R' => Rotation {
            dir: RotationDir::Right,
            amount: rotation_amt.unwrap(),
        },
        'F' => Forwards { amount },
        _ => unreachable!(),
    }
}

pub fn setup() -> Vec<ShipInstruction> {
    file_to_vec_transform("src/bigboy/12_bigboy.txt", parse)
}

pub fn silver(data: &[ShipInstruction]) {
    let mut start = Ship {
        facing: 1,
        pos: (0, 0),
    };
    data.iter().for_each(|x| start.advance(x));
    // println!("Manhattan Distance: {:?} | {}", start.pos, start.manhattan_distance());
}

pub fn gold(data: &[ShipInstruction]) {
    let mut start = NavigationInfo {
        ship_position: (0, 0),
        waypoint_position: (10, 1),
    };
    data.iter().for_each(|x| start.advance(x));
    // println!("Manhattan Distance: {:?} | {}", start.ship_position, start.manhattan_distance());
}

pub fn silver_gold_iter() {
    let data = file_parse_lazy::<ShipInstruction>("src/bigboy/12_bigboy.txt");

    let mut ship_silver = Ship {
        facing: 1,
        pos: (0, 0),
    };

    let mut ship_wp_gold = NavigationInfo {
        ship_position: (0, 0),
        waypoint_position: (10, 1),
    };

    data.for_each(|x| {
        ship_silver.advance(&x);
        ship_wp_gold.advance(&x);
    });
}

pub fn day_12_soln() {
    let data = setup();
    silver(&data);
    gold(&data);
}
