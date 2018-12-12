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

fn parse_instructions(input: &str) {

}

fn main() {

    let input_string = include_str!("input.txt");

    println!("{:?}", input_string);

    // let dependency_edges: Vec<(Vertex, Vertex)> = input_string.trim().lines().map(parse_instructions).collect();

    // list of all vertices in the graph
    let vertices: Vertices = Vertices::new();
    // list of all vertices in the graph that have no pre-requisites
    let root_vertices: Vertices = Vertices::new();
    let edges: Edges = HashMap::new();

    // the min-heap always ensures available work is ordered alphabetically
    let mut work_queue: BinaryHeap<Vertex> = BinaryHeap::new();

    // TODO: add root into work queue

    let mut work_order: Vec<String> = vec![];

    while let Some(current_work) = work_queue.pop() {
        let Vertex(name) = &current_work;
        work_order.push(name.to_string());

        // get all vertices adjacent to current_work, and add them to the work_queue
        let vertices = {
            edges.get(&current_work).unwrap();
            vertices.get_vertices()
        };

        work_queue.extend(vertices.into_iter());
    }

    let work_order: String = work_order.join("");

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

}
