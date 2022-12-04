use std::fs::File;
use std::io::{ BufReader, Lines, prelude::* };

pub fn read_file(file_name: &str) -> Lines<BufReader<File>> {
    let file_name = String::from(file_name);
    let file = File::open(file_name).expect("Can't read a file");
    BufReader::new(file).lines()
}
