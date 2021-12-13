use std::fs::{self, File};
use std::io::{BufRead, BufReader};

fn main() {
    let filename = "src/data/data.txt";
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    // Open the file in read-only mode (ignoring errors).
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut flip_bool = false;
    let mut points_vec: Vec<(i32, i32)> = Vec::new();
    let mut instructions: Vec<(bool, i32)> = Vec::new();
    // Read the file line by line using the lines() iterator from std::io::BufRead.
    for line in reader.lines() {
        let tmp = line.unwrap();
        if tmp == "" {
            flip_bool = true;
            continue;
        }
        if !flip_bool {
            let split: Vec<i32> = tmp.split(",").map(|f| f.parse().unwrap()).collect();
            points_vec.push((split[0], split[1]));
        } else {
            let mut split = tmp.split(" ").nth(2).unwrap().split("=");
            if split.next().unwrap() == "y" {
                instructions.push((true, split.next().unwrap().parse().unwrap()))
            } else {
                instructions.push((false, split.next().unwrap().parse().unwrap()))
            }
        }
    }

    // Determine how many dots are in the first fold
    determine_folds(&mut points_vec, instructions);
    plot_and_print(&mut points_vec);
}

fn determine_folds(points: &mut Vec<(i32, i32)>, intstructions: Vec<(bool, i32)>) {
    let mut counter = 0;
    for instruction in intstructions {
        for point in points.iter_mut() {
            // Flip on y axis
            if instruction.0 {
                if point.1 > instruction.1 {
                    point.1 = instruction.1 - (point.1 - instruction.1);
                }
            }
            // Flip on x axis
            else {
                if point.0 > instruction.1 {
                    point.0 = instruction.1 - (point.0 - instruction.1);
                }
            }
        }
            *points = determine_unique_points(points,counter);
            counter+=1;
            //break;
    }
}

fn determine_unique_points(points: &Vec<(i32, i32)>, counter: i32) -> Vec<(i32, i32)> {
    let mut new_points: Vec<(i32, i32)> = Vec::new();
    for point in points {
        if !new_points.contains(&point) {
            new_points.push(point.clone());
        }
    }
    println!(
        "There were {} points after the {} fold",
        new_points.len(),
        counter
    );
    return new_points;
}

fn plot_and_print(points : &mut Vec<(i32, i32)>){
    points.sort_by(|a, b| b.cmp(a));
    let max = points[0];
    let mut arr : [[i32; 39]; 6] = [[0;39];6];
    for point in points{
        arr[point.1 as usize][point.0 as usize] = 1;
    }
    for line in arr{
        for point in line{
            if point == 1{
                print!("{}", point);
            }else {
                print!(" ");
            }

        }
        println!("")

    }


}