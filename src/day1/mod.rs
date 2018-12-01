use std::fs::File;
use std::io::prelude::*;
use std::collections::HashSet;


fn parse_input() -> Vec<i32> {

    let mut f = File::open("./src/day1/input1.txt").expect("File not Found");
    let mut contents = String::new();

    f.read_to_string(&mut contents)
        .expect("Error reading file");

    let values: Vec<&str> = contents.split("\n").collect();

    let result: Vec<i32> = values
        .into_iter()
        .filter(|value| value != &"")
        .map(|value| {
            let num = value.parse::<i32>();
            match num {
                Ok(v) => v,
                Err(e) => 0
            }
        })
        .collect();

    result
}

pub fn part1() -> String<> {

    let result = parse_input()
        .into_iter()
        .fold(0, |acc, value| acc + value);

    result.to_string()
}

pub fn part2() -> String<> {

    let mut values = HashSet::new();

    let mut done = false;
    let mut result = 0;

    while done != true {

        let input = parse_input()
            .into_iter()
            .fold(result, |acc, value| {

                if done { return acc }

                let v = acc + value;

                if values.contains(&v) {
                    done = true;
                } else {
                    values.insert(v);
                }

                v
            });

        result = input;
    }

    result.to_string()
}
