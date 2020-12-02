use std::fs::File;
use std::io::prelude::*;
use nom::*;
use nom::character::complete::*;
use nom::bytes::complete::*;

#[allow(dead_code)]
fn main() {
    let mut file = File::open("input.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let mut count = 0;
    for line in contents.split("\n") {
        if line == "" { break; }
        if parse(line).unwrap().1 {
            count += 1;
        }
    }
    println!("{}",count);
}

fn parse(i: &str) -> IResult<&str, bool> {
    let (i, min) = digit1(i)?;
    let (i, _) = tag("-")(i)?;
    let (i, max) = digit1(i)?;
    let (i, _) = tag(" ")(i)?;
    let (i, chr) = anychar(i)?;
    let (i, _) = tag(": ")(i)?;
    let (i, st) = alphanumeric1(i)?;
    
    let min = min.parse::<u32>().unwrap();
    let max = max.parse::<u32>().unwrap();

    let mut count = 0;
    for c in st.chars() {
        if c == chr { count += 1; }
    }
    return Ok((i, count >=min && count <= max));
}
