use std::fs;

fn main() {
    let filename = "src/data/datamed.txt";
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let split_new_line: Vec<&str> = contents.split("\n").collect();

    for line in split_new_line {
        
    }