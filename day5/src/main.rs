use std;
use std::fs::File;
use std::io::prelude::*;

#[allow(dead_code)]
fn main_1() {
    let mut file = File::open("input.txt").unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();
    let mut max = 0;
    for line in content[0.. content.len()-1].split("\n") {
        let bits = line.replace("F","0").replace("B","1").replace("L","0").replace("R","1");
        let id = i32::from_str_radix(&bits, 2).unwrap();
        if id > max { max = id; }
    }
    println!("max={}",max);
}

fn main() {
    let mut file = File::open("input.txt").unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();
    let mut seats = content[0.. content.len()-1].split("\n").into_iter().map(|line| {
        let bits = line.replace("F","0").replace("B","1").replace("L","0").replace("R","1");
        i32::from_str_radix(&bits, 2).unwrap()
    }).collect::<Vec<i32>>();
    seats.sort();
    for s in 0..seats.len()-2 {
        if seats[s] + 2 == seats[s+1] {
            println!("found {}", seats[s] + 1);
        }
    }
}

