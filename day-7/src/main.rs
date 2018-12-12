// https://adventofcode.com/2018/day/7

// imports

use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;

// types

#[derive(PartialEq, Hash, Eq, Clone, Debug)]
struct Vertex(String);

impl PartialOrd for Vertex {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        other.0.partial_cmp(&self.0)
    }
}

impl Ord for Vertex {
    fn cmp(&self, other: &Vertex) -> Ordering {
        let ord = self.partial_cmp(other).unwrap();
        match ord {
            Ordering::Greater => Ordering::Less,
            Ordering::Less => Ordering::Greater,
            Ordering::Equal => ord,
        }
    }
}

struct Vertices {
    set: HashSet<Vertex>,
    order: BinaryHeap<Vertex>,
}

impl Vertices {
    fn new() -> Vertices {
        Vertices {
            set: HashSet::new(),
            order: BinaryHeap::new(),
        }
    }

    fn pop(&mut self) -> Option<Vertex> {
        let popped = self.order.pop();

        match popped {
            None => None,
            Some(popped) => {
                self.set.remove(&popped);
                return Some(popped);
            }
        }
    }

    fn get_vertices(&self) -> HashSet<Vertex> {
        return self.set.clone();
    }

    fn has_vertex(&self, vertex: &Vertex) -> bool {
        return self.set.contains(vertex);
    }

    fn add_vertex(&mut self, vertex: Vertex) {
        if self.has_vertex(&vertex) {
            return;
        }

        self.set.insert(vertex.clone());
        self.order.push(vertex);
    }
}

type Edges = HashMap<Vertex, Vertices>;

fn parse_instructions(input: &str) -> (Vertex, Vertex) {
    let tokens: Vec<&str> = input.split_whitespace().collect();

    let first = tokens.get(1).unwrap();
    let second = tokens.get(7).unwrap();

    return (Vertex(first.to_string()), Vertex(second.to_string()));
}

fn part_1(input_string: &str) -> String {
    let dependency_edges: Vec<(Vertex, Vertex)> = input_string
        .trim()
        .lines()
        .map(parse_instructions)
        .collect();

    // list of all vertices in the graph that have no pre-requisites
    let mut root_vertices: HashSet<Vertex> = HashSet::new();
    // list of direct edges mapping one vertex to a set of vertices
    let mut edges: Edges = HashMap::new();
    // vertices that have remaining work (vertices that need to be visited)
    let mut remaining_work: HashMap<Vertex, HashSet<Vertex>> = HashMap::new();

    for (maybe_root, _) in &dependency_edges {
        root_vertices.insert(maybe_root.clone());
    }

    for (first, second) in &dependency_edges {
        root_vertices.remove(second);

        edges
            .entry(first.clone())
            .and_modify(|x| {
                x.add_vertex(second.clone());
            })
            .or_insert_with(|| {
                let mut x = Vertices::new();
                x.add_vertex(second.clone());
                x
            });

        remaining_work
            .entry(second.clone())
            .and_modify(|x| {
                x.insert(first.clone());
            })
            .or_insert_with(|| {
                let mut x = HashSet::new();
                x.insert(first.clone());
                x
            });
    }

    // the min-heap always ensures available work is ordered alphabetically
    let mut work_queue = Vertices::new();

    // add roots into work queue
    for vertex in root_vertices {
        work_queue.add_vertex(vertex);
    }

    let mut work_order: Vec<String> = vec![];

    while let Some(current_work) = work_queue.pop() {
        // perform work 🛠️

        let Vertex(name) = &current_work;
        work_order.push(name.to_string());

        // println!("visiting {}", name);

        // get all vertices adjacent to current_work, and add them to the work_queue

        let adjacent_vertices = edges.get(&current_work);

        match adjacent_vertices {
            None => {}
            Some(adjacent_vertices) => {
                let vertices = adjacent_vertices.get_vertices();

                for adjacent_vertex in vertices.into_iter() {
                    // for each adjacent vertex, remove current work from their set of remaining work

                    if remaining_work.contains_key(&adjacent_vertex) {
                        let mut should_delete = false;

                        remaining_work
                            .entry(adjacent_vertex.clone())
                            .and_modify(|x| {
                                x.remove(&current_work);

                                should_delete = x.is_empty();
                            });

                        if should_delete {
                            // adjacent vertex has no remaining work left, add it to the work queue

                            remaining_work.remove(&adjacent_vertex);
                            work_queue.add_vertex(adjacent_vertex);
                        }
                    }
                }
            }
        }
    }

    let work_order: String = work_order.join("");

    return work_order;
}

fn main() {
    let input_string = include_str!("input.txt");

    let work_order = part_1(input_string);

    println!("Part 1: {}", work_order);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_binary_heap() {
        let mut heap: BinaryHeap<Vertex> = BinaryHeap::new();

        heap.push(Vertex("Z".to_string()));
        heap.push(Vertex("A".to_string()));
        heap.push(Vertex("B".to_string()));

        assert_eq!(heap.pop(), Some(Vertex("A".to_string())));
        assert_eq!(heap.pop(), Some(Vertex("B".to_string())));
        assert_eq!(heap.pop(), Some(Vertex("Z".to_string())));
    }

    #[test]
    fn test_part_1() {
        let input = r###"
Step C must be finished before step A can begin.
Step C must be finished before step F can begin.
Step A must be finished before step B can begin.
Step A must be finished before step D can begin.
Step B must be finished before step E can begin.
Step D must be finished before step E can begin.
Step F must be finished before step E can begin.
        "###;

        assert_eq!(part_1(input), "CABDFE".to_string());
    }

}
