use std::collections::HashMap;

const INCREMENT_DATA_POINTER: char = '>';
const DECREMENT_DATA_POINTER: char = '<';
const INCREMENT_DATA: char = '+';
const DECREMENT_DATA: char = '-';
const OUTPUT_DATA: char = '.';
const READ_DATA: char = ',';
const JUMP_FORWARD: char = '[';
const JUMP_BACKWARD: char = ']';

fn create_jump_map(instructions: &String) -> HashMap<usize, usize>{
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

struct VM<'a> {
    jump_map: &'a HashMap<usize, usize>,
    instruction_pointer: &'a mut usize,
    data_pointer: &'a mut usize,
    data: &'a mut [u8; 3200],
    byte_ins: &'a Vec<u8>,
}

impl<'a> VM<'a> {
    pub fn fetch(&self) -> u8 {
        self.byte_ins[*self.instruction_pointer]
    }

    pub fn increment_data_pointer(&mut self) {
        *self.data_pointer += 1;
    }

    pub fn decrement_data_pointer(&mut self) {
        *self.data_pointer -= 1;
    }

    fn increment_data(&mut self) {
        self.data[*self.data_pointer] += 1;
    }

    fn decrement_data(&mut self) {
        self.data[*self.data_pointer] -= 1;
    }

    fn increment_instruction_pointer(&mut self) {
        *self.instruction_pointer += 1;
    }

    fn get_data(&self) -> u8 {
        self.data[*self.data_pointer]
    }

    fn jump(&mut self) {
        let destination = self.jump_map.get(self.instruction_pointer);
        *self.instruction_pointer = *destination.unwrap();
    }

    fn jump_forward(&mut self) {
        if self.get_data() == 0 {
            self.jump();
        }
    }

    fn jump_backward(&mut self) {
        if self.get_data() != 0 {
            self.jump();
        }
    }

        fn done(&self) -> bool {
        *self.instruction_pointer >= self.byte_ins.len()
    }
}

pub fn execute(instructions: String) {
    let jump_map = create_jump_map(&instructions);
    let mut instruction_pointer: usize = 0;
    let mut data_pointer = 0;
    let mut data = [0u8; 3200];
    let byte_ins = instructions.into_bytes();

    let mut vm = VM {
        jump_map: &jump_map,
        instruction_pointer: &mut instruction_pointer,
        data_pointer: &mut data_pointer,
        data: &mut data,
        byte_ins: &byte_ins
    };

    while !vm.done() {
        let instruction = vm.fetch();
        match instruction as char {
            INCREMENT_DATA_POINTER => {
                vm.increment_data_pointer()
            },
            DECREMENT_DATA_POINTER => {
                vm.decrement_data_pointer()
            },
            INCREMENT_DATA => {
                vm.increment_data()
            },
            DECREMENT_DATA => {
                vm.decrement_data()
            },
            JUMP_FORWARD => {
                vm.jump_forward()
            },
            JUMP_BACKWARD => {
                vm.jump_backward()
            },
            READ_DATA => {
                println!("Reading data not supported yet");
            },
            OUTPUT_DATA => {
                print!("{}", vm.get_data() as char);
            },
            _ => (),
        }
        vm.increment_instruction_pointer();
    }
}
