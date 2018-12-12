// https://adventofcode.com/2018/day/7

// imports

use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;
use std::iter::FromIterator;

// types

#[derive(PartialEq, Hash, Eq, Clone, Debug)]
struct Vertex(char);

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

    fn len(&self) -> usize {
        return self.set.len();
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

    return (
        Vertex(first.chars().next().unwrap()),
        Vertex(second.chars().next().unwrap()),
    );
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

fn parse_to_work_load(x: char, base_workload: i32) -> i32 {
    return x as i32 - 65 + 1 + base_workload;
}

type RemainingWork = i32;

// NOTE: RemainingWork is the amount of seconds required to complete the task identified by Vertex
#[derive(PartialEq, Hash, Eq, Clone, Debug)]
struct WorkTask(Vertex, RemainingWork);

impl WorkTask {
    fn from_vertex(vertex: Vertex, base_workload: i32) -> WorkTask {
        let Vertex(name) = &vertex;

        let work_load = parse_to_work_load(*name, base_workload);

        WorkTask(vertex, work_load)
    }

    fn to_vertex(self) -> Vertex {
        self.0
    }

    fn remaining_work(&self) -> RemainingWork {
        return self.1;
    }

    fn progress_work(&mut self, progress_work: RemainingWork) {
        self.1 = self.1 - progress_work;
    }
}

impl PartialOrd for WorkTask {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        other.1.partial_cmp(&self.1)
    }
}

impl Ord for WorkTask {
    fn cmp(&self, other: &WorkTask) -> Ordering {
        let ord = self.partial_cmp(other).unwrap();
        match ord {
            Ordering::Greater => Ordering::Less,
            Ordering::Less => Ordering::Greater,
            Ordering::Equal => ord,
        }
    }
}

fn part_2(input_string: &str, base_workload: i32, max_worker_limit: i32) -> i32 {
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

    // how long it took to complete all the tasks
    let mut duration = 0;

    // min-heap with WorkTask items sorted from smallest remaining work to the largest
    let mut work_in_progress: BinaryHeap<WorkTask> = BinaryHeap::new();

    loop {
        if remaining_work.len() <= 0 && work_queue.len() <= 0 {
            break;
        }

        // assign any available work to any available workers

        while (work_in_progress.len() as i32) < max_worker_limit && work_queue.len() > 0 {
            let current_work = work_queue.pop().unwrap();
            work_in_progress.push(WorkTask::from_vertex(current_work, base_workload));
        }

        // get work task(s) that can complete first

        let mut completed_work: Vec<WorkTask> = vec![];

        // invariant: work_in_progress is non-empty
        assert!(!work_in_progress.is_empty());

        let task = work_in_progress.pop().unwrap();
        let min_remaining_work = task.remaining_work();
        completed_work.push(task);

        // find any work task(s) that can also be completed simultaneously
        while let Some(task) = work_in_progress.peek() {
            if task.remaining_work() <= min_remaining_work {
                let task = work_in_progress.pop().unwrap();
                completed_work.push(task);
            } else {
                break;
            }
        }

        duration += min_remaining_work;

        {
            // for each remaining work tasks in work_in_progress,
            // min_remaining_work seconds would have occurred.
            // shave min_remaining_work off for each task in work_in_progress

            let tasks = work_in_progress.into_vec().into_iter().map(|mut task| {
                // invariant: work_in_progress contain work tasks that have remaining work that is greater than min_remaining_work
                assert!(task.remaining_work() > min_remaining_work);

                task.progress_work(min_remaining_work);
                task
            });

            work_in_progress = BinaryHeap::from_iter(tasks);
        };

        // for each completed_work, get their adjacent vertices,
        // and add them to the work_queue only if their remaining work is completed

        for current_work_task in completed_work {
            let current_work = current_work_task.to_vertex();

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
    }

    return duration;
}

fn main() {
    let input_string = include_str!("input.txt");

    let work_order = part_1(input_string);

    println!("Part 1: {}", work_order);

    let base_workload = 60;
    let max_worker_limit = 5;
    let duration = part_2(input_string, base_workload, max_worker_limit);

    println!("Part 2: {}", duration);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vertex_binary_heap() {
        let mut heap: BinaryHeap<Vertex> = BinaryHeap::new();

        heap.push(Vertex('Z'));
        heap.push(Vertex('A'));
        heap.push(Vertex('B'));

        assert_eq!(heap.pop(), Some(Vertex('A')));
        assert_eq!(heap.pop(), Some(Vertex('B')));
        assert_eq!(heap.pop(), Some(Vertex('Z')));
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

    #[test]
    fn test_parse_to_work_load() {
        let base_workload = 60;

        assert_eq!(parse_to_work_load('A', base_workload), 61);
        assert_eq!(parse_to_work_load('B', base_workload), 62);
        assert_eq!(parse_to_work_load('C', base_workload), 63);
        assert_eq!(parse_to_work_load('Z', base_workload), 86);
    }

    #[test]
    fn test_worktask_binary_heap() {
        let base_workload = 60;

        let mut heap: BinaryHeap<WorkTask> = BinaryHeap::new();

        heap.push(WorkTask::from_vertex(Vertex('B'), base_workload));
        heap.push(WorkTask::from_vertex(Vertex('Z'), base_workload));
        heap.push(WorkTask::from_vertex(Vertex('A'), base_workload));

        assert_eq!(
            heap.pop(),
            Some(WorkTask::from_vertex(Vertex('A'), base_workload))
        );
        assert_eq!(
            heap.pop(),
            Some(WorkTask::from_vertex(Vertex('B'), base_workload))
        );
        assert_eq!(
            heap.pop(),
            Some(WorkTask::from_vertex(Vertex('Z'), base_workload))
        );
    }

    #[test]
    fn test_part_2() {
        let input = r###"
Step C must be finished before step A can begin.
Step C must be finished before step F can begin.
Step A must be finished before step B can begin.
Step A must be finished before step D can begin.
Step B must be finished before step E can begin.
Step D must be finished before step E can begin.
Step F must be finished before step E can begin.
        "###;

        let base_workload = 0;
        let max_worker_limit = 2;

        assert_eq!(part_2(input, base_workload, max_worker_limit), 15);
    }

}
