use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;


fn get_data() -> String {

    let mut f = File::open("./src/day5/input.txt").expect("File not Found");
    let mut contents = String::new();

    f.read_to_string(&mut contents)
        .expect("Error reading file");

    contents
}

fn destroy(a: char, b: char) -> bool {

    if a != b && a.to_ascii_lowercase() == b.to_ascii_lowercase() {
        return true
    }

    false
}

fn reduce(input: String) -> Vec<char> {

    let mut initial = vec![];

    for c in input.chars() {

        let last = initial.pop();
        let value;

        match last {
            Some(v) => value = v,
            None => {
                initial.push(c);
                continue;
            }
        }

        if destroy(c, value) { continue; }

        initial.push(value);
        initial.push(c);
    }

    initial
}

pub fn part1() -> String {

    let input = get_data();

    let res = reduce(input);

    res.len().to_string()
}

pub fn part2() -> String {

    let input = get_data();

    let mut results = HashMap::new();
    let chars = "abcdefghijklmnopqrstuvwxyz";

    for c in chars.chars() {

        let stripped: String = input.chars()
            .filter(|v| *v != c && *v != c.to_ascii_uppercase())
            .collect::<String>();

        let res = reduce(stripped).len();

        results.insert(c, res);
    }

    let mut min: usize = 12000;

    for (_, len) in results {

        if len < min { min = len }
    }

    min.to_string()
}
