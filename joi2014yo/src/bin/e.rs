use proconio::input;
use proconio::marker::Usize1;
use std::collections::{BinaryHeap, HashSet, VecDeque};

fn reachable_vertexes(graph: &Vec<Vec<usize>>, s: usize, max_c: usize) -> HashSet<usize> {
  let mut set: HashSet<usize> = HashSet::new();
  let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
  queue.push_back((s, 0));
  while let Some((u, c)) = queue.pop_front() {
    for v in &graph[u] {
      if set.contains(v) {
        continue;
      }
      let new_c = c + 1;
      if new_c > max_c {
        continue;
      }
      set.insert(*v);
      queue.push_back((*v, new_c));
    }
  }
  set
}

fn shortest_path(
  graph: &Vec<HashSet<usize>>,
  start: usize,
  cr: &Vec<(usize, usize)>,
) -> Vec<usize> {
  // dist[node] = current shortest distance from `start` to `node`
  let mut dist: Vec<_> = (0..graph.len()).map(|_| std::usize::MAX).collect();

  let mut heap = BinaryHeap::new();

  // We're at `start`, with a zero cost
  dist[start] = 0;
  heap.push(std::cmp::Reverse((0, start)));

  // Examine the frontier with lower cost nodes first (min-heap)
  while let Some(std::cmp::Reverse((cost, u))) = heap.pop() {
    // Important as we may have already found a better way
    if cost > dist[u] {
      continue;
    }

    // For each node we can reach, see if we can find a way with
    // a lower cost going through this node
    for v in &graph[u] {
      let new_cost = cost + cr[u].0;

      // If so, add it to the frontier and continue
      if new_cost < dist[*v] {
        heap.push(std::cmp::Reverse((new_cost, *v)));
        // Relaxation, we have now found a better way
        dist[*v] = new_cost;
      }
    }
  }
  dist
}

fn main() {
  input! {
    n: usize,
    k: usize,
    cr: [(usize, usize); n],
    ab: [(Usize1, Usize1); k]
  };

  let mut graph = Vec::new();
  for _ in 0..n {
    graph.push(Vec::new());
  }
  for (a, b) in ab {
    graph[a].push(b);
    graph[b].push(a);
  }

  let mut reachable = Vec::new();
  for i in 0..n {
    let (_, r) = cr[i];
    reachable.push(reachable_vertexes(&graph, i, r));
  }
  let d = shortest_path(&reachable, 0, &cr);
  let ans = d[n - 1];
  println!("{}", ans);
}
