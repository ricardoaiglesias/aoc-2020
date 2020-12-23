use crate::helper::file_to_vec_transform;
use std::fmt;

#[derive(Copy, Clone, PartialEq)]
enum Cell {
    Floor,
    Empty,
    Occupied,
}

impl fmt::Debug for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Cell::Floor => write!(f, "."),
            Cell::Empty => write!(f, "L"),
            Cell::Occupied => write!(f, "#"),
        }
    }
}

struct Grid<'a> {
    data: &'a [Vec<Cell>],
    n_rows: usize,
    n_cols: usize,
}

// Utility functions for working in a grid.
impl<'a> Grid<'a> {
    fn in_bounds(&self, (row, col): (Row, Col)) -> bool {
        row < self.n_rows && col < self.n_cols
    }

    fn offset(&self, (row, col): (Row, Col), (dy, dx): (isize, isize)) -> Option<(Row, Col)> {
        if (row == 0 && dy == -1) || (col == 0 && dx == -1) {
            return None;
        }

        let (new_row, new_col) = ((row as isize + dy) as usize, (col as isize + dx) as usize);

        if !self.in_bounds((new_row, new_col)) {
            return None;
        }
        Some((new_row, new_col))
    }
}

// cardinal and diagonal directions as (dy, dx) pairs.
// Remember that dy = 1 is below, dy = -1 is above.
const DIRECTIONS: [(isize, isize); 8] = [
    // Diagonals:
    (-1, -1), // Top Left
    (-1, 1),  // Top Right
    (1, 1),   // Bottom Right
    (1, -1),  // Bottom left
    // Cardinal Directions:
    (0, 1),  // Right.
    (0, -1), // Left
    (1, 0),  // Down
    (-1, 0), // Up
];

type Row = usize;
type Col = usize;

/// Implementation of an iterator that returns the valid immediately adjacent
/// neighbors of an index.

/// Note: Implementing it this way allows me to separate the logic of "traverse
/// these array elements" from that of "What do I do with these values?"
struct GridAdjacentIterator<'v> {
    grid: &'v Grid<'v>,
    index: (Row, Col),
    direction_index: usize,
}

impl<'v> Iterator for GridAdjacentIterator<'v> {
    type Item = Cell;
    fn next(&mut self) -> Option<Cell> {
        let n_dirs = DIRECTIONS.len();

        loop {
            if self.direction_index == n_dirs {
                break;
            }
            let direction = DIRECTIONS[self.direction_index];

            // The "offset" function makes the code a _LOT_ cleaner.
            let new_index_opt = self.grid.offset(self.index, direction);
            if new_index_opt.is_none() {
                self.direction_index += 1;
                continue;
            }

            let new_index = new_index_opt.unwrap();

            if self.grid.in_bounds(new_index) {
                self.direction_index += 1;
                return Some(self.grid.data[new_index.0][new_index.1]);
            }

            self.direction_index += 1;
        }

        None
    }
}

fn grid_adj_iter_gen<'g>(grid: &'g Grid<'g>, index: (Row, Col)) -> GridAdjacentIterator {
    GridAdjacentIterator {
        grid,
        index,
        direction_index: 0,
    }
}

/** SILVER IMPLEMENTATION STARTS HERE. **/
fn get_num_occupied_silver(grid: &[Vec<Cell>], index: (Row, Col)) -> usize {
    let grid_struct = Grid {
        data: grid,
        n_rows: grid.len(),
        n_cols: grid[0].len(),
    };
    let iter: GridAdjacentIterator = grid_adj_iter_gen(&grid_struct, index);
    iter.map(|value| match value {
        Cell::Occupied => 1,
        _ => 0,
    })
    .sum()
}

fn rules_silver(grid: &[Vec<Cell>], index: (Row, Col)) -> Cell {
    let state = grid[index.0][index.1];
    let n_occupied = get_num_occupied_silver(grid, index);
    if state == Cell::Empty && n_occupied == 0 {
        return Cell::Occupied;
    }
    if state == Cell::Occupied && n_occupied >= 4 {
        return Cell::Empty;
    }
    state
}

