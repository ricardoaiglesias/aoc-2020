use crate::helper::file_to_vec;

type History = Vec<bool>;

#[derive(PartialEq)]
enum InstructionType {
    NOP,
    ACC,
    JUMP
}

struct Instruction {
    i_type: InstructionType,
    argument: i64
}

pub struct Interpreter {
    instruction_ptr : i64,
    instructions: Vec<Instruction>,
    register: i64
}

fn dispatch_instruction(console: &mut Interpreter) {
    let instr: &mut Instruction = &mut console.instructions[console.instruction_ptr as usize];
    match instr.i_type {
        InstructionType::NOP => {
            console.instruction_ptr += 1; // Next instruction.
        },
       InstructionType::JUMP => {
            console.instruction_ptr += instr.argument;
        },
        InstructionType::ACC => {
            console.register += instr.argument;
            console.instruction_ptr += 1;
        }
    };
}

fn has_run_instruction(seen_instructions: &[bool], instr_ptr: i64)
                       -> bool {
    seen_instructions[instr_ptr as usize]
}

fn line_to_instruction(s: &str) -> Instruction {
    let split_loc = s.find(' ').unwrap();
    let instruction : &str = &s[0..split_loc];

    let arg_value : i64 = s[split_loc+1..].parse().unwrap();

    match instruction {
        "jmp" => Instruction {
            i_type : InstructionType::JUMP,
            argument: arg_value
        },
        "nop" => Instruction {
            i_type : InstructionType::NOP,
            argument: arg_value
        },
        "acc" => Instruction {
            i_type : InstructionType::ACC,
            argument: arg_value
        },
        _ => {
            println!("AGHF UCK! SOMETHING WENT WRONG.");
            Instruction {
            i_type : InstructionType::NOP,
            argument: arg_value
        }},
    }
}

fn run(console: &mut Interpreter) -> bool {
    let mut seen_instructions: History = vec![false; console.instructions.len()];

    while !has_run_instruction(&seen_instructions, console.instruction_ptr) {
        seen_instructions[console.instruction_ptr as usize] = true;
        dispatch_instruction(console);
        if console.instructions.len() == console.instruction_ptr as usize { return true; }
    }
    false
}

fn run_write_history(console: &mut Interpreter, executed_instrs: &mut History) -> bool {
    while !has_run_instruction(&executed_instrs, console.instruction_ptr) {
        executed_instrs[console.instruction_ptr as usize] = true;

        dispatch_instruction(console);
        if console.instructions.len() == console.instruction_ptr as usize { return true; }
    }
    false
}

fn modify_itype(itype : &InstructionType) -> InstructionType{
    match itype {
        InstructionType::JUMP => InstructionType::NOP,
        InstructionType::NOP => InstructionType::JUMP,
        _ => InstructionType::ACC
    }
}

fn reset_interpreter(console: &mut Interpreter) {
    console.instruction_ptr = 0;
    console.register =  0;
}

fn run_2(console: &mut Interpreter) {
    reset_interpreter(console);

    // For each instruction, if it's a jump, change to a nop and see if it
    // terminates. If it's a nop, change to a jump and see if it terminates.
    let n_instructions = console.instructions.len();
    let mut seen_instrs : History = vec![false; n_instructions];

    // Write history into vector. Though it's an extra loop before any useful
    // work is done, it tells us which instructions we need to worry about,
    // saving time in the long run.
    run_write_history(console, &mut seen_instrs);

    for (i, viewed_instruction) in seen_instrs.iter().enumerate() {
        if !viewed_instruction ||
            console.instructions[i].i_type == InstructionType::ACC {
            continue;
        }

        reset_interpreter(console);

        // Use blocks to limit the scope of this mutable borrow.
        // Without this, we would have two borrows on a part of the "console"
        // structure, which breaks Rust's reference rules.
        {
            let changed_instruction: &mut Instruction = &mut console.instructions[i];
            changed_instruction.i_type = modify_itype(&changed_instruction.i_type);
        }

        // Run and see if it terminates. 
        if run(console) {
            break;
        }

        // Change back.
        {
            let changed_instruction: &mut Instruction = &mut console.instructions[i];
            changed_instruction.i_type = modify_itype(&changed_instruction.i_type);
        }
    }
}

pub fn silver(game_console: &mut Interpreter) {
    reset_interpreter(game_console);
    run(game_console);
}

pub fn gold(game_console: &mut Interpreter) {
    run_2(game_console);
}

pub fn setup() -> Interpreter {
    let vec : Vec<String> = file_to_vec("src/8_input.txt").unwrap();
    let vec_iter = vec.iter();
    let rhs = vec_iter.clone().map( |s| line_to_instruction(s)).collect();

    Interpreter {
        instruction_ptr: 0,
        register: 0,
        instructions: rhs
    }
}

pub fn day_8_soln_bench() {
    let mut game_console = setup();

    silver(&mut game_console);
    gold(&mut game_console);
}

pub fn day_8_soln () {
    let mut game_console = setup();

    silver(&mut game_console);
    println!("(Silver): Value of register: {}", game_console.register);

    gold(&mut game_console);
    println!("(Gold): Value of register: {}", game_console.register);
}
