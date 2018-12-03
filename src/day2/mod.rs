use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;

fn parse_input() -> Vec<String> {

    let mut f = File::open("./src/day2/input.txt").expect("File not Found");
    let mut contents = String::new();

    f.read_to_string(&mut contents)
        .expect("Error reading file");

    let values: Vec<&str> = contents.split("\n").collect();

    values.into_iter()
        .filter(|value| value != &"")
        .map(|value| String::from(value))
        .collect()
}

pub fn part1() -> String {

    let values = parse_input();

    let mut threes = 0;
    let mut twos = 0;

    values.into_iter()
        .filter(|value| value != &"")
        .for_each(|value| {

            let mut letters = HashMap::new();

            value.chars().for_each(|ch| {
                let counter = letters.entry(ch).or_insert(0);
                *counter += 1;
            });

            let mut has_two = false;
            let mut has_three = false;

            for (key, val) in letters.iter() {
                if *val == 2 { has_two = true; }
                if *val == 3 { has_three = true; }
            }

            if has_two { twos += 1; }
            if has_three { threes += 1; }
        });

    (threes * twos).to_string()
}

pub fn part2() -> String {

    let values = parse_input();
    let mut done = false;
    let mut result = String::new();

    for val in values.iter() {
        for val2 in values.iter() {
            let count = val.chars().zip(val2.chars())
                .fold(0, |acc, (ch1, ch2)| {
                    if ch1 == ch2 { return acc + 1 }
                    acc
                });

            if count == val.len() - 1 {
                println!("{:?}, {:?}", val, val2);
                val.chars().zip(val2.chars())
                    .for_each(|(ch1, ch2)| {
                        if ch1 == ch2 { result.push(ch1) }
                    });
                done = true;
                break;
            }
        }
        if done == true { break; }
    }

    result
}
