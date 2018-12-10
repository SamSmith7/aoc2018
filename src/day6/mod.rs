use std::fs::File;
use std::io::prelude::*;
use std::collections::HashSet;


fn get_data() -> Vec<(isize, isize)> {
    let mut f = File::open("./src/day6/input.txt").expect("File not Found");

    let mut contents = String::new();

    f.read_to_string(&mut contents)
        .expect("Error reading file");

    contents.split("\n")
        .filter(|v| v != &"")
        .map(|coord| {

            let split: Vec<isize> = coord.split(", ")
                .map(|v| v.parse::<isize>().unwrap())
                .collect();

            (split[0], split[1])
        })
        .collect()
}

fn get_distance(a: (isize, isize), b: (isize, isize)) -> isize {

    (b.0 - a.0).abs() + (b.1 - a.1).abs()
}

fn get_edges(area: &Vec<usize>, width: isize, height: isize) -> HashSet<usize> {

    let mut infinite = HashSet::new();
    let offset = ((width * height) - width) as usize;
    let u_width = width as usize;
    let u_height = height as usize;

    for i in 0..u_width {
        infinite.insert(area[i]);
        infinite.insert(area[i + offset]);
    }

    for i in 0..u_height {
        let left = i * u_width;
        infinite.insert(area[left + (u_width - 1)]);
        infinite.insert(area[left]);
    }

    infinite
}


pub fn part1() -> String {

    let coords = get_data();
    let width = 500;
    let height = 500;
    let len = (width * height) as usize;
    let mut area = vec![0; len];

    for idx in 0..len {

        let new_idx = idx as isize;
        let mut closest_id = 0;
        let mut closest_distance = 1000;
        let coord = (new_idx % height, new_idx / height);

        for (id, point2) in coords.iter().enumerate() {

            let dist = get_distance(coord, *point2);

            if dist == closest_distance {
                closest_id = 1000000;
            }

            if dist < closest_distance {
                closest_distance = dist;
                closest_id = id;
            }
        }

        area[idx] = closest_id;
    }

    let infinite = get_edges(&area, width, height);
    let mut finite = HashSet::new();

    for (id, _) in coords.iter().enumerate() {
        if !infinite.contains(&id) {
            finite.insert(id);
        }
    }

    let mut max_count = 0;

    for id in finite {

        let count = area.iter()
            .filter(|&&v| v == id)
            .count();

        if count > max_count { max_count = count; }
    }

    max_count.to_string()
}

pub fn part2() -> String {

    let coords = get_data();
    let width = 500;
    let height = 500;
    let len = (width * height) as usize;
    let mut count = 0;

    for idx in 0..len {

        let new_idx = idx as isize;
        let mut sum = 0;
        let coord = (new_idx % height, new_idx / height);

        for (_, point2) in coords.iter().enumerate() {
            let dist = get_distance(coord, *point2);
            sum += dist;
        }

        if sum < 10000 { count += 1; }
    }

    count.to_string()
}
