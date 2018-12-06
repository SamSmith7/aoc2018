use std::fs::File;
use std::io::prelude::*;
use std::cmp::Ordering;
use std::collections::HashMap;

#[derive(Debug)]
enum Action {
    Start(i32),
    Sleep,
    Wake
}

fn get_action(input: &str) -> Action {

    if input.contains("wake") { return Action::Wake }

    if input.contains("fall") {
        return Action::Sleep
    } else {

        let split: Vec<&str> = input.split(|v| v == ' ' || v == '#').collect();
        Action::Start(split[3].parse::<i32>().unwrap())
    }

}

#[derive(Debug)]
struct Date {
    year: i32,
    month: i32,
    day: i32,
    hour: i32,
    min: i32
}

impl Date {
    fn new(input: &str) -> Date {

        let parts: Vec<&str> = input.split(|v| { v == ' ' }).collect();
        let date: Vec<i32> = parts[0]
            .split(|v| v == '-')
            .map(|v| v.parse::<i32>().unwrap())
            .collect();

        let time: Vec<i32> = parts[1]
            .split(|v| v == ':')
            .map(|v| v.parse::<i32>().unwrap())
            .collect();

        Date {
            year: date[0],
            month: date[1],
            day: date[2],
            hour: time[0],
            min: time[1]
        }
    }
}

impl Ord for Date {
    fn cmp(&self, other: &Date) -> Ordering {
        self.year.cmp(&other.year)
            .then(self.month.cmp(&other.month))
            .then(self.day.cmp(&other.day))
            .then(self.hour.cmp(&other.hour))
            .then(self.min.cmp(&other.min))
    }
}

impl PartialOrd for Date {
    fn partial_cmp(&self, other: &Date) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for Date {}

impl PartialEq for Date {
    fn eq(&self, other: &Date) -> bool {
        self.year == other.year
            && self.month == other.month
            && self.day == other.day
            && self.hour == other.hour
            && self.min == other.min
    }
}

#[derive(Debug)]
struct Entry {
    date: Date,
    action: Action
}

impl Entry {
    fn new(input: &str) -> Entry {

        let parts: Vec<&str> = input
            .split(|v| { v == '[' || v == ']' })
            .collect();

        Entry {
            date: Date::new(parts[1]),
            action: get_action(parts[2])
        }
    }
}


fn get_data() -> Vec<Entry> {

    let mut f = File::open("./src/day4/input.txt").expect("File not Found");
    let mut contents = String::new();

    f.read_to_string(&mut contents)
        .expect("Error reading file");

    let mut entries: Vec<Entry> = contents.split("\n")
        .filter(|value| value != &"")
        .map(|value| Entry::new(value))
        .collect();

    entries.sort_by(|a, b| a.date.cmp(&b.date));

    entries

}

fn get_sleep_patterns(data: Vec<Entry>) -> HashMap<i32, Vec<i32>> {

    let mut current_guard: i32 = 0;
    let mut start_sleep: usize = 0;
    let mut sleep_patterns = HashMap::new();

    for entry in data {

        match entry.action {
            Action::Start(id) => { current_guard = id }
            Action::Sleep => { start_sleep = entry.date.min as usize }
            Action::Wake => {

                let guard = sleep_patterns.entry(current_guard)
                    .or_insert(vec![0; 59]);

                for idx in start_sleep..entry.date.min as usize {
                    guard[idx] += 1
                }
            }
        }
    }

    sleep_patterns
}

pub fn part1() -> String {

    let data = get_data();

    let sleep_patterns = get_sleep_patterns(data);

    let mut max = 0;
    let mut max_id: i32 = 0;
    let mut max_minute: i32 = 0;

    for (id, sleep) in &sleep_patterns {

        let sum = sleep.into_iter().sum();

        if sum > max {
            max = sum;
            max_id = *id;
            let max_index = sleep.into_iter().enumerate().max_by_key(|&(_, item)| item).unwrap();
            max_minute = max_index.0 as i32;
        }
    }

    let res = max_id * max_minute;

    res.to_string()
}

pub fn part2() -> String {

    let data = get_data();

    let sleep_patterns = get_sleep_patterns(data);

    let mut max = 0;
    let mut max_id: i32 = 0;
    let mut max_minute: i32 = 0;

    for (id, sleep) in &sleep_patterns {

        let max_index = sleep.into_iter().enumerate().max_by_key(|&(_, item)| item).unwrap();

        if *max_index.1 > max {
            max = *max_index.1;
            max_id = *id;
            max_minute = max_index.0 as i32;
        }
    }

    let res = max_id * max_minute;

    res.to_string()
}
