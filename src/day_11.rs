use crate::helper::file_to_vec_transform;
use std::fmt;

#[derive(Copy, Clone, PartialEq)]
enum Cell {
    Floor,
    Empty,
    Occupied
}

impl fmt::Debug for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Cell::Floor => write!(f, "."),
            Cell::Empty => write!(f, "L"),
            Cell::Occupied => write!(f, "#")
        }
    }
}

struct Grid<'a> {
    data: &'a Vec<Vec<Cell>>,
    n_rows: usize,
    n_cols: usize
}

impl<'a> Grid<'a> {
    fn in_bounds(&self, index: (Row, Col)) -> bool{
        index.0 < self.n_rows && index.1 < self.n_cols
    }
}

const DIRECTIONS : [(isize, isize); 8] = [
    // Diagonals:
    (-1, -1), // Top Left
    (-1, 1), // Top Right
    (1, 1), // Bottom Right
    (1, -1), // Bottom left

    // Cardinal Directions:
    (0, 1), // Right.
    (0, -1), // Left
    (1, 0), // Down
    (-1, 0) // Up
];


type Row = usize;
type Col = usize;

struct GridAdjacentIterator<'v> {
    grid: &'v Grid<'v>,
    index: (Row, Col),
    direction_index : usize
}

impl<'v> Iterator  for GridAdjacentIterator<'v> {
    type Item = Cell;
    fn next(&mut self) -> Option<Cell> {
        let n_dirs = DIRECTIONS.len();

        loop {
            if self.direction_index == n_dirs {
                break;
            }
            let direction = DIRECTIONS[self.direction_index];

            // Top Row.
            if self.index.0 == 0 && direction.0 == -1 {
                self.direction_index += 1;
                continue;
            }

            // Left column.
            if self.index.1 == 0 && direction.1 == -1 {
                self.direction_index += 1;
                continue;
            }

            let new_index =
                ((self.index.0 as isize + direction.0) as usize,
                 (self.index.1 as isize + direction.1) as usize);
            if self.grid.in_bounds(new_index){
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
        grid, index, direction_index: 0
    }
}

fn get_num_occupied_silver(grid: &Vec<Vec<Cell>>, index: (Row, Col)) -> usize {
    let grid_struct = Grid { data: grid, n_rows: grid.len(), n_cols: grid[0].len()};
    let iter : GridAdjacentIterator = grid_adj_iter_gen(&grid_struct, index);
    iter.map(|value| match value
             { Cell::Occupied => 1, _ => 0} ).sum()
}

fn rules_silver(grid: &Vec<Vec<Cell>> , index: (Row, Col)) -> Cell{
    let state = &grid[index.0 as usize][index.1 as usize];
    match state {
        Cell::Empty =>
        {
            if get_num_occupied_silver(grid, index) == 0 { Cell::Occupied}
            else { Cell::Empty }
        }
        Cell::Occupied =>
        {
            if get_num_occupied_silver(grid, index) >= 4 { Cell::Empty }
            else { Cell::Occupied }
        },
        Cell::Floor => *state
    }
}

fn parse(s: &str) -> Vec<Cell> {
    s.chars().map(|c| match c {
        'L' => Cell::Empty,
        '.' => Cell::Floor,
        '#' => Cell::Occupied,
        _=> panic!("Fuck")
    }).collect()
}

type RuleFunction = fn(&Vec<Vec<Cell>>, (Row, Col)) -> Cell;

fn advance(curr_state: &Vec<Vec<Cell>>, rule_fn: RuleFunction) -> Vec<Vec<Cell>> {
    curr_state.iter().enumerate().map
        (|(row, vec)| vec.iter().enumerate().map(
            |(col, _state)| rule_fn(&curr_state, (row, col))
        ).collect()).collect()
}

fn get_num_occupied(grid: &Vec<Vec<Cell>>) -> usize {
    grid.iter().fold(0 as usize, |sum : usize, vec : &Vec<Cell>|{
            let row_sum : usize =
                vec.iter().map(
                    |x| match x {
                        Cell::Occupied => 1 as usize,
                        _ => 0 as usize
                    }).sum();
            sum + row_sum
        })
}

pub fn silver() 
{
    let mut data : Vec<Vec<Cell>> = file_to_vec_transform("src/11_input.txt", parse);
    let mut next : Vec<Vec<Cell>> = advance(&data, rules_silver);

    while data != next {
        data = next;
        next = advance(&data, rules_silver);
    }

    // Now, calculate the number of occupied seats. 
    let value = get_num_occupied(&data);
    println!("Value: {}", value);
}

// (dy, dx) tuple.
type Direction = (isize, isize);

fn in_bounds(n_rows: usize, n_cols: usize, index: (Row, Col)) -> bool {
    (index.0 as usize) < n_rows && (index.1 as usize)  < n_cols
}

fn does_direction_have_seat(grid: &Vec<Vec<Cell>>, index: (Row, Col), dir: Direction) -> bool {
    let (mut cur_row , mut cur_col) = (index.0, index.1);

    let n_rows = grid.len();
    let n_cols = grid[0].len();

    if (cur_row == 0 && dir.0 == -1) || (cur_col == 0 && dir.1 == -1) { return false; }

    cur_row = (cur_row as isize + dir.0) as usize;
    cur_col = (cur_col as isize + dir.1) as usize;
    
    while in_bounds(n_rows, n_cols, (cur_row, cur_col)) {

        let state = grid[cur_row as usize][cur_col as usize];

        if state != Cell::Floor {
            return state == Cell::Occupied ;
        }
        if cur_row == 0 && dir.0 == -1 || cur_col == 0 && dir.1 == -1 { return false; }

        cur_row = (cur_row as isize + dir.0) as usize;
        cur_col = (cur_col as isize + dir.1) as usize;
    }
    false
}

fn get_num_adjacent_gold(grid: &Vec<Vec<Cell>>, index: (Row, Col)) -> usize{
    // Calculate number of seats
    DIRECTIONS.iter().map(|dir| match does_direction_have_seat(grid, index, *dir) {
        true => 1,
        false => 0
    }).sum()
}

fn rules_gold(grid: &Vec<Vec<Cell>>, index: (Row, Col)) -> Cell{
    let state = grid[index.0 as usize][index.1 as usize];
    match state {
        Cell::Empty =>
        {
            if get_num_adjacent_gold(grid, index) == 0 { Cell::Occupied}
            else { Cell::Empty }
        }
        Cell::Occupied =>
        {
            if get_num_adjacent_gold(grid, index) >= 5 { Cell::Empty }
            else { Cell::Occupied }
        },
        Cell::Floor => state
    }
}

pub fn gold() {
    let mut data : Vec<Vec<Cell>> = file_to_vec_transform("src/11_input.txt", parse);
    let mut next : Vec<Vec<Cell>> = advance(&data, rules_gold);

    while data != next {
        data = next;
        next = advance(&data, rules_gold);
    }

    // Now, calculate the number of occupied seats.
    let value = get_num_occupied(&data);
    println!("Value: {}", value);
}

pub fn setup(){  }

pub fn day_11_soln() {
    silver();
    gold();
}
