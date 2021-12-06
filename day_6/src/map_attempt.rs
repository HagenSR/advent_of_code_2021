use std::{collections::HashMap, fs};

fn main() {
    let filename = "src/data.txt";
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let state: Vec<i32> = contents
        .split(",")
        .map(|fish| fish.trim().parse().unwrap())
        .collect();
    let mut Map: HashMap<i32, i32> = HashMap::new();
    for i in state {
        Map.insert(i, 1);
    }

    // Laternfish in the ocean reproduce every 6 days. A new laternfish reproduces after 8 days.
    //How many lanternfish are there after 80 and 256 days?
    for i in 0..256 {
        let mut temp_map: HashMap<i32, i32> = HashMap::new();
        for fish_days in Map.keys() {
            if *fish_days == 0 {
                if temp_map.contains_key(fish_days) {
                    let old_6 = temp_map.get_mut(&6).unwrap();
                    *old_6 += *Map.get(&6).unwrap();
                    let old_8 = temp_map.get_mut(&8).unwrap();
                    *old_8 += *Map.get(&8).unwrap();

                }else{
                    temp_map.insert(6, *Map.get(fish_days).unwrap());
                    temp_map.insert(8, *Map.get(fish_days).unwrap());
                }
            }else{
                if temp_map.contains_key(&(*fish_days -1)) {
                    let old = temp_map.get_mut(&(*fish_days -1)).unwrap();
                    *old += *Map.get(&(*fish_days)).unwrap();
                }else{
                    temp_map.insert(*fish_days -1, *Map.get(fish_days).unwrap());
                }
            }
            
        }
        Map = temp_map;
        if i == 79 {
            println!(
                "There are {} Fish after {} days",
                Map.values().sum::<i32>(),
                i + 1
            );
        }
    }
    println!(
        "There are {} Fish after {} days",
        Map.values().sum::<i32>(),
        256
    );
}