fn get_num_occupied(grid: &[Vec<Cell>]) -> usize {
    grid.iter().fold(0 as usize, |sum: usize, vec: &Vec<Cell>| {
        let row_sum: usize = vec
            .iter()
            .map(|x| match x {
                Cell::Occupied => 1 as usize,
                _ => 0 as usize,
            })
            .sum();
        sum + row_sum
    })
}

pub fn silver() {
    let mut data: Vec<Vec<Cell>> = file_to_vec_transform("src/11_input.txt", parse);
    let mut next: Vec<Vec<Cell>> = advance(&data, rules_silver);

    while data != next {
        data = next;
        next = advance(&data, rules_silver);
    }

    // Now, calculate the number of occupied seats.
    let value = get_num_occupied(&data);
    println!("Value: {}", value);
}

/*
 * GOLD IMPLEMENTATION BELOW:
 */
struct GridLineIterator<'v> {
    grid: &'v Grid<'v>,
    index: (Row, Col),
    direction_index: usize,
}

impl<'v> Iterator for GridLineIterator<'v> {
    type Item = Cell;

    fn next(&mut self) -> Option<Cell> {
        let n_dirs = DIRECTIONS.len();
        if self.direction_index == n_dirs {
            return None;
        }

        let dir = DIRECTIONS[self.direction_index];

        let mut next_index = self.grid.offset(self.index, dir);
        while next_index.is_some() {
            let ind = next_index.unwrap();

            let value = self.grid.data[ind.0][ind.1];
            if value != Cell::Floor {
                self.direction_index += 1;
                return Some(value);
            }

            next_index = self.grid.offset(ind, dir);
        }

        self.direction_index += 1;
        Some(Cell::Floor)
    }
}

fn get_num_occupied_line(grid: &[Vec<Cell>], index: (Row, Col)) -> usize {
    // Create an iterator.
    let iter = GridLineIterator {
        grid: &Grid {
            data: &grid,
            n_rows: grid.len(),
            n_cols: grid[0].len(),
        },
        index,
        direction_index: 0,
    };

    iter.map(|opt| match opt {
        Cell::Occupied => 1,
        _ => 0,
    })
    .sum()
}

fn rules_gold(grid: &[Vec<Cell>], index: (Row, Col)) -> Cell {
    let state = grid[index.0][index.1];
    let n_occupied = get_num_occupied_line(grid, index);

    if state == Cell::Empty && n_occupied == 0 {
        return Cell::Occupied;
    }
    if state == Cell::Occupied && n_occupied >= 5 {
        return Cell::Empty;
    }

    state
}

pub fn gold() {
    let mut data: Vec<Vec<Cell>> = file_to_vec_transform("src/11_input.txt", parse);
    let mut next: Vec<Vec<Cell>> = advance(&data, rules_gold);

    while data != next {
        data = next;
        next = advance(&data, rules_gold);
    }

    // Now, calculate the number of occupied seats.
    let value = get_num_occupied(&data);
    println!("Value: {}", value);
}

fn parse(s: &str) -> Vec<Cell> {
    s.chars()
        .map(|c| match c {
            'L' => Cell::Empty,
            '.' => Cell::Floor,
            '#' => Cell::Occupied,
            _ => panic!("Fuck"),
        })
        .collect()
}

pub fn setup() {}

// Generic function that goes through a 2D grid and applies rule_fn to every
// element.
type RuleFunction = fn(&[Vec<Cell>], (Row, Col)) -> Cell;
fn advance(curr_state: &[Vec<Cell>], rule_fn: RuleFunction) -> Vec<Vec<Cell>> {
    curr_state
        .iter()
        .enumerate()
        .map(|(row, vec)| {
            vec.iter()
                .enumerate()
                .map(|(col, _state)| rule_fn(&curr_state, (row, col)))
                .collect()
        })
        .collect()
}

pub fn day_11_soln() {
    silver();
    gold();
}
