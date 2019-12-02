// https://adventofcode.com/2019/day/2

fn main() {

    let input_string = include_str!("input.txt");

    let inputs: Vec<i32> = input_string.trim().split(',').map(|opcode| -> i32 {
        return opcode.parse().unwrap();
    }).collect();

    println!("{:?}", inputs);
}
