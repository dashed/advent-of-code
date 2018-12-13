// https://adventofcode.com/2018/day/8

struct Node {
    children: Vec<Node>,

    metadata: Vec<i32>,
}

fn construct_tree<I: Iterator<Item = i32>>(mut iter: I) {
    // parse headers
    let num_of_child_nodes = iter.next().unwrap();
    let num_of_meta_entries = iter.next().unwrap();

    if num_of_child_nodes > 0 {}

    let metadata: i32 = iter.take(num_of_meta_entries as usize).sum();
}

fn main() {
    let input_string = include_str!("input.txt");

    let mut iter = input_string.trim().split_whitespace().map(|x| -> i32 {
        return x.trim().parse().unwrap();
    });

    construct_tree(iter);

    // let stack: Vec<State> = vec![];

    // parse headers
    // let num_of_child_nodes = iter.next().unwrap();
    // let num_of_meta_entries = iter.next().unwrap();

    // loop {

    //     // parse children
    //     if num_of_child_nodes > 0 {

    //         continue;
    //     }

    //     let metadata: Vec<i32> = iter.take(num_of_meta_entries as usize).collect();
    // }

    // for input in iter {
    //     println!("{:?}", input);
    // }
}
