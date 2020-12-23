use crate::helper::file_to_vec;
use std::collections::HashMap;

type Ones = usize;
type Zeroes = usize;

#[derive(Copy, Clone)]
pub struct MaskValue {
    on: Ones,
    off: Zeroes,
    floating: Ones,
}

pub enum Instruction {
    Mask { arg: MaskValue },
    Assign { location: usize, arg: usize },
}

struct Program {
    memory: HashMap<usize, usize>,
    mask: MaskValue,
}

fn get_all_writable_memory(program: &mut Program, location: usize) -> Vec<usize> {
    let mut floating: usize = program.mask.floating;
    let mut index = 0;

    let mut vec_prev: Vec<usize> = Vec::with_capacity(1 << 11);
    let mut vec_next: Vec<usize> = Vec::with_capacity(1 << 11);

    vec_prev.push(location);
    // Add the values for the curr
    while floating > 0 {
        if vec_prev.is_empty() {
            panic!();
        }

        let flag = 1 << index;

        let is_floating = floating & flag;
        if is_floating != 0 {
            vec_prev.iter().for_each(|v| {
                let on = flag | v;
                let off = !flag & v;

                vec_next.push(off); // Off
                vec_next.push(on); // On
            });

            vec_prev = vec_next.clone();
            vec_next.clear();
        }

        floating &= !flag;
        index += 1;
    }

    // Now, create the memory addresses we will write to:
    let ones = program.mask.on;
    vec_prev.into_iter().map(|mem| mem | ones).collect()
}

impl Program {
    fn dispatch(&mut self, instr: &Instruction) {
        use Instruction::*;
        match instr {
            Mask { arg } => self.mask = *arg,
            Assign { location, arg } => {
                // let fall_through = (arg & !self.mask.off) | self.mask.on;
                // (arg & self.mask.off) | (arg self.mask.on) ;
                let pass = self.mask.floating & arg;
                let ones = self.mask.on;
                let zeros = self.mask.off & arg;

                let value = ones | (pass & zeros);
                self.memory.insert(*location, value);
            }
        }
    }

    fn dispatch_gold(&mut self, instr: &Instruction) {
        use Instruction::*;
        match instr {
            Mask { arg } => self.mask = *arg,
            Assign { location, arg } => {
                let mem_locations = get_all_writable_memory(self, *location);
                mem_locations.iter().for_each(|mem| {
                    self.memory.insert(*mem, *arg);
                });
            }
        };
    }
}

fn parse(s: &str) -> Instruction {
    enum BitValue {
        One,
        Zero,
        Invalid,
    };
    use BitValue::*;

    let tokens: Vec<&str> = s.split(" = ").collect();
    let rhs: &str = tokens[1];

    if s.find("mask").is_some() {
        let mut ones = 0;
        let mut zeroes = usize::MAX;
        let mut pass = 0;

        let bits = rhs
            .chars()
            .map(|c| match c {
                '1' => One,
                '0' => Zero,
                _ => Invalid,
            })
            .rev();
        bits.enumerate().for_each(|(index, b)| match b {
            One => {
                ones |= 1 << index;
            }
            Zero => {
                zeroes &= !(1 << index);
            }
            Invalid => {
                pass |= 1 << index;
            }
        });

        return Instruction::Mask {
            arg: MaskValue {
                on: ones,
                off: zeroes,
                floating: pass,
            },
        };
    }

    if s.find("mem").is_some() {
        let location_idx = s.find('[').unwrap();
        let location_idx_end = s.find(']').unwrap();

        let location: usize = s[location_idx + 1..location_idx_end].parse().unwrap();
        let arg: usize = rhs.parse().unwrap();

        return Instruction::Assign { location, arg };
    }

    unreachable!();
}

pub fn silver(data: &[Instruction]) {
    let mut program = Program {
        mask: MaskValue {
            on: 0,
            off: 0,
            floating: 0,
        },
        memory: HashMap::new(),
    };
    data.iter().for_each(|i| program.dispatch(i));

    let result = program
        .memory
        .iter()
        .fold(0, |sum, (_location, value)| value + sum);
    println!("Result: {}", result);
}

pub fn gold(data: &[Instruction]) -> usize {
    let mut program = Program {
        mask: MaskValue {
            on: 0,
            off: 0,
            floating: 0,
        },
        memory: HashMap::new(),
    };
    data.iter().for_each(|i| program.dispatch_gold(i));

    let result = program
        .memory
        .iter()
        .fold(0, |sum, (_location, value)| value + sum);
    println!("Result: {}", result);
    result
}

pub fn setup(filepath: &str) -> Vec<Instruction> {
    file_to_vec(filepath)
        .unwrap()
        .iter()
        .map(|s| parse(s))
        .collect()
}

// fn parse_test(s: &str) { println!("Parse Test: {}", s.parse::<usize>().unwrap()); }

pub fn day_14_soln() {
    let instructions = setup("src/14_input.txt");
    assert!(gold(&instructions) == 3434009980379);
}
