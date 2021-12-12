use std::fs;

fn main() {
    let filename = "src/data/data.txt";
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    // Collect file into 2d array
    let mut map: Vec<Vec<i32>> = contents
        .split_inclusive("\n")
        .map(|f| {
            f.split_inclusive("")
                .filter(|l| !l.is_empty() && !l.trim().is_empty())
                .map(|s| s.trim().parse::<i32>().unwrap())
                .collect()
        })
        .collect();

    // Determine what the local mins are, only considering cardinal directions
    first(&mut map);
    second(&mut map);

    // Determine "basins" around local mins, which are areas that don't have height nine
    // Print product of top 3 basin sizes
    //second(&map, local_mins);
}

fn first(map: &mut Vec<Vec<i32>>){
    let mut flashes = 0;
    for _i in 0..100{
        flashes += increment(map);
    }
    println!("There were a total of {} flashes for the first 100 turns", flashes);
}

fn second(map: &mut Vec<Vec<i32>>) {
    let mut i = 0;
    loop{
        i += 1;
        increment(map);

        let mut zero_check = true;
        for i in 0..10{
            for j in 0..10{
                if map[i][j] != 0{
                    zero_check = false;
                }
            }
        }

        if zero_check{
            // Add 100 to final results because 100 ticks have already run
            println!("All jellyfish flashed on turn {}", i+100);
            break;
        }
    }
}

fn increment(map: &mut Vec<Vec<i32>>) -> i32 {
    // First, increment every octopus
    for i in 0..map.len() {
        for j in 0..map[i].len() {
            map[i][j] += 1;
        }
    }
    // Then, check for flashes
    let mut already_flashed : Vec<(i32, i32)> = Vec::new();
    for i in 0..map.len() {
        for j in 0..map[i].len() {
            let pos =(i as i32,j as i32);
            if map[i][j] > 9 && !already_flashed.contains(&pos){
                increment_around(map, pos, &mut already_flashed)
            }
        }
    }

    // Then set any at or over 9 to zero
    for i in 0..map.len() {
        for j in 0..map[i].len() {
            if map[i][j] > 9{
                map[i][j] = 0;
            }
        }
    }

    return already_flashed.len() as i32;
}

fn increment_around(map: &mut Vec<Vec<i32>>, flashing_octo: (i32, i32), already_flashed : &mut Vec<(i32, i32)>  ){
    let mut positions_to_check: Vec<(i32, i32)> = Vec::new();
    //Begin by checking the points around the minimum
    positions_to_check.push(flashing_octo.clone());
    already_flashed.push(flashing_octo.clone());
    while positions_to_check.len() > 0 {
        let neighbor: (i32, i32) = positions_to_check.remove(0);
        for i in -1..2 {
            for j in -1..2 {
                if i == 0 && j == 0 {
                    continue;
                }
                // Check if point is inbounds
                if (neighbor.0 + i >= 0 && neighbor.0 + i < map.len() as i32)
                    && (neighbor.1 + j >= 0
                        && neighbor.1 + j < map[(neighbor.0 + i) as usize].len() as i32)
                {
                    // Check if point is a part of the basin and already hasn't been added to the basin
                    let new_position = (neighbor.0 + i, neighbor.1 + j);
                    map[new_position.0 as usize][new_position.1 as usize] += 1;
                    if map[(new_position.0) as usize][(new_position.1) as usize] > 9
                        && !already_flashed.contains(&new_position)
                    {
                        // add point to the basin, and add it to the list of positions to be checked
                        already_flashed.push((neighbor.0 + i, neighbor.1 + j));
                        //println!("{}",already_flashed.len());
                        positions_to_check.push(new_position.clone());
                    }
                }
            }
        }
    }
}
