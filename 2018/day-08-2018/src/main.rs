// https://adventofcode.com/2018/day/8

#[derive(Debug)]
struct Node {
    children: Vec<Node>,

    metadata: Vec<i32>,
}

impl Node {
    fn part_2_get_value(&self) -> i32 {
        if self.children.is_empty() {
            return self.metadata.iter().sum();
        }

        let mut total_value = 0;

        for nth_child in &self.metadata {
            let index = *nth_child - 1;

            if index < 0 {
                continue;
            }

            let child_node = self.children.get(index as usize);

            if child_node.is_none() {
                continue;
            }

            total_value += child_node.unwrap().part_2_get_value();
        }

        total_value
    }
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

    children
}

fn part_1(input_string: &str) -> i32 {
    let mut iter = input_string.split_whitespace().map(|x| -> i32 {
        return x.trim().parse().unwrap();
    });

    let root_node = parse_node(&mut iter);

    let mut total: i32 = 0;

    let mut stack = vec![root_node];

    while let Some(node) = stack.pop() {
        let metadata_sum: i32 = node.metadata.iter().sum();
        total += metadata_sum;

        stack.extend(node.children);
    }

    total
}

fn part_2(input_string: &str) -> i32 {
    let mut iter = input_string.split_whitespace().map(|x| -> i32 {
        return x.trim().parse().unwrap();
    });

    let root_node = parse_node(&mut iter);

    root_node.part_2_get_value()
}

fn main() {
    let input_string = include_str!("input.txt");

    let total = part_1(input_string);
    println!("Part 1: {}", total);

    let root_node_value = part_2(input_string);
    println!("Part 2: {}", root_node_value);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input_string = r###"
2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2
        "###;

        assert_eq!(part_1(input_string), 138);
    }

    #[test]
    fn test_part_2() {
        let input_string = r###"
2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2
        "###;

        assert_eq!(part_2(input_string), 66);
    }
}
