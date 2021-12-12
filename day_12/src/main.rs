use std::{collections::BTreeMap, fs};
use trees::{tree, Tree};

fn main() {
    let filename = "src/data/datasmall.txt";
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    // Collect file into 2d array
    let mut map: BTreeMap<&str, Vec<&str>> = BTreeMap::new();
    contents.split_inclusive("\n").for_each(|f| {
        let parts: Vec<&str> = f.split("-").collect();
        let mut pointer = map.entry(parts[0]).or_insert(Vec::new());
        pointer.push(parts[1]);
        pointer = map.entry(parts[1]).or_insert(Vec::new());
        pointer.push(parts[0]);
    });

    // Determine what the local mins are, only considering cardinal directions
    first(map);

    // Determine "basins" around local mins, which are areas that don't have height nine
    // Print product of top 3 basin sizes
    //second(&map, local_mins);
}

fn first(map: BTreeMap<&str, Vec<&str>>) {
    let mut different_paths: Tree<&str> = Tree::new("*");
    different_paths.push_back(Tree::new("start"));
    let mut changed_check: bool = true;
    while changed_check {
        changed_check = false;
        for mut position in different_paths.iter_mut(){
            if position.has_no_child() {
                for paths in map.get(position.data()).unwrap() {
                    changed_check = true;
                    position.push_back(Tree::new(paths));
                }
            }
        }
    }
    println!("{}", different_paths.to_string());
}

// fn path_depth_first(map: BTreeMap<&str, Vec<&str>>, start: &str) {
//     let mut stack: Vec<&str> = Vec::new();
//     let mut discovered: Vec<&str> = Vec::new();
//     let paths : Vec<&str> = Vec::new();
//     stack.push(start);
//     while !stack.is_empty() {
//         let node = stack.pop().unwrap();
//         if !discovered.contains(&node) {
//             for new_nodes in map.get(node) {
//                 for new_node in new_nodes {
//                     stack.push(new_node);
//                 }
//             }
//         }

//         //println!("{}", position);
//     }
// }
