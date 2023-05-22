use std::env;

use input::*;
use system::*;

mod input;
mod system;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Please provide a filename !");
        return;
    }

    let filename = &args[1];
    let program = read_file(filename);

    System::new().run(program);

    println!();
}
