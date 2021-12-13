use std::env;
use std::io::{self, prelude::*, BufReader};
use std::fs::File;
use std::collections::{VecDeque,HashSet,HashMap};

enum CaveType {
    Start,
    End,
    Large,
    Small,
}
impl CaveType {
    pub fn type_from(name: &str) -> Self {
        match name {
            "start" => CaveType::Start,
            "end"   => CaveType::End,
            other => match other.chars().next() {
                Some('a'..='z') => CaveType::Small,
                Some('A'..='Z') => CaveType::Large,
                other => panic!("Unknown character: {:?}", other),
            }
        }
    }
}

struct Cave {
    name: String,
    connections: Vec<String>,
}
impl From<&str> for Cave {
    fn from(input: &str) -> Self {
        Self {
            name: input.to_string(),
            connections: Vec::new(),
        }
    }
}

// Checks whether any small caves have been revisited along this path already
fn has_revisited_small_cave(path: &String) -> bool {
    let mut small_cave_visits: HashMap<String,usize> = HashMap::new();
    let small_caves = path.split("-").filter(|c| match CaveType::type_from(c) { CaveType::Small => true, _ => false});
    for small_cave in small_caves {
        *small_cave_visits.entry(small_cave.to_string()).or_insert(0) += 1;
    }
    small_cave_visits.iter().filter(|&(_,v)| *v > 1).count() > 0
}

fn count_paths(caves: &Vec<Cave>, allow_revisit: bool) -> usize {
    let mut paths: HashSet<String> = HashSet::new();
    let mut q: VecDeque<String> = VecDeque::new();
    q.push_back("start".to_string());
    while q.len() > 0 {
        
        // Get current cave name and data
        let current_path = q.pop_front().unwrap();
        let current_name = current_path.split("-").last().unwrap();
        let current_cave = &caves.iter().filter(|c| c.name == current_name).next().unwrap();
        
        // Investigate connections
        for connection in &current_cave.connections {
            let new_path = format!("{}-{}",current_path,connection);
            match CaveType::type_from(&connection) {
                CaveType::Start => {}, // can't revisit start
                CaveType::End   => { paths.insert(new_path); },
                CaveType::Large => { q.push_back(new_path); },
                CaveType::Small => {
                    // Count instances of this connection's name in current path
                    let count = current_path.split("-").filter(|c| c == connection).count();
                    match count {
                        0 => q.push_back(new_path), // first visit to this small cave
                        1 if allow_revisit && !has_revisited_small_cave(&current_path) => q.push_back(new_path), // first small cave revisit
                        _ => {},
                    }
                },
            }
        }
    }
    paths.len()
}

fn solve(input: &str) -> io::Result<()> {
    let file = File::open(input).expect("Input file not found.");
    let reader = BufReader::new(file);

    // Input
    let input: Vec<String> = match reader.lines().collect() {
        Err(err) => panic!("Unknown error reading input: {}", err),
        Ok(result) => result,
    };

    // Make caves
    let mut caves: Vec<Cave> = Vec::new();
    for line in &input {
        for cave in line.split('-') {
            if caves.iter().filter(|c| c.name == cave).count() == 0 {
                caves.push(Cave::from(cave));
            }
        }
    }

    // Connect caves
    for line in &input {
        let mut both_caves = line.split('-');
        let cave1 = both_caves.next().unwrap();
        let cave2 = both_caves.next().unwrap();
        caves.iter_mut().filter(|c| c.name == cave1).for_each(|c| c.connections.push(cave2.to_string()));
        caves.iter_mut().filter(|c| c.name == cave2).for_each(|c| c.connections.push(cave1.to_string()));
    }

    // Solve parts 1 & 2
    println!("Part 1: {}", count_paths(&caves,false)); // 4411
    println!("Part 2: {}", count_paths(&caves,true)); // 136767

    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    solve(&filename).unwrap();
}

