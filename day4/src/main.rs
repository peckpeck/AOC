use std;
use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;
use nom::{ *, error::*, branch::*, character::*, character::complete::*, bytes::complete::*, sequence::*, multi::*, combinator::* };

static MANDATORY_FIELD_LIST: [&str; 7] = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

#[allow(dead_code)]
fn main_1() {
    let mut file = File::open("input.txt").unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();
    let res: IResult<&str,Vec<bool>> = separated_list1(
        tag("\n"),
        map(
            terminated(
                separated_list1(
                    one_of(" \n"),
                    separated_pair(
                        alphanumeric1,
                        tag(":"),
                        take_till(|c| c == ' ' || c == '\n')
                    )
                ),
                tag("\n")
            ),
            |x| {
                let passport: HashMap<&str,&str> = x.into_iter().collect();
                MANDATORY_FIELD_LIST.iter().map(|f| passport.contains_key(f)).all(|b| b)
            }
        )
    )(&content);
    let count = res.unwrap().1.iter().fold(0, |count, &b| if b {count + 1} else {count});
    println!("{}", count);
}
fn main() {
    let mut file = File::open("input.txt").unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();
    let res: IResult<&str,Vec<bool>> = separated_list1(
        tag("\n"),
        map(
            terminated(
                separated_list1(
                    one_of(" \n"),
                    separated_pair(
                        alphanumeric1,
                        tag(":"),
                        take_till(|c| c == ' ' || c == '\n')
                    )
                ),
                tag("\n")
            ),
            |x| {
                let passport: HashMap<&str,&str> = x.into_iter().collect();
                if MANDATORY_FIELD_LIST.iter().map(|f| passport.contains_key(f) ).all(|b| b) {
                    match check(&passport) { 
                        Ok(b) => b,
                        Err(_) => false
                    }
                } else { 
                    println!("missing");
                    false
                }
            }
        )
    )(&content);
    let count = res.unwrap().1.iter().fold(0, |count, &b| if b {count + 1} else {count});
    println!("{}", count);
}

// Make type cheking in results stop bugging us
enum AnyErr { Any }
impl<E> From<E> for AnyErr where E: std::error::Error {
    fn from(error: E) -> Self { AnyErr::Any }
}
// Make parser calling shorter without error bugging
macro_rules! parse {
    ($i:expr,$j:expr) => {{ let x:IResult<&str,_> = $i($j); x?.1 }}
}

fn check(passport: &HashMap<&str,&str>) -> Result<bool,AnyErr> {
    let byr = parse!(all_consuming(digit1),passport["byr"]).parse::<i32>()?;
    if byr < 1920 || byr > 2002 { return Ok(false); }
    let iyr = parse!(all_consuming(digit1),passport["iyr"]).parse::<i32>()?;
    if iyr < 2010 || iyr > 2020 { return Ok(false); }
    let eyr = parse!(all_consuming(digit1),passport["eyr"]).parse::<i32>()?;
    if eyr < 2020 || eyr > 2030 { return Ok(false); }
    let (hgt,unit) = parse!(all_consuming(alt((
            pair(digit1,tag("cm")),
            pair(digit1,tag("in"))
        ))),
        passport["hgt"]);
    let hgt = hgt.parse::<i32>()?;
    if unit == "cm" && (hgt < 150 || hgt > 193) { return Ok(false); }
    if unit == "in" && (hgt < 59 || hgt > 76) { return Ok(false); }
    let _hcl = parse!(all_consuming(preceded(tag("#"),count(one_of("0123456789abcdef"),6))),passport["hcl"]);
    let _ecl = parse!(all_consuming(alt((tag("amb"),tag("blu"),tag("brn"),tag("gry"),tag("grn"),tag("hzl"),tag("oth")))),passport["ecl"]);
    let _pid = parse!(all_consuming(count(one_of("0123456789"),9)),passport["pid"]);
    Ok(true)
}
