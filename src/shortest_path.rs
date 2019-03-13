use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::usize;

#[snippet = "dijkstra"]
#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    position: usize,
}

#[snippet = "dijkstra"]
impl Ord for State {
    fn cmp(&self, other: &State) -> Ordering {
        other.cost.cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

#[snippet = "dijkstra"]
impl PartialOrd for State {
    fn partial_cmp(&self, other: &State) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[snippet = "dijkstra"]
struct Edge {
    node: usize,
    cost: usize,
}

#[snippet = "dijkstra"]
#[allow(dead_code)]
fn shortest_path(adj_list: &Vec<Vec<Edge>>, start: usize, goal: usize) -> Option<usize> {
    let mut dist: Vec<_> = (0..adj_list.len()).map(|_| usize::MAX).collect();
    let mut heap = BinaryHeap::new();

    dist[start] = 0;
    heap.push(State { cost: 0, position: start });

    while let Some(State { cost, position }) = heap.pop() {
        if position == goal { return Some(cost); }
        if cost > dist[position] { continue; }

        for edge in &adj_list[position] {
            let next = State { cost: cost + edge.cost, position: edge.node };
            if next.cost < dist[next.position] {
                heap.push(next);
                dist[next.position] = next.cost;
            }
        }
    }
    None
}

#[test]
fn test_dijkstra() {
    let graph = vec![
            // Node 0
            vec![Edge { node: 2, cost: 10 },
                Edge { node: 1, cost: 1 }],
            // Node 1
            vec![Edge { node: 3, cost: 2 }],
            // Node 2
            vec![Edge { node: 1, cost: 1 },
                Edge { node: 3, cost: 3 },
                Edge { node: 4, cost: 1 }],
            // Node 3
            vec![Edge { node: 0, cost: 7 },
                Edge { node: 4, cost: 2 }],
            // Node 4
            vec![]];

    assert_eq!(shortest_path(&graph, 0, 1), Some(1));
    assert_eq!(shortest_path(&graph, 0, 3), Some(3));
    assert_eq!(shortest_path(&graph, 3, 0), Some(7));
    assert_eq!(shortest_path(&graph, 0, 4), Some(5));
    assert_eq!(shortest_path(&graph, 4, 0), None);
}
