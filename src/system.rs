use std::collections::HashMap;

use super::read_single_char;

pub struct System {
    memory: [u8; 1024],
    pc: usize,
    bracemap: HashMap<usize, usize>,
}

impl System {
    pub fn new() -> Self {
        Self {
            memory: [0; 1024],
            pc: 0,
            bracemap: HashMap::new(),
        }
    }

    pub fn run(mut self: Self, instructions: Vec<char>) -> () {
        self.generate_bracemap(&instructions);
        let mut ptr = 0;
        while ptr < instructions.len() {
            match instructions[ptr] {
                '>' => {
                    self.pc += 1;
                }
                '<' => {
                    self.pc -= 1;
                }
                '+' => {
                    self.memory[self.pc] += 1;
                }
                '-' => {
                    self.memory[self.pc] -= 1;
                }
                '[' => {
                    if self.memory[self.pc] == 0 {
                        ptr = self.bracemap.get(&ptr).copied().unwrap();
                    }
                }
                ']' => {
                    if self.memory[self.pc] != 0 {
                        ptr = self.bracemap.get(&ptr).copied().unwrap();
                    }
                }
                '.' => {
                    print!("{}", self.memory[self.pc] as char);
                }
                ',' => {
                    self.memory[self.pc] = read_single_char();
                }
                _ => (),
            }

            ptr += 1;
        }
    }

    fn generate_bracemap(self: &mut Self, instructions: &Vec<char>) -> () {
        let mut stack: Vec<usize> = Vec::new();
        for i in 0..instructions.len() {
            if instructions[i] == '[' {
                stack.push(i);
            } else if instructions[i] == ']' {
                let start_index = stack.pop().unwrap();
                self.bracemap.insert(i, start_index);
                self.bracemap.insert(start_index, i);
            };
        }
    }
}
