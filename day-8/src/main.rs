// https://adventofcode.com/2018/day/8

struct Node {
    num_of_child_nodes: i32,
    num_of_meta_entries: i32,

    children: Vec<Node>,

    metadata: Vec<i32>,
}

fn main() {
    let input_string = include_str!("input.txt");

    let iter = input_string.trim().split_whitespace().map(|x| -> i32 {
        return x.trim().parse().unwrap();
    });

    for input in iter {
        println!("{:?}", input);
    }
}
