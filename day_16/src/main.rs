use std::{collections::BTreeMap, fs, ops::Add, str::Chars};

fn main() {
    let filename = "src/data/datasmall1.txt";
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let binary = convert_hex_to_binary(&contents);
    count_version_numbers(&binary);
}

fn count_version_numbers(binary: &str) {
    let mut count = 0;
    let mut packet_information: Vec<(i32, i32, i32, i32, i32)> = Vec::new();
    packet_information.push(get_packet_information(binary, 0));
    let mut cur_packet = packet_information.last().unwrap().clone();
    while cur_packet.4 < binary.len() as i32 {
        if cur_packet.2 == 1 {
            for i in 0..cur_packet.3 {
                if substring(binary, cur_packet.4 as usize, 1) == "0" {
                    println!("Value Packet");
                } else {
                    packet_information.push(get_packet_information(binary, (cur_packet.4 + (i * 11)) as usize))
                }
            }
        } else {
            panic!();
        }
        cur_packet = packet_information.last().unwrap().clone();
    }
}

fn get_packet_information(binary_string: &str, start_index: usize) -> (i32, i32, i32, i32, i32) {
    let version = binary_string_to_int(substring(binary_string, start_index, 3));
    let id = binary_string_to_int(substring(binary_string, start_index + 2, 3));
    let length_id = binary_string_to_int(substring(binary_string, start_index + 5, 3));
    let mut length_packet = 0;
    let char_index: i32;
    if length_id == 0 {
        length_packet = binary_string_to_int(substring(binary_string, start_index + 8, 15));
        char_index = (start_index + 8 + 15) as i32;
    } else {
        length_packet = binary_string_to_int(substring(binary_string, 8, 11));
        char_index = (start_index + 8 + 11) as i32;
    }
    return (version, id, length_id, length_packet, char_index);
}

fn convert_hex_to_binary(contents: &str) -> String {
    let mut rtn: String = String::new();
    let conversion_map: BTreeMap<char, &str> = BTreeMap::from([
        ('0', "0000"),
        ('1', "0001"),
        ('2', "0010"),
        ('3', "0011"),
        ('4', "0100"),
        ('5', "0101"),
        ('6', "0110"),
        ('7', "0111"),
        ('8', "1000"),
        ('9', "1001"),
        ('A', "1010"),
        ('B', "1011"),
        ('C', "1100"),
        ('D', "1101"),
        ('E', "1110"),
        ('F', "1111"),
    ]);
    for char in contents.chars() {
        rtn += conversion_map.get(&char).unwrap().clone();
    }

    return rtn;
}

fn chars_to_string(chars: Vec<char>) -> String {
    let mut rtn = String::new();
    for char in chars {
        rtn += &char.to_string();
    }
    return rtn;
}

fn binary_string_to_int(gamma_res: &str) -> i32 {
    let mut final_val: i32 = 0;
    let base: i32 = 2;
    for (index, res) in gamma_res.chars().enumerate() {
        if res == '1' {
            final_val += base.pow(((gamma_res.len() - index - 1) as usize) as u32) as i32;
        }
    }
    return final_val;
}

// Borrowed from https://users.rust-lang.org/t/how-to-get-a-substring-of-a-string/1351/10
fn substring(str: &str, start: usize, len: usize) -> &str {
    let mut char_pos = 0;
    let mut byte_start = 0;
    let mut it = str.chars();
    loop {
        if char_pos == start {
            break;
        }
        if let Some(c) = it.next() {
            char_pos += 1;
            byte_start += c.len_utf8();
        } else {
            break;
        }
    }
    char_pos = 0;
    let mut byte_end = byte_start;
    loop {
        if char_pos == len {
            break;
        }
        if let Some(c) = it.next() {
            char_pos += 1;
            byte_end += c.len_utf8();
        } else {
            break;
        }
    }
    return &str[byte_start..byte_end];
}
