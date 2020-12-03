use std::fs::File;
use std::io::prelude::*;

#[allow(dead_code)]
fn main_1() {
    let mut trees = 0;
    let mut file = File::open("input.txt").unwrap();

    // find first tree
    let mut chr = [0; 1];
    file.read(&mut chr[..]).unwrap();
    if chr[0] == '#' as u8 {
        trees += 1;
    }
    // find line size
    let mut size = 0;
    while chr[0] != '\n' as u8 {
        file.read(&mut chr[..]).unwrap();
        size += 1;
    }
    // read and count trees
    let mut pos = 3;
    let mut buffer = vec![0; size+1]; // +1 for \n
    println!("width={}", size);
    while file.read(&mut buffer[..]).unwrap() > 0 {
        if buffer[pos] == '#' as u8 {
            trees += 1;
        }
        pos = (pos+3)%size;
    }
    println!("trees={}", trees);
}

fn main() {
    let bytes = std::fs::read("input.txt").unwrap();
    let width = bytes.iter().position(|&r| r == '\n' as u8).unwrap();
    let trees1 = count_trees(&bytes, width, 1, 1);
    let trees2 = count_trees(&bytes, width, 3, 1);
    let trees3 = count_trees(&bytes, width, 5, 1);
    let trees4 = count_trees(&bytes, width, 7, 1);
    let trees5 = count_trees(&bytes, width, 1, 2);
    println!("trees={}*{}*{}*{}*{}", trees1,trees2,trees3,trees4,trees5);
    println!("trees={}", trees1*trees2*trees3*trees4*trees5);
}

fn count_trees(bytes: &[u8], width: usize, shift: usize, down:usize) -> i64 {
    let mut trees = 0;
    let mut posx = 0;
    let mut posy = 0;
    let mut pos = 0;
    while pos < bytes.len() {
        if bytes[pos] == '#' as u8 {
            trees += 1;
        }
        posx = (posx+shift)%width;
        posy += down;
        pos = posx + posy * (width+1);
    }
    return trees;
}

