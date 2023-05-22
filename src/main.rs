#![allow(dead_code)]
use std::collections::HashMap;

use getch::Getch;

fn main() {
    let code = "This is some sample text".chars();
    let code = Vec::from_iter(code.into_iter());
    System::new().run(code);
}

fn read_single_char() -> u8 {
    let getch = Getch::new();
    let result = getch.getch();
    match result {
        Ok(key) => key,
        Err(_) => 0,
    }
}

struct System {
    memory: [u8; 1024],
    pc: usize,
    bracemap: HashMap<usize, usize>,
}

impl System {
    fn new() -> Self {
        Self {
            memory: [0; 1024],
            pc: 0,
            bracemap: HashMap::new(),
        }
    }

    fn run(mut self: Self, instructions: Vec<char>) -> () {
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
                    print!("{}", self.memory[self.pc]);
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
