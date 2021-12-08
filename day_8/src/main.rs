use std::{fs, collections::BTreeMap};

fn main() {
    let filename = "src/data/data.txt";
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let mut displays: Vec<&str> = contents
        .split_inclusive("\n")
        .collect();
    first(&displays);
    
}

fn first(displays : &Vec<&str>) {
    let mut counter = 0;
    for display in displays{
        let output : Vec<&str> = display.split("|").nth(1).unwrap().split_inclusive(" ").map(|f| f.trim()).collect();
        for out in output{
            if out.len() == 2 || out.len() == 4 || out.len() == 3 || out.len() == 7{
                counter+=1;
            }
        }
    }
    println!("There were {} unique digits (1, 4, 7, or 8)", counter);
}

fn second(displays : &Vec<&str>){

}

fn determine_display_mapping(display : &Vec<&str>) -> BTreeMap<char, char>{
    let mut mapping : BTreeMap<char,char> = BTreeMap::from([('a', 'n'),('b','n'),('c','n'),('d', 'n'),('e','n'),('f','n'), ('g', 'n')]);
    


    return  mapping;
}