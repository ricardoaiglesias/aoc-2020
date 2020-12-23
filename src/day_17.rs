use crate::helper::*;
use std::collections::{HashMap, HashSet};


// type OffsetCollection = [(isize, isize, isize); NUM_NEIGHBORS];

const NUM_NEIGHBORS_SILVER: usize = 26; // Excluding self.
const NUM_NEIGHBORS_GOLD: usize = 80; // Excluding self.

type OffsetArrayT<T> = Vec<T>;

type Point = Vec<isize>;

enum Part { Gold, Silver }

struct NeighborInfo {
    offsets: Vec<Point>,
    num_neighbors: usize
}

/// T is either GoldLocation or SilverLocation.
struct CubeGrid {
    grid: HashMap<Point, bool>,
    offsets: NeighborInfo
}

#[derive (PartialEq, Eq, Hash, Debug, Copy, Clone)]
struct Location { x: isize, y: isize, z: isize }


struct LocationIter<'a, 'b> {
    // grid: &'a CubeGrid<T>,
    grid: &'a CubeGrid,
    current: &'b Point,
    offset_index: usize,
}

impl <'a, 'b> Iterator for LocationIter<'a, 'b> {
    type Item = Point;

    fn next(&mut self) -> Option<Point> {
        if self.offset_index >= self.grid.offsets.num_neighbors { return None; }
        let offsets = &self.grid.offsets.offsets[self.offset_index];

        let offset_location: Point = self.current.iter().enumerate().map(
            |(index, position)| position + offsets[index]).collect();

        self.offset_index += 1;
        Some(offset_location)
    }
}

impl CubeGrid{
    // Returns an iterator over the neighbors that _are_ on the grid.
    fn get_num_active_neighbors(&self, loc: &Point)
                                -> usize {
        let iterator = LocationIter{ grid: &self, current: loc, offset_index: 0};
        iterator.map( |neighbor| {
            let access = self.grid.get(&neighbor);
            match access {
                None => false,
                Some(is_active) => *is_active
            }
        }).filter(|v| *v).count()
    }

    fn update(&mut self) {
        // Set that contains all the current elements and all their neighbors.
        let mut to_check: HashSet<Point> = HashSet::new();
        self.grid.iter().for_each(|(location, _active)| {
            to_check.insert(location.clone());
            let iter = LocationIter{grid: &self, current: location, offset_index: 0};
            iter.for_each(|neighbor| {to_check.insert(neighbor);} )
        });

        // Update map.
        let mut new_map : HashMap<Point, bool> = HashMap::new();
        for location in to_check {
            let num_neighboring = self.get_num_active_neighbors(&location);

            let current_value = self.grid.get(&location).unwrap_or(&false);

            let new_value = match (current_value, num_neighboring) {
                (true, 2) => true,
                (true, 3) => true, // Active and 2-3 neighbors are active.
                (false, 3) => true, // Inactive but three of its neighbors are active.
                _ => false
            };

            new_map.insert(location, new_value);
        }
        self.grid = new_map;
    }
}

fn init_silver() -> CubeGrid{
    let mut offset_array : Vec<Point> = Vec::with_capacity(NUM_NEIGHBORS_SILVER);
    for dx in -1 as isize..=1 { for dy in -1 as isize..=1 { for dz in -1 as isize..=1 {
        if!(dx == 0 && dy == 0 && dz == 0) {
            let result : Vec<isize> =vec![dx, dy, dz];
            offset_array.push(result);
        }
    }}}

    CubeGrid {
        grid: HashMap::new(),
        offsets: NeighborInfo{
            offsets: offset_array,
            num_neighbors: NUM_NEIGHBORS_SILVER
        },

    }
}

pub fn silver(data: &[Vec<bool>]) {
    let mut grid = init_silver();

    for (row, vec) in data.iter().enumerate() {
        for (col, data) in vec.iter().enumerate() {
            let loc = vec![col as isize, row as isize, 0];
            grid.grid.insert(loc, *data);
        }
    }

    const N_ITER : usize = 6;
    for _ in 0..N_ITER {
        println!("Starting the update: ");
        grid.update();
        println!("{}", grid.grid.iter().filter(|(_key, value)| **value).count());
    }
}

fn init_gold() -> CubeGrid {
    let mut offset_array : Vec<Point> = Vec::with_capacity(NUM_NEIGHBORS_GOLD);
    for dx in -1 as isize..=1 { for dy in -1 as isize..=1 { for dz in -1 as isize..=1 {
        for dw in -1 as isize..=1 {
            if!(dx == 0 && dy == 0 && dz == 0 && dw == 0) {
                let result : Vec<isize> =vec![dx, dy, dz, dw];
                offset_array.push(result);
            }
        }
    }}}

    CubeGrid {
        grid: HashMap::new(),
        offsets: NeighborInfo{
            offsets: offset_array,
            num_neighbors: NUM_NEIGHBORS_GOLD
        }
    }
}
pub fn gold(data: &[Vec<bool>]) {
    let mut grid = init_gold();

    for (row, vec) in data.iter().enumerate() {
        for (col, data) in vec.iter().enumerate() {
            let loc = vec![col as isize, row as isize, 0, 0];
            grid.grid.insert(loc, *data);
        }
    }

    const N_ITER : usize = 6;
    for _ in 0..N_ITER {
        println!("Starting the update: ");
        grid.update();
        println!("{}", grid.grid.iter().filter(|(_key, value)| **value).count());
        println!("Size: {}", grid.grid.len());
    }
}

pub fn setup(filepath: &str) -> Vec<Vec<bool>>{
    file_to_vec_transform(filepath, |line| {
        let vec: Vec<bool> = line.chars().map(
            |c| match c {
                '#' => true,
                _ => false
            }).collect();
        vec
    })
}
pub fn day_17_soln() {
    let data = setup("src/17_input.txt");
    // silver(&data);
    gold(&data);
}
