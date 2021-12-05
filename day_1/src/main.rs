use std::collections::VecDeque;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::Index;

fn main() {
    count_diff();
    count_sliding();
}

// count the number of times there is an increase in a sliding average of three
fn count_sliding(){
    let filename = "src/data.txt";
    // Open the file in read-only mode (ignoring errors).
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut qu: VecDeque<i32> =  VecDeque::from([]);
    // Read the file line by line using the lines() iterator from std::io::BufRead.
    let mut count = 0;
    for  line in reader.lines(){
        let tmp = line.unwrap().parse::<i32>().unwrap();
        qu.push_back(tmp);
        if qu.len() > 3 {
            let first = qu.index(0) + qu.index(1) + qu.index(2);
            let second = qu.index(1) + qu.index(2) + qu.index(3);
            if first < second {
                count += 1;
            }
            qu.pop_front();
        }
    }
    println!("{}", count)

}

// Count the number of times there is an increase in the numbers
fn count_diff(){
    let filename = "src/data.txt";
    // Open the file in read-only mode (ignoring errors).
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut count : i32 = 0;
    let mut prev : i32 = 0;
    let mut first: bool = true;
    // Read the file line by line using the lines() iterator from std::io::BufRead.
    for line in reader.lines() {
        let tmp = line.unwrap().parse::<i32>().unwrap();
        if !first && prev < tmp {
            count += 1
        }
        prev = tmp;
        first = false;
    }
    println!("{}", count-1)

}
