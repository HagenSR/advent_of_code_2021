use std::{collections::BTreeMap, fs};

fn main() {
    let filename = "src/data.txt";
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let state: Vec<i64> = contents
        .split(",")
        .map(|fish| fish.trim().parse().unwrap())
        .collect();
    // For some reason, Hash map has strange random behaviors and doesn't work?
    let mut map: BTreeMap<i64, i64> = BTreeMap::new();
    for i in state {
        if map.contains_key(&i) {
            let old = map.get_mut(&i).unwrap();
            *old += 1;
        } else {
            map.insert(i, 1);
        }
    }

    // Laternfish in the ocean reproduce every 6 days. A new laternfish reproduces after 8 days.
    //How many lanternfish are there after 80 and 256 days?
    for i in 0..256 {
        let mut temp_map: BTreeMap<i64, i64> = BTreeMap::new();
        for fish_days in map.keys() {
            if *fish_days == 0 {
                if temp_map.contains_key(&6) {
                    let old_6 = temp_map.get_mut(&6).unwrap();
                    *old_6 = *map.get(&6).unwrap() + *old_6;
                } else {
                    temp_map.insert(6, *map.get(fish_days).unwrap());
                }
                if temp_map.contains_key(&8) {
                    let old_8 = temp_map.get_mut(&8).unwrap();
                    *old_8 = *map.get(&8).unwrap() + *old_8;
                } else {
                    temp_map.insert(8, *map.get(fish_days).unwrap());
                }
            } else {
                if temp_map.contains_key(&(*fish_days - 1)) {
                    let old = temp_map.get_mut(&(fish_days - 1)).unwrap();
                    *old = *map.get(fish_days).unwrap() + *old;
                } else {
                    temp_map.insert(fish_days - 1, *map.get(fish_days).unwrap());
                }
            }
        }
        map = temp_map.clone();
        if i == 79 || i == 17 {
            println!(
                "There are {} Fish after {} days",
                map.values().sum::<i64>(),
                i + 1
            );
        }
        // let mut rtn: String = String::new();
        // for j in map.keys(){
        //     for res in 0..*map.get(j).unwrap(){
        //         rtn += &j.to_string();
        //         rtn += ",";
        //     }
        // }
        // println!("Day {}: {}",i+1, rtn)
    }
    println!(
        "There are {} Fish after {} days",
        map.values().sum::<i64>(),
        256
    );
}
