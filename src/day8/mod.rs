use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;


fn parse_input() -> Vec<i32> {

    let mut f = File::open("./src/day8/input.txt").expect("File not Found");
    let mut contents = String::new();

    f.read_to_string(&mut contents)
        .expect("Error reading file");

    let values: Vec<&str> = contents.split(" ").collect();

    values.into_iter()
        .filter(|value| value != &"")
        .map(|value| value.trim().parse::<i32>().unwrap())
        .collect()
}

#[derive(Debug)]
struct Leaf {
    parent: i32,
    children: Vec<i32>,
    meta_data: Vec<i32>,
    child_count: i32,
    meta_data_count: i32
}

impl Leaf {
    fn new(parent: i32, child_count: i32) -> Leaf {
        Leaf {
            parent: parent,
            children: vec![],
            meta_data: vec![],
            child_count: child_count,
            meta_data_count: 0
        }
    }

    fn set_meta_count(&mut self, meta_count: i32) {
        self.meta_data_count = meta_count;
    }

    fn new_child(&mut self, num: i32) {
        self.children.push(num);
    }

    fn new_meta(&mut self, num: i32) {
        self.meta_data.push(num);
    }
}

// fn get_parent_id(id: i32, &tree: &HashMap<i32, Leaf>) -> i32 {
//
//     let leaf = tree.get(&id).unwrap();
//     leaf.parent
// }

pub fn part1() -> String {

    let input = parse_input();

    let mut tree: HashMap<i32, Leaf> = HashMap::new();
    let mut current_leaf = 1;
    let mut child_count = true;
    let mut meta_count = false;
    let mut remaining_meta = 0;
    let mut remaining_children = 0;
    let mut next_leaf = 1;

    for num in input {

        if child_count == true {

            let parent: i32 = match tree.get(&current_leaf) {
                Some(leaf) => leaf.parent,
                None => 0
            };

            tree.insert(current_leaf, Leaf::new(parent, num));
            child_count = false;
            next_leaf += 1;
            remaining_children = num;
            meta_count = true;
        }


        if meta_count == true {

            let leaf = tree.entry(current_leaf).or_insert(Leaf::new(0, 0));

            leaf.set_meta_count(num);
            meta_count = false;

            if leaf.child_count == 0 {
                remaining_meta = leaf.meta_data_count;
            } else {
                child_count = true;
            }
        }

        if remaining_meta > 0 {

            let leaf = tree.entry(current_leaf).or_insert(Leaf::new(0, 0));
            leaf.new_meta(num);
        } else {

            remaining_children -= 1;
            println!("{:?}", tree);
            println!("{:?}", current_leaf);
            let leaf = tree.get(&current_leaf).unwrap();
            let parent_id = leaf.parent;
            // let parent_id = get_parent_id(current_leaf, &tree);
            // let parent = tree.get(&parent_id).unwrap();
            // let parent_id = leaf.parent;
            let parent = tree.entry(parent_id).or_insert(Leaf::new(0, 0));
            parent.new_child(current_leaf);

            if remaining_children == 0 {
                current_leaf = parent_id;
                remaining_meta = parent.meta_data_count;
            } else {
                current_leaf = next_leaf;
            }

        }
    }
    println!("{:?}", tree);
    String::new()
}

pub fn part2() -> String {

    String::new()
}
