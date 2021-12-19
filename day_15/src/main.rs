use std::{collections::{BTreeMap, BTreeSet}, fs};
use std::iter::FromIterator;
use std::fs::File;
use std::io::Write;

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
    //first(&map);

    let map_bigger = make_bigger_map(map);
    djikstras(&map_bigger);
    //first(&map_bigger);

    // Determine "basins" around local mins, which are areas that don't have height nine
    // Print product of top 3 basin sizes
    //second(&map, local_mins);
}

fn djikstras(map: &Vec<Vec<i32>>) {
    let mut prev: BTreeMap<(usize, usize), (usize, usize)> = BTreeMap::new();
    let mut distance: BTreeMap<(usize, usize), i32> = BTreeMap::new();
    let mut vertex_set: BTreeSet<(usize, usize)> = BTreeSet::new();
    for i in 0..map.len() {
        for j in 0..map.len() {
            distance.insert((i, j), i32::MAX);
            vertex_set.insert((i, j));
        }
    }
    distance.insert((0, 0), 0);
    let mut left = vertex_set.len();
    while left != 0 {
        let vert: (usize, usize) = vert_get_smallest(&distance, &mut vertex_set);
        for i in -1..2i32 {
            for j in -1..2i32 {
                if i == -1 && j == -1
                    || i == 1 && j == 1
                    || i == -1 && j == 1
                    || i == 1 && j == -1
                    || i == 0 && j == 0
                {
                    continue;
                }

                let neighbor = (vert.0 as i32 - i, vert.1 as i32 - j);
                if neighbor.0 >= 0
                    && neighbor.0 < map.len() as i32
                    && neighbor.1 >= 0
                    && neighbor.1 < map[0].len() as i32
                {
                    if vertex_set.contains(&(neighbor.0 as usize, neighbor.1 as usize)) {
                        let alt = distance.get(&(vert.0 as usize, vert.1 as usize)).unwrap()
                            + map[neighbor.0 as usize][neighbor.1 as usize];
                        if alt
                            < *distance
                                .get(&(neighbor.0 as usize, neighbor.1 as usize))
                                .unwrap()
                        {
                            distance.insert((neighbor.0 as usize, neighbor.1 as usize), alt);
                            prev.insert(
                                (neighbor.0 as usize, neighbor.1 as usize),
                                (vert.0 as usize, vert.1 as usize),
                            );
                        }
                    }
                }
            }
        }
        left = vertex_set.len();
        if left % 5001 == 5000{
            println!("{}", left);
        }
    }

    // println!("");
    // let mut start_node = (map.len(),map);
    // let mut dist = 0;
    // while start_node != (0,0){
    //     let item = prev.iter().filter(|f| *f.0 == start_node).nth(0).unwrap();
    //     println!(
    //         "({},{}) -> ({},{})",
    //         item.0 .0, item.0 .1, item.1 .0, item.1 .1
    //     );
    //     start_node = *item.1;
    // }
    // println!("");
    println!(
        "Dist: {}",
        distance.get(&(map.len() - 1, map[0].len() - 1)).unwrap()
    );
    write_file(distance);
}


fn vert_get_smallest(
    dist: &BTreeMap<(usize, usize), i32>,
    vertex_set: &mut BTreeSet<(usize, usize)>,
) -> (usize, usize) {
    let mut v = Vec::from_iter(dist);
    v.sort_by(|&(_, a), &(_, b)| a.cmp(&b));
    for vert in v{
        if vertex_set.contains(vert.0){
            vertex_set.remove(vert.0);
            return  *vert.0;
        }
    }
    return (0, 0)
}

fn make_bigger_map(map: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut bigger_map: Vec<Vec<i32>> = Vec::new();
    for i in 0..map.len() * 5 {
        bigger_map.push(Vec::new());
        for j in 0..map[0].len() * 5 {
            let i_add: i32 = i as i32 / map.len() as i32;
            let j_add: i32 = j as i32 / map.len() as i32;
            let new_val = map[i % map.len()][j % map.len()] + i_add + j_add;
            if new_val / 10 >= 1 {
                bigger_map[i].push((new_val % 10) + 1);
            } else {
                bigger_map[i].push(new_val % 10);
            }
        }
    }
    return bigger_map;
}

fn write_file(dist : BTreeMap<(usize, usize), i32>) {
    // Create a temporary file.

    // Open a file in write-only (ignoring errors).
    // This creates the file if it does not exist (and empty the file if it exists).
    let mut file = File::create("distout.txt").unwrap();

    // Write a &str in the file (ignoring the result).
    for keyval in dist{
        writeln!(&mut file, "{},{} -> {}", keyval.0.0, keyval.0.1, keyval.1).unwrap();
    }
}
