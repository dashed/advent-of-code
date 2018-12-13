// https://adventofcode.com/2018/day/8

#[derive(Debug)]
struct Node {
    children: Vec<Node>,

    metadata: Vec<i32>,
}

fn parse_node<I: Iterator<Item = i32>>(iter: &mut I) -> Node {
    // parse headers
    let num_of_child_nodes = iter.next().unwrap();
    let num_of_meta_entries = iter.next().unwrap();

    let children = if num_of_child_nodes > 0 {
        parse_children(iter, num_of_child_nodes)
    } else {
        vec![]
    };

    let metadata: Vec<i32> = iter.take(num_of_meta_entries as usize).collect();

    Node { children, metadata }
}

fn parse_children<I: Iterator<Item = i32>>(iter: &mut I, num_of_child_nodes: i32) -> Vec<Node> {
    if num_of_child_nodes <= 0 {
        return vec![];
    }

    let mut idx = 0;
    let mut children = vec![];

    while idx < num_of_child_nodes {
        children.push(parse_node(iter));
        idx += 1;
    }

    return children;
}

fn part_1(input_string: &str) -> i32 {
    let mut iter = input_string
        .trim()
        .split_whitespace()
        .map(|x| -> i32 {
            return x.trim().parse().unwrap();
        })
        .into_iter();

    let root_node = parse_node(&mut iter);

    let mut total: i32 = 0;

    let mut stack = vec![root_node];

    while let Some(node) = stack.pop() {
        let metadata_sum: i32 = node.metadata.iter().sum();
        total += metadata_sum;

        stack.extend(node.children);
    }

    return total;
}

fn main() {
    let input_string = include_str!("input.txt");

    let total = part_1(input_string);
    println!("Part 1: {}", total);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vertex_binary_heap() {
        let input_string = r###"
2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2
        "###;

        assert_eq!(part_1(input_string), 138);
    }

}
