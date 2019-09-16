extern crate aoc2018;

use std::env;
use aoc2018::day1;
use aoc2018::day2;
use aoc2018::day3;
use aoc2018::day4;
use aoc2018::day5;
use aoc2018::day6;
use aoc2018::day7;
use aoc2018::day8;


struct AocResult {
    part1: String,
    part2: String
}

impl AocResult {
    fn format(&self) -> String<> {
        format!("Part 1: {}, Part 2: {}", self.part1, self.part2)
    }
}

fn main() {

    let args: Vec<String> = env::args().collect();

    let day = args.get(1).unwrap();

    let result = match day.as_ref() {
        "day1" => {
            AocResult {
                part1: day1::part1(),
                part2: day1::part2()
            }
        },
        "day2" => {
            AocResult {
                part1: day2::part1(),
                part2: day2::part2()
            }
        },
        "day3" => {
            AocResult {
                part1: day3::part1(),
                part2: day3::part2()
            }
        },
        "day4" => {
            AocResult {
                part1: day4::part1(),
                part2: day4::part2()
            }
        },
        "day5" => {
            AocResult {
                part1: day5::part1(),
                part2: day5::part2()
            }
        },
        "day6" => {
            AocResult {
                part1: day6::part1(),
                part2: day6::part2()
            }
        },
        "day7" => {
            AocResult {
                part1: day7::part1(),
                part2: day7::part2()
            }
        },
        "day8" => {
            AocResult {
                part1: day8::part1(),
                part2: day8::part2()
            }
        },
        _ => AocResult {
            part1: String::new(),
            part2: String::new()
        }
    };


    println!("{:?}", result.format());
}
