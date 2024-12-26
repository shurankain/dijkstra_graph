use std::collections::BinaryHeap;
use std::cmp::Ordering;

#[derive(Eq, PartialEq)]
struct Node {
    cost: i32,
    index: usize,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn main() {
    let graph = vec![
        vec![(1, 2), (2, 5)],
        vec![(0, 2), (2, 4), (3, 7)],
        vec![(0, 5), (1, 4), (3, 3)],
        vec![(1, 7), (2, 3)],
    ];

    let dist = dijkstra(&graph, 0);
    println!("{:?}", dist);
}

// dijkstra.rs

fn dijkstra(graph: &[Vec<(usize, i32)>], start: usize) -> Vec<i32> {
    let mut heap = BinaryHeap::new();
    let mut dist = vec![i32::MAX; graph.len()];

    heap.push(Node { cost: 0, index: start });
    dist[start] = 0;

//     The active selection is part of the implementation of Dijkstra's algorithm, which is used to find the shortest paths from a starting node to all other nodes in a weighted graph. This specific code block is responsible for processing nodes in the priority queue (heap) and updating the shortest path estimates (dist).

// The while let Some(Node { cost, index }) = heap.pop() loop continues as long as there are nodes in the priority queue. Each iteration extracts the node with the smallest cost (distance) from the heap. The if cost > dist[index] check ensures that the node being processed has the most up-to-date shortest path estimate. If the cost is greater than the current known shortest distance to the node, the loop skips further processing for that node.

// The inner for loop iterates over all the neighbors of the current node (graph[index]). For each neighbor, it calculates the potential new cost (next_cost) to reach that neighbor by adding the edge weight (weight) to the current node's cost. If this new cost is less than the currently known shortest distance to the neighbor (dist[next]), the neighbor is added to the priority queue with the updated cost, and the shortest distance estimate is updated.

// This process ensures that the algorithm explores the shortest paths first and updates the shortest path estimates efficiently, ultimately resulting in the correct shortest paths from the starting node to all other nodes in the graph.
    while let Some(Node { cost, index }) = heap.pop() {
        if cost > dist[index] {
            continue;
        }

        for &(next, weight) in &graph[index] {
            let next_cost = cost + weight;
            if next_cost < dist[next] {
                heap.push(Node { cost: next_cost, index: next });
                dist[next] = next_cost;
            }
        }
    }

    dist
}