use std::fs;

fn main() {
    let filename = "src/data/data.txt";
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    // Collect file into 2d array
    let map: Vec<Vec<i32>> = contents
        .split_inclusive("\n")
        .map(|f| {
            f.split_inclusive("")
                .filter(|l| !l.is_empty() && !l.trim().is_empty())
                .map(|s| s.trim().parse::<i32>().unwrap())
                .collect()
        })
        .collect();

    // Determine what the local mins are, only considering cardinal directions
    let local_mins: Vec<(i32, (i32, i32))> = first(&map);

    // Determine "basins" around local mins, which are areas that don't have height nine
    // Print product of top 3 basin sizes
    second(&map, local_mins);
}

fn first(map: &Vec<Vec<i32>>) -> Vec<(i32, (i32, i32))> {
    let mut smallest_locals: Vec<(i32, (i32, i32))> = Vec::new();
    for i in 0..map.len() {
        for j in 0..map[i].len() {
            if check_local_minimum(map, i as i32, j as i32) {
                smallest_locals.push((map[i][j], (i as i32, j as i32)))
            }
        }
    }
    let sum: i32 = smallest_locals.iter().map(|f| f.0 + 1).sum();
    println!("The sum of the local minimums was {}", sum);
    return smallest_locals;
}

// Determines what the basins around a local minimum are, which are points that aren't nine
// prints the product of the top three basins length
fn second(map: &Vec<Vec<i32>>, local_mins: Vec<(i32, (i32, i32))>) {
    let mut basin_sizes: Vec<i32> = Vec::new();

    // Find the basin for each local min
    for local_min in local_mins {
        let mut positions_to_check: Vec<(i32, i32)> = Vec::new();
        let mut basin: Vec<(i32, i32)> = Vec::new();

        //Begin by checking the points around the minimum
        positions_to_check.push(local_min.1);
        basin.push(local_min.1);
        while positions_to_check.len() > 0 {
            let neighbor: (i32, i32) = positions_to_check.remove(0);
            for z in -1 as i32..2 as i32 {
                for y in -1 as i32..2 as i32 {
                    if z == -1 && y == -1
                        || z == 1 && y == 1
                        || z == -1 && y == 1
                        || z == 1 && y == -1
                        || z == 0 && y == 0
                    {
                        continue;
                    }
                    // Check if point is inbounds
                    if (neighbor.0 + z >= 0 && neighbor.0 + z < map.len() as i32)
                        && (neighbor.1 + y >= 0
                            && neighbor.1 + y < map[(neighbor.0 + z) as usize].len() as i32)
                    {
                        // Check if point is a part of the basin and already hasn't been added to the basin
                        let new_position = (neighbor.0 + z, neighbor.1 + y);
                        if map[(neighbor.0 + z) as usize][(neighbor.1 + y) as usize] != 9
                            && !basin.contains(&(neighbor.0 + z, neighbor.1 + y))
                        {
                            // add point to the basin, and add it to the list of positions to be checked
                            positions_to_check.push(new_position);
                            basin.push(new_position);
                        }
                    }
                }
            }
        }
        basin_sizes.push(basin.len() as i32);
    }

    basin_sizes.sort_by(|a, b| b.cmp(a));
    println!(
        "The product of the top three basin sizes is {}",
        basin_sizes[0] * basin_sizes[1] * basin_sizes[2]
    )
}

// Determines if a point is a local minimum on the map
fn check_local_minimum(map: &Vec<Vec<i32>>, i: i32, j: i32) -> bool {
    let mut smallest_local = true;
    for z in -1 as i32..2 as i32 {
        for y in -1 as i32..2 as i32 {
            // Checks four cardinal neighbors, not diagnols nor itself
            if z == -1 && y == -1
                || z == 1 && y == 1
                || z == -1 && y == 1
                || z == 1 && y == -1
                || z == 0 && y == 0
            {
                continue;
            }
            // Check if position is within the bounds of the map
            if (i + z >= 0 && i + z < map.len() as i32)
                && (j + y >= 0 && j + y < map[(i + z) as usize].len() as i32)
            {
                // If the neighbor is smaller or equal to than the position, this is not a local minimum
                if map[(i + z) as usize][(j + y) as usize] <= map[i as usize][j as usize] {
                    smallest_local = false;
                }
            }
        }
    }
    return smallest_local;
}
