use getch::Getch;
use std::fs::File;
use std::io::Read;

pub fn read_file(filename: &str) -> Vec<char> {
    let mut file = File::open(filename).unwrap();

    let mut contents: Vec<char> = Vec::new();

    // Read the file character by character
    let mut buffer = [0; 1];
    while let Ok(_) = file.read_exact(&mut buffer) {
        contents.push(buffer[0] as char);
    }

    contents
}

pub fn read_single_char() -> u8 {
    let getch = Getch::new();
    let result = getch.getch();
    match result {
        Ok(key) => key,
        Err(_) => 0,
    }
}
