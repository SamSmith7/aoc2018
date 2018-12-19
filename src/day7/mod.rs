use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;
use std::collections::HashSet;


pub fn part1() -> String {

    let mut f = File::open("./src/day7/input.txt").expect("File not Found");
    let mut contents = String::new();

    f.read_to_string(&mut contents)
        .expect("Error reading file");

    let lines: Vec<(&str, &str)> = contents
        .split("\n")
        .filter(|value| value != &"")
        .map(|line| {

            let split: Vec<&str> = line.split(" ")
                .collect();

            (split[1], split[7])
        })
        .collect();

    let mut tree = HashMap::new();
    let mut nodes = HashSet::new();
    let mut dependants = HashSet::new();

    for (a, b) in lines {

        let node = tree.entry(b).or_insert(vec![]);
        node.push(a);
        node.sort();
        node.dedup();
        nodes.insert(a);
        nodes.insert(b);
        dependants.insert(b);
    }

    let mut start = nodes.difference(&dependants)
        .collect::<Vec<&&str>>();

    let mut order = vec![];

    while start.len() > 0 {

        start.sort_by(|a, b| b.cmp(a));
        start.dedup_by(|a, b| a == b);

        let last = start.pop().unwrap();
        order.push(*last);

        for (node, blockers) in &tree {

            let mut count = 0;

            for blocked in blockers.iter() {
                if !order.contains(&blocked) && blocked != last {
                    count += 1;
                }
            }

            if count == 0 && !order.contains(&node) { start.push(&node); }
        }
    }

    let strings: Vec<String> = order.iter()
        .map(|v| v.to_string())
        .collect();

    strings.join("")
}


pub fn part2() -> String {

    let mut f = File::open("./src/day7/input.txt").expect("File not Found");
    let mut contents = String::new();

    f.read_to_string(&mut contents)
        .expect("Error reading file");

    let lines: Vec<(&str, &str)> = contents
        .split("\n")
        .filter(|value| value != &"")
        .map(|line| {

            let split: Vec<&str> = line.split(" ")
                .collect();

            (split[1], split[7])
        })
        .collect();

    let mut tree = HashMap::new();
    let mut nodes = HashSet::new();
    let mut dependants = HashSet::new();

    for (a, b) in lines {

        let node = tree.entry(b).or_insert(vec![]);
        node.push(a);
        node.sort();
        node.dedup();
        nodes.insert(a);
        nodes.insert(b);
        dependants.insert(b);
    }

    let mut start = nodes.difference(&dependants)
        .collect::<Vec<&&str>>();

    let mut workers = vec![(&"", 0), (&"", 0), (&"", 0), (&"", 0), (&"", 0)];
    let mut tick = 0;
    let mut remaining_time = 0;
    let mut in_progress = HashSet::new();
    let mut order = vec![];

    while start.len() > 0 || remaining_time != 0 || in_progress.len() > 0 {

        let mut new_workers = vec![];

        for (key, count) in workers.iter() {

            if *count == 0 {

                for (node, blockers) in &tree {

                    let mut still_blocked = 0;

                    for blocked in blockers.iter() {
                        if !order.contains(&blocked) && blocked != *key {
                            still_blocked += 1;
                        }
                    }

                    if still_blocked == 0 && !order.contains(&node) && !in_progress.contains(&node) {
                        start.push(&node);
                    }
                }
            }
        }

        for (key, count) in workers.iter() {

            if *count == 0 {

                if *key != &"" {
                    order.push(*key);
                    in_progress.remove(*key);
                }

                start.sort_by(|a, b| b.cmp(a));
                start.dedup_by(|a, b| a == b);

                match start.pop() {
                    Some(last) => {
                        let num = last.chars().next().unwrap() as i32;
                        in_progress.insert(last);
                        new_workers.push((last, num - 5));
                    },
                    None => {
                        new_workers.push((&"", 0));
                    }
                };
            } else {

                new_workers.push((key, count - 1));
            }
        }

        remaining_time = workers.iter().map(|(_, time)| time).sum();
        workers = new_workers;
        tick += 1;
    }

    tick.to_string()
}
