use proconio::input;

pub type WeightedGraph = [Vec<usize>];

pub fn shortest_path(
  graph: &WeightedGraph,
  start: usize,
  res: &mut std::collections::HashMap<usize, usize>,
) {
  let mut visited = std::collections::HashSet::new();
  let mut que = std::collections::VecDeque::new();
  visited.insert(start);
  que.push_back((start, 0));
  while let Some((u, cost)) = que.pop_front() {
    for &v in &graph[u] {
      if visited.contains(&v) {
        continue;
      }
      visited.insert(v);
      let new_cost = cost + 1;
      let x = res.entry(new_cost).or_insert(0);
      *x += 1;
      que.push_back((v, new_cost));
    }
  }
}

fn main() {
  input! {
    n: usize, x: usize, y: usize
  };

  let mut graph = vec![Vec::new(); n];
  for i in 0..(n - 1) {
    graph[i].push(i + 1);
    graph[i + 1].push(i);
  }
  graph[x - 1].push(y - 1);
  graph[y - 1].push(x - 1);

  // println!("{:?}", graph);
  let mut res = std::collections::HashMap::new();
  for i in 0..n {
    shortest_path(&graph, i, &mut res);
  }
  for i in 1..n {
    let x = res.get(&i).unwrap_or(&0) / 2;
    println!("{}", x);
  }
}
