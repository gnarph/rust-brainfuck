use std::env;
use std::collections::HashMap;

const INCREMENT_DATA_POINTER: char = '>';
const DECREMENT_DATA_POINTER: char = '<';
const INCREMENT_DATA: char = '+';
const DECREMENT_DATA: char = '-';
const OUTPUT_DATA: char = '.';
const READ_DATA: char = ',';
const JUMP_FORWARD: char = '[';
const JUMP_BACKWARD: char = ']';

pub struct VM {
    instructions: String,
    instruction_pointer: usize,
    data_pointer: usize,
    data: [u8; 3200],
    jump_map: HashMap<usize, usize>,
}

pub fn create_jump_map(instructions: String) -> HashMap<usize, usize>{
    let mut openings: Vec<usize> = Vec::new();
    let mut jump_map: HashMap<usize, usize> = HashMap::new();

    for (position, instruction) in instructions.chars().enumerate() {
        match instruction {
            JUMP_FORWARD => openings.push(position),
            JUMP_BACKWARD => {
                let forward_position = openings.pop().unwrap();
                let backward_position = position;
                jump_map.insert(forward_position, backward_position);
                jump_map.insert(backward_position, forward_position);
            },
            _ => (),
        }
    }
    jump_map
}

fn get_arg_one() -> String {
    // Can't just get the args as an array for some reason
    let mut args: Vec<String> = Vec::new();
    for argument in env::args_os() {
        match argument.into_string() {
            Ok(str_arg) => args.push(str_arg),
            Err(e) => println!("{:?}", e),
        }
    }
    args[1].clone()
}

fn main() {
    let instructions = get_arg_one();
    println!("Got: {}", instructions);
    let mut jump_map = create_jump_map(instructions.clone());
    println!("jumps: {:?}", jump_map);
    let mut vm = fuckvm::VM::new(instructions, jump_map);
    let mut instruction_pointer = 0;
    let mut data_pointer = 0;
    let mut data = [u8; 3200];
    let end = instructions.length();
}
