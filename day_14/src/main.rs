use std::collections::BTreeMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::{SystemTime};

fn main() {
    let filename = "src/data/data.txt";
    // Open the file in read-only mode (ignoring errors).
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut flip_bool = false;
    let mut initial_template: Vec<char> = Vec::new();
    let mut instructions: Vec<(char, char, char)> = Vec::new();
    // Read the file line by line using the lines() iterator from std::io::BufRead.
    for line in reader.lines() {
        let tmp = line.unwrap();
        if tmp == "" {
            flip_bool = true;
            continue;
        }
        if !flip_bool {
            initial_template = tmp
                .split("")
                .filter(|f| *f != "")
                .map(|f| f.chars().next().unwrap())
                .collect::<Vec<char>>()
                .to_owned();
        } else {
            let mut split = tmp.split("->");
            let mut group = split.next().unwrap().split("");
            instructions.push((
                group.nth(1).unwrap().chars().next().unwrap(),
                group.next().unwrap().chars().next().unwrap(),
                split.next().unwrap().chars().next().unwrap(),
            ));
        }
    }

    let vec : Vec<char> = perform_instructions(&instructions, &initial_template, 10);
    count_occurances(vec);

    // Runs in exponential time, oh well
    let vec : Vec<char> = perform_instructions(&instructions, &initial_template, 40);
    count_occurances(vec);
}

fn count_occurances(vec : Vec<char>){
    let mut map : BTreeMap<char, i32> = BTreeMap::new();
    for char in vec{
        let pointer = map.entry(char).or_insert(0);
        *pointer += 1;
    }
    let mut values : Vec<i32> = map.values().map(|f| *f).collect();
    values.sort();
    let biggest_smallest = values.last().unwrap() - values.first().unwrap();
    println!("The largest value subtracted by the smallest value is {}", biggest_smallest);

}

fn perform_instructions(
    instructions: &Vec<(char, char, char)>,
    initial_template: &Vec<char>,
    ending_index: i32
) -> Vec<char> {
    let start = SystemTime::now();
    let mut vec: Vec<char> = initial_template.clone();
    for repeat in 0..ending_index {
        let mut new_vec: Vec<char> = vec.clone();
        let mut inserts_before = 0;
        for index in 0..vec.len() - 1 {
            for instruction in instructions.iter() {
                if vec[index] == instruction.0 && vec[index + 1] == instruction.1 {
                    new_vec.insert(index + inserts_before + 1, instruction.2.clone());
                    inserts_before += 1;
                }
            }
        }
        vec = new_vec.clone();
        if repeat % 3 == 2{
            println!("Index {} at {:?}", repeat, SystemTime::now().duration_since(start).expect("msg"));
        }
    }
    return vec;
}
