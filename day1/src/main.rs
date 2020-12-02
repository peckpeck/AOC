use std::fs::File;
use std::io::prelude::*;

#[allow(dead_code)]
fn main_1() {
    let mut file = File::open("input.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let mut data: Vec<i32> = contents.split("\n").map(|x| x.parse().unwrap_or(2021)).collect();
    data.sort();
    let mut start=0;
    let mut end=data.len()-1;
    while start < end {
        let sum = data[start] + data[end];
        if sum < 2020 {
            start += 1;
        } else if sum > 2020 {
            end -= 1;
        } else {
            println!("{}", data[start]*data[end]);
            return;
        }
    }
}

fn main() {
    let mut file = File::open("input.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let mut data: Vec<i32> = contents.split("\n").map(|x| x.parse().unwrap_or(2021)).collect();
    data.sort();
    let mut start=0;
    let mut end = data.len()-1;
    for v0 in 0..end {
        while start < end {
            let sum = data[v0] + data[start] + data[end];
            if sum < 2020 {
                start += 1;
            } else if sum > 2020 {
                end -= 1;
            } else {
                println!("{}", data[v0]*data[start]*data[end]);
                return;
            }
        }
    }
}
