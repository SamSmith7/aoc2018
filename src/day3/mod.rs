use std::fs::File;
use std::io::prelude::*;


#[derive(Debug)]
struct Claim {
    id: usize,
    height: usize,
    width: usize,
    x: usize,
    y: usize
}

impl Claim {
    fn new(input: &str) -> Claim {

        let parts: Vec<usize> = input
            .split(|v| {
                v == '#' || v == '@' || v == ':' || v == ',' || v == 'x'
            })
            .filter(|value| value != &"")
            .map(|part| {
                part.trim().parse::<usize>().unwrap()
            })
            .collect();

        Claim {
            id: parts[0],
            height: parts[4],
            width: parts[3],
            x: parts[1],
            y: parts[2]
        }
    }
}

fn get_claims() -> Vec<Claim> {

    let mut f = File::open("./src/day3/input.txt").expect("File not Found");
    let mut contents = String::new();

    f.read_to_string(&mut contents)
        .expect("Error reading file");

    contents
        .split("\n")
        .filter(|value| value != &"")
        .map(|value| Claim::new(value))
        .collect()
}

fn create_area(claims: &Vec<Claim>) -> Vec<usize> {

    let mut area = vec![0; 1000000];

    claims.into_iter()
        .for_each(|claim| {

            for y in claim.y..(claim.y + claim.height) {
                let idx = (y * 1000) + claim.x;
                for x in idx..(idx + claim.width) {
                    area[x] += 1;
                }
            }
        });

    area
}

fn check_area(area: &Vec<usize>, claim: &Claim) -> bool {

    let mut overlap = false;

    for y in claim.y..(claim.y + claim.height) {
        let idx = (y * 1000) + claim.x;
        for x in idx..(idx + claim.width) {
            if area[x] > 1 { overlap = true }
        }
    }

    !overlap
}

pub fn part1() -> String {

    let claims = get_claims();

    let area = create_area(&claims);

    let res = area.into_iter()
        .fold(0, |acc, val| {

            if val > 1 { return acc + 1 }
            acc
        });

    res.to_string()
}

pub fn part2() -> String {

    let claims: Vec<Claim> = get_claims();
    let area = create_area(&claims);
    let mut done = false;

    let res = claims.into_iter()
        .fold(0, |acc, claim| {

            if done { return acc }

            if check_area(&area, &claim) {
                done = true;
                return claim.id
            }

            acc
        });

    res.to_string()
}
