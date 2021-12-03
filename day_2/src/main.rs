use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    first();
    second();
}

fn second(){
    let filename = "src/data.txt";
    // Open the file in read-only mode (ignoring errors).
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut horizontal : i32 = 0;
    let mut aim : i32 = 0;
    let mut depth : i32 = 0;
    // Read the file line by line using the lines() iterator from std::io::BufRead.
    for line in reader.lines() {
        let tmp = line.unwrap();
        let split : Vec<&str> = tmp.split(" ").collect();
        let dist:i32 = split[1].parse().unwrap();
        match split[0]{
            "down"=> aim += dist,
            "up"=> aim -= dist,
            "forward"=> {
                horizontal += dist;
                depth += aim * dist;
            }
                ,
            _=>print!("Whoops")
        }

    }
    println!("{}", horizontal * depth)
}


fn first(){
    let filename = "src/data.txt";
    // Open the file in read-only mode (ignoring errors).
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut horizontal : i32 = 0;
    let mut depth : i32 = 0;
    // Read the file line by line using the lines() iterator from std::io::BufRead.
    for line in reader.lines() {
        let tmp = line.unwrap();
        let split : Vec<&str> = tmp.split(" ").collect();
        let dist:i32 = split[1].parse().unwrap();
        match split[0]{
            "down"=> depth += dist,
            "up"=> depth -= dist,
            "forward"=> horizontal += dist,
            _=>print!("Whoops")
        }

    }
    println!("{}", horizontal * depth)

}