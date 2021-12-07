use std::fs;

fn main() {
    let filename = "src/data/data.txt";
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let mut state: Vec<i32> = contents
        .split(",")
        .map(|fish| fish.trim().parse().unwrap())
        .collect();

    // Determine median of vector and distance
    let len = state.len() / 2;
    state.sort();
    let med = state[len];
    let distance_median = calc_distance(&state, med, false);

    println!(
        "Median is {} with smallest distance {}",
        med, distance_median
    );

    // Get exp middle and cost
    let exp_middle = determine_exp_middle(&state);

    println!(
        "Average is {} with smallest distance {}",
        exp_middle.0, exp_middle.1
    )
}

// Brute force determination of middle when distances have exponential cost
fn determine_exp_middle(state: &Vec<i32>) -> (i32, i32) {
    let mut smallest_distance = -1;
    let mut middle = -1;
    for i in state[0]..*state.last().unwrap() {
        let mut distance = 0;
        distance += calc_distance(state, i, true);
        if distance < smallest_distance || smallest_distance == -1 {
            middle = i;
            smallest_distance = distance
        }
    }
    return (middle, smallest_distance);
}

// Calc distance either with linear cost or exponential cost
fn calc_distance(state: &Vec<i32>, middle: i32, base: bool) -> i32 {
    let mut distance = 0;
    for j in state {
        if base {
            let tmp = (middle - *j).abs();
            for i in 1..tmp + 1 {
                distance += i;
            }
        } else {
            distance += (middle - j).abs();
        }
    }
    return distance;
}
