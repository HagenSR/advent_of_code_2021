use std::collections::BTreeMap;
use std::fs;

fn main() {
    let filename = "src/data/data.txt";
    let contents = fs::read_to_string(filename).expect("Bad File");
    let splits = contents.split("\n");
    let mut edge_list: BTreeMap<&str, Vec<&str>> = BTreeMap::new();
    for split in splits {
        let nodes: Vec<&str> = split.split("-").map(|f| f.trim()).collect();
        let entry = edge_list.entry(nodes[0]).or_insert(Vec::new());
        entry.push(nodes[1]);

        let entry = edge_list.entry(nodes[1]).or_insert(Vec::new());
        entry.push(nodes[0]);
    }
    //print_map(&edge_list);
    println!(
        "Number of paths if you can only travel to small caves once: {}",
        count(&edge_list, true, "start", &Vec::new())
    );
    println!(
        "Number of paths if you can travel to a small cave twice: {}",
        count(&edge_list, false, "start", &Vec::new())
    );
}

fn count(
    edge_list: &BTreeMap<&str, Vec<&str>>,
    visit_small_caves_multiple: bool,
    cave: &str,
    seen: &Vec<&str>,
) -> i32 {
    let mut cancel_multiple = false;
    if cave == "end" {
        return 1;
    }
    if seen.contains(&cave) {
        if cave == "start" {
            return 0;
        }
        if cave.chars().next().expect("no char").is_lowercase() {
            if visit_small_caves_multiple {
                return 0;
            } else {
                cancel_multiple = true;
            }
        }
    }
    let mut rtn_values: Vec<i32> = Vec::new();
    let mut neighbor_clone = seen.clone();
    neighbor_clone.push(cave);
    for neigbor in edge_list.get(cave).expect("Bad cave") {
        rtn_values.push(count(
            edge_list,
            visit_small_caves_multiple || cancel_multiple,
            neigbor,
            &neighbor_clone,
        ));
    }

    return rtn_values.iter().sum();
}

fn print_map(edge_list: &BTreeMap<char, Vec<&str>>) {
    for key in edge_list.keys() {
        print!("Key {} has connections to ", key);
        for value in edge_list.get(key) {
            for str in value {
                print!("{}, ", str);
            }
        }
        println!();
    }
}
