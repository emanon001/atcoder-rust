use cargo_snippet::snippet;

// graph

#[snippet("graph")]
#[derive(Clone)]
pub struct Graph {
    graph: WeightedGraph,
}

#[snippet("graph")]
pub type Edge = (usize, usize);

#[snippet("graph")]
impl Graph {
    pub fn new(edges: &[Edge], vc: usize) -> Self {
        let edges = Self::to_weighted_edges(&edges);
        let graph = WeightedGraph::new(&edges, vc);
        Graph { graph }
    }

    pub fn new_directed(edges: &[Edge], vc: usize) -> Self {
        let edges = Self::to_weighted_edges(&edges);
        let graph = WeightedGraph::new_directed(&edges, vc);
        Graph { graph }
    }

    pub fn add_directed_edge(&mut self, e: Edge) {
        self.graph.add_directed_edge((e.0, e.1, 1));
    }

    pub fn add_edge(&mut self, e: Edge) {
        self.graph.add_edge((e.0, e.1, 1));
    }

    pub fn shortest_path(&self, start: usize) -> Vec<Option<i64>> {
        self.graph.shortest_path_1(start)
    }

    pub fn to_weighted_graph(&self) -> WeightedGraph {
        self.graph.clone()
    }

    pub fn vertex_count(&self) -> usize {
        self.graph.vertex_count()
    }

    fn to_weighted_edges(edges: &[Edge]) -> Vec<WeightedEdge> {
        edges
            .into_iter()
            .map(|(u, v)| (*u, *v, 1))
            .collect::<Vec<_>>()
    }
}

#[snippet("graph")]
#[derive(Clone)]
pub struct WeightedGraph {
    graph: Vec<Vec<(usize, i64)>>,
    vc: usize,
}

#[snippet("graph")]
pub type WeightedEdge = (usize, usize, i64);

#[snippet("graph")]
impl WeightedGraph {
    const INF: i64 = 1 << 60;

    pub fn new(edges: &[WeightedEdge], vc: usize) -> Self {
        let mut graph = vec![Vec::new(); vc];
        for &(u, v, w) in edges {
            graph[u].push((v, w));
            graph[v].push((u, w));
        }
        Self { graph, vc }
    }

    pub fn new_directed(edges: &[WeightedEdge], vc: usize) -> Self {
        let mut graph = vec![Vec::new(); vc];
        for &(u, v, w) in edges {
            graph[u].push((v, w));
        }
        Self { graph, vc }
    }

    pub fn add_directed_edge(&mut self, e: WeightedEdge) {
        self.graph[e.0].push((e.1, e.2));
    }

    pub fn add_edge(&mut self, e: WeightedEdge) {
        self.graph[e.0].push((e.1, e.2));
        self.graph[e.1].push((e.0, e.2));
    }

    pub fn bellman_ford(&self, s: usize) -> Option<Vec<Option<i64>>> {
        let vc = self.vc;
        let inf = Self::INF;
        let mut cost_list = vec![inf; vc];
        cost_list[s] = 0;
        for c in 0..vc {
            for u in 0..vc {
                for &(v, w) in &self.graph[u] {
                    if cost_list[u] == inf {
                        continue;
                    }
                    let new_cost = cost_list[u] + w;
                    if new_cost < cost_list[v] {
                        cost_list[v] = new_cost;
                        if c == vc - 1 {
                            return None;
                        }
                    }
                }
            }
        }
        Some(self.optionalize(cost_list))
    }

    pub fn prim(&self) -> i64 {
        let mut used = std::collections::HashSet::new();
        let mut heap = std::collections::BinaryHeap::new();

        let mut res = 0_i64;
        heap.push(std::cmp::Reverse((0_i64, 0)));
        while let Some(std::cmp::Reverse((weight, u))) = heap.pop() {
            if used.contains(&u) {
                continue;
            }
            used.insert(u);
            res += weight;
            for &(v, w) in &self.graph[u] {
                if used.contains(&v) {
                    continue;
                }
                heap.push(std::cmp::Reverse((w, v)));
            }
        }
        res
    }

    pub fn reachable_vertexes(&self, s: usize) -> std::collections::HashSet<usize> {
        let mut visited = std::collections::HashSet::new();
        let mut queue = std::collections::VecDeque::new();
        visited.insert(s);
        queue.push_back(s);
        while let Some(u) = queue.pop_front() {
            for &(v, _) in &self.graph[u] {
                if visited.contains(&v) {
                    continue;
                }
                visited.insert(v);
                queue.push_back(v);
            }
        }
        visited
    }

    pub fn rev(&self) -> WeightedGraph {
        let mut edges = Vec::new();
        for u in 0..self.vc {
            for &(v, w) in &self.graph[u] {
                edges.push((v, u, w));
            }
        }
        Self::new_directed(&edges, self.vc)
    }

    pub fn shortest_path(&self, start: usize) -> Vec<Option<i64>> {
        let mut cost_list = vec![Self::INF; self.vc];
        let mut heap = std::collections::BinaryHeap::new();

        cost_list[start] = 0;
        heap.push(std::cmp::Reverse((0_i64, start)));

        while let Some(std::cmp::Reverse((cost, u))) = heap.pop() {
            if cost > cost_list[u] {
                continue;
            }
            for &(v, w) in &self.graph[u] {
                let new_cost = cost + w;
                if new_cost < cost_list[v] {
                    heap.push(std::cmp::Reverse((new_cost, v)));
                    cost_list[v] = new_cost;
                }
            }
        }
        self.optionalize(cost_list)
    }

    pub fn shortest_path_1(&self, start: usize) -> Vec<Option<i64>> {
        let mut cost_list = vec![None; self.vc];
        let mut que = std::collections::VecDeque::new();
        cost_list[start] = Some(0);
        que.push_back(start);
        while let Some(u) = que.pop_front() {
            for &(v, w) in &self.graph[u] {
                if w != 1 {
                    panic!("weight is not 1");
                }
                if cost_list[v].is_some() {
                    continue;
                }
                let new_cost = cost_list[u].unwrap() + 1;
                cost_list[v] = Some(new_cost);
                que.push_back(v);
            }
        }
        cost_list
    }

    pub fn shortest_path_01(&self, start: usize) -> Vec<Option<i64>> {
        let mut cost_list = vec![Self::INF; self.vc];
        let mut que = std::collections::VecDeque::new();

        cost_list[start] = 0;
        que.push_front((start, 0_i64));

        while let Some((u, cost)) = que.pop_front() {
            if cost > cost_list[u] {
                continue;
            }
            for &(v, w) in &self.graph[u] {
                if w != 0 && w != 1 {
                    panic!("weight is not 01");
                }
                let new_cost = cost + w;
                if new_cost < cost_list[v] {
                    cost_list[v] = new_cost;
                    if w == 0 {
                        que.push_front((v, new_cost));
                    } else {
                        que.push_back((v, new_cost));
                    }
                }
            }
        }
        self.optionalize(cost_list)
    }

    pub fn traveling_salesman(&self, start: usize) -> i64 {
        let mut dp = vec![vec![None; self.vc]; 1 << self.vc];
        let fin = (1 << self.vc) - 1;
        self.traveling_salesman_impl(0, start, &mut dp, start, fin)
    }

    fn traveling_salesman_impl(
        &self,
        state: usize,
        u: usize,
        dp: &mut [Vec<Option<i64>>],
        start: usize,
        fin: usize,
    ) -> i64 {
        if let Some(res) = dp[state][u] {
            return res;
        }
        if state == fin && u == start {
            dp[state][u] = Some(0);
            return 0;
        }
        let mut res = Self::INF;
        for &(v, cost) in &self.graph[u] {
            let new_state = state | (1 << v);
            if new_state != state {
                let cost = self.traveling_salesman_impl(new_state, v, dp, start, fin) + cost;
                res = res.min(cost);
            }
        }
        dp[state][u] = Some(res);
        res
    }

    pub fn vertex_count(&self) -> usize {
        self.vc
    }

    pub fn warshall_floyd(&self) -> Vec<Vec<Option<i64>>> {
        let inf = Self::INF;
        let vc = self.vc;
        let mut cost_list = vec![vec![inf; vc]; vc];
        for u in 0..vc {
            for &(v, w) in &self.graph[u] {
                cost_list[u][v] = w;
            }
        }
        for i in 0..vc {
            cost_list[i][i] = 0;
        }
        for k in 0..vc {
            for i in 0..vc {
                for j in 0..vc {
                    cost_list[i][j] =
                        std::cmp::min(cost_list[i][j], cost_list[i][k] + cost_list[k][j]);
                }
            }
        }
        cost_list
            .into_iter()
            .map(|v| self.optionalize(v))
            .collect::<Vec<_>>()
    }

    fn optionalize(&self, v: Vec<i64>) -> Vec<Option<i64>> {
        v.into_iter()
            .map(|x| if x == Self::INF { None } else { Some(x) })
            .collect::<Vec<_>>()
    }
}

// grid

#[snippet("grid")]
#[snippet(include = "graph")]
pub struct Grid {
    grid: Vec<Vec<char>>,
    h: usize,
    w: usize,
    ng_char: char,
}

#[snippet("grid")]
impl Grid {
    pub fn new(grid: &[Vec<char>], ng_char: char) -> Self {
        assert!(grid.len() > 0);
        let grid = grid.into_iter().cloned().collect::<Vec<_>>();
        let h = grid.len();
        let w = grid[0].len();
        Self {
            grid,
            h,
            w,
            ng_char,
        }
    }

    pub fn to_graph<F>(&self, generator: F) -> WeightedGraph
    where
        F: Fn(&Grid, usize, usize) -> Vec<GridDestination>,
    {
        let mut edges = Vec::new();
        for i in 0..self.h {
            for j in 0..self.w {
                let from = self.vertex(i, j);
                for (pos, w) in generator(&self, i, j) {
                    let to = self.vertex(pos.0, pos.1);
                    edges.push((from, to, w));
                }
            }
        }
        WeightedGraph::new_directed(&edges, self.h * self.w)
    }

    pub fn height(&self) -> usize {
        self.h
    }

    pub fn width(&self) -> usize {
        self.w
    }

    pub fn in_grid(&self, i: isize, j: isize) -> bool {
        i >= 0 && i < self.h as isize && j >= 0 && j < self.w as isize
    }

    pub fn cell(&self, i: usize, j: usize) -> char {
        self.grid[i][j]
    }

    pub fn ng_char(&self) -> char {
        self.ng_char
    }

    pub fn vertex(&self, i: usize, j: usize) -> usize {
        i * self.w + j
    }
}

/// 上下左右 (i, j)
#[snippet("grid")]
pub const UDLR_DIRS: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

/// 上下左右 + 斜め (i, j)
#[snippet("grid")]
pub const ALL_DIRS: [(isize, isize); 8] = [
    // 時計回り
    (-1, 0),  // 上
    (-1, 1),  // 右上
    (0, 1),   // 右
    (1, 1),   // 右下
    (1, 0),   // 下
    (1, -1),  // 左下
    (0, -1),  // 左
    (-1, -1), // 左上
];

#[snippet("grid")]
pub type GridPos = (usize, usize);

#[snippet("grid")]
pub type GridDestination = (GridPos, i64);

#[snippet("grid")]
pub fn gen_grid_destinations(
    grid: &Grid,
    i: usize,
    j: usize,
    directions: &[(isize, isize)],
) -> Vec<GridDestination> {
    let mut dest = Vec::new();
    if grid.cell(i, j) == grid.ng_char() {
        return dest;
    }
    for &(di, dj) in directions {
        let new_i = i as isize + di;
        let new_j = j as isize + dj;
        if !grid.in_grid(new_i, new_j) {
            continue;
        }
        let new_i = new_i as usize;
        let new_j = new_j as usize;
        if grid.cell(new_i, new_j) == grid.ng_char() {
            continue;
        }
        dest.push(((new_i, new_j), 1));
    }
    dest
}

#[cfg(test)]
mod tests {
    mod graph {
        use super::super::Graph;

        #[test]
        fn shortest_path() {
            let edges = vec![(0, 1), (0, 2), (1, 2), (2, 3), (2, 4), (3, 4), (4, 5)];
            // 頂点6には到達しない
            let graph = Graph::new(&edges, 7);
            let res = graph.shortest_path(0);
            assert_eq!(res[0], Some(0));
            assert_eq!(res[1], Some(1));
            assert_eq!(res[2], Some(1));
            assert_eq!(res[3], Some(2));
            assert_eq!(res[4], Some(2));
            assert_eq!(res[5], Some(3));
            assert_eq!(res[6], None);
        }

        #[test]
        fn vertex_count() {
            let graph = Graph::new(&[], 5);
            assert_eq!(graph.vertex_count(), 5);
        }
    }

    mod weighted_graph {
        use super::super::WeightedGraph;

        #[test]
        fn bellman_ford() {
            let edges = vec![(0, 1, 1), (0, 2, 2), (1, 3, 3), (2, 3, 3)];
            // 頂点4には到達しない
            let graph = WeightedGraph::new(&edges, 5);
            let res = graph.bellman_ford(0);
            assert!(res.is_some());
            let res = res.unwrap();
            assert_eq!(res[0], Some(0));
            assert_eq!(res[1], Some(1));
            assert_eq!(res[2], Some(2));
            assert_eq!(res[3], Some(4));
            assert_eq!(res[4], None);
        }

        #[test]
        fn bellman_ford_has_negative_weight() {
            let edges = vec![(0, 1, 1), (1, 2, -3), (1, 3, 3), (2, 0, 2), (2, 3, 3)];
            let graph = WeightedGraph::new_directed(&edges, 4);
            let res = graph.bellman_ford(0);
            assert!(res.is_some());
            let res = res.unwrap();
            assert_eq!(res[0], Some(0));
            assert_eq!(res[1], Some(1));
            assert_eq!(res[2], Some(-2));
            assert_eq!(res[3], Some(1));
        }

        #[test]
        fn bellman_ford_has_negative_loop() {
            let edges = vec![(0, 1, 1), (1, 2, -4), (1, 3, 3), (2, 0, 2), (2, 3, 3)];
            let graph = WeightedGraph::new_directed(&edges, 4);
            let res = graph.bellman_ford(0);
            assert!(res.is_none());
        }

        #[test]
        fn prim() {
            let edges = vec![(0, 1, 1), (0, 2, 5), (0, 3, 2), (1, 3, 1), (2, 3, 3)];
            let graph = WeightedGraph::new(&edges, 4);
            assert_eq!(graph.prim(), 5);
        }

        #[test]
        fn reachable_vertexes() {
            let edges = vec![(0, 1, 1), (1, 2, 1), (1, 3, 1)];
            let graph = WeightedGraph::new_directed(&edges, 4);
            assert_eq!(
                graph.reachable_vertexes(0),
                vec![0, 1, 2, 3].into_iter().collect()
            );
            assert_eq!(
                graph.reachable_vertexes(1),
                vec![1, 2, 3].into_iter().collect()
            );
            assert_eq!(graph.reachable_vertexes(2), vec![2].into_iter().collect());
            assert_eq!(graph.reachable_vertexes(3), vec![3].into_iter().collect());
        }

        #[test]
        fn rev() {
            let edges = vec![(0, 1, 1), (0, 2, 2), (1, 2, 3)];
            let graph = WeightedGraph::new_directed(&edges, 4);
            let rev_graph = graph.rev();
            assert_eq!(rev_graph.vertex_count(), graph.vertex_count());
            assert_eq!(rev_graph.graph.len(), graph.graph.len());
            assert_eq!(rev_graph.graph[0], vec![]);
            assert_eq!(rev_graph.graph[0], vec![]);
            assert_eq!(rev_graph.graph[1], vec![(0, 1)]);
            assert_eq!(rev_graph.graph[2], vec![(0, 2), (1, 3)]);
        }

        #[test]
        fn shortest_path() {
            let edges = vec![
                (0, 1, 1),
                (0, 2, 2),
                (1, 3, 5),
                (1, 4, 1),
                (2, 3, 3),
                (3, 4, 2),
            ];
            // 頂点5には到達しない
            let graph = WeightedGraph::new(&edges, 6);
            let res = graph.shortest_path(0);
            assert_eq!(res[0], Some(0));
            assert_eq!(res[1], Some(1));
            assert_eq!(res[2], Some(2));
            assert_eq!(res[3], Some(4));
            assert_eq!(res[4], Some(2));
            assert_eq!(res[5], None);
        }

        #[test]
        fn shortest_path_01() {
            let edges = vec![
                (0, 1, 1),
                (0, 2, 0),
                (1, 3, 0),
                (1, 4, 1),
                (2, 3, 0),
                (3, 4, 1),
            ];
            // 頂点5には到達しない
            let graph = WeightedGraph::new(&edges, 6);
            let res = graph.shortest_path_01(0);
            assert_eq!(res[0], Some(0));
            assert_eq!(res[1], Some(0));
            assert_eq!(res[2], Some(0));
            assert_eq!(res[3], Some(0));
            assert_eq!(res[4], Some(1));
            assert_eq!(res[5], None);
        }

        #[test]
        fn test_traveling_salesman() {
            // ref. https://atcoder.jp/contests/abc180/tasks/abc180_e
            let n = 3;
            let vertexes: Vec<(i64, i64, i64)> = vec![(0, 0, 0), (1, 1, 1), (-1, -1, -1)];
            let mut graph = WeightedGraph::new(&[], 3);
            for u in 0..n {
                for v in 0..n {
                    if u == v {
                        continue;
                    }
                    let (ux, uy, uz) = vertexes[u];
                    let (vx, vy, vz) = vertexes[v];
                    let cost = (vx - ux).abs() + (vy - uy).abs() + 0.max(vz - uz);
                    graph.add_directed_edge((u, v, cost));
                }
            }
            let cost = graph.traveling_salesman(0);
            assert_eq!(cost, 10);
        }

        #[test]
        fn vertex_count() {
            let graph = WeightedGraph::new(&[], 5);
            assert_eq!(graph.vertex_count(), 5);
        }

        #[test]
        fn warshall_floyd() {
            let edges = vec![
                (0, 1, 1),
                (0, 2, 2),
                (1, 3, 5),
                (1, 4, 1),
                (2, 3, 3),
                (3, 4, 2),
            ];
            let graph = WeightedGraph::new(&edges, 6);
            let res = graph.warshall_floyd();
            // start: 0
            assert_eq!(res[0][0], Some(0));
            assert_eq!(res[0][1], Some(1));
            assert_eq!(res[0][2], Some(2));
            assert_eq!(res[0][3], Some(4));
            assert_eq!(res[0][4], Some(2));
            assert_eq!(res[0][5], None);
            // start: 1
            assert_eq!(res[1][0], Some(1));
            assert_eq!(res[1][1], Some(0));
            assert_eq!(res[1][2], Some(3));
            assert_eq!(res[1][3], Some(3));
            assert_eq!(res[1][4], Some(1));
            assert_eq!(res[1][5], None);
            // start: 2
            assert_eq!(res[2][0], Some(2));
            assert_eq!(res[2][1], Some(3));
            assert_eq!(res[2][2], Some(0));
            assert_eq!(res[2][3], Some(3));
            assert_eq!(res[2][4], Some(4));
            assert_eq!(res[2][5], None);
            // start: 3
            assert_eq!(res[3][0], Some(4));
            assert_eq!(res[3][1], Some(3));
            assert_eq!(res[3][2], Some(3));
            assert_eq!(res[3][3], Some(0));
            assert_eq!(res[3][4], Some(2));
            assert_eq!(res[3][5], None);
            // start: 4
            assert_eq!(res[4][0], Some(2));
            assert_eq!(res[4][1], Some(1));
            assert_eq!(res[4][2], Some(4));
            assert_eq!(res[4][3], Some(2));
            assert_eq!(res[4][4], Some(0));
            assert_eq!(res[4][5], None);
            // start: 5
            assert_eq!(res[5][0], None);
            assert_eq!(res[5][1], None);
            assert_eq!(res[5][2], None);
            assert_eq!(res[5][3], None);
            assert_eq!(res[5][4], None);
            assert_eq!(res[5][5], Some(0));
        }
    }

    mod grid {
        use super::super::{gen_grid_destinations, Grid, ALL_DIRS, UDLR_DIRS};

        #[test]
        fn to_graph_udlr_dirs() {
            let grid = vec!["...#.", ".#.#.", ".#..."]
                .into_iter()
                .map(|s| s.chars().collect::<Vec<char>>())
                .collect::<Vec<_>>();
            let grid = Grid::new(&grid, '#');
            let graph = grid.to_graph(|grid, i, j| gen_grid_destinations(grid, i, j, &UDLR_DIRS));
            let s = grid.vertex(0, 0);
            let d = graph.shortest_path(s);
            assert_eq!(d[grid.vertex(0, 0)], Some(0));
            assert_eq!(d[grid.vertex(0, 1)], Some(1));
            assert_eq!(d[grid.vertex(0, 2)], Some(2));
            assert_eq!(d[grid.vertex(0, 4)], Some(8));
            assert_eq!(d[grid.vertex(1, 0)], Some(1));
            assert_eq!(d[grid.vertex(1, 2)], Some(3));
            assert_eq!(d[grid.vertex(1, 4)], Some(7));
            assert_eq!(d[grid.vertex(2, 0)], Some(2));
            assert_eq!(d[grid.vertex(2, 2)], Some(4));
            assert_eq!(d[grid.vertex(2, 3)], Some(5));
            assert_eq!(d[grid.vertex(2, 4)], Some(6));
        }

        #[test]
        fn to_graph_all_dirs() {
            let grid = vec![".....", ".#.#.", "....."]
                .into_iter()
                .map(|s| s.chars().collect::<Vec<char>>())
                .collect::<Vec<_>>();
            let grid = Grid::new(&grid, '#');
            let graph = grid.to_graph(|grid, i, j| gen_grid_destinations(grid, i, j, &ALL_DIRS));
            let s = grid.vertex(1, 2);
            let d = graph.shortest_path(s);
            assert_eq!(d[grid.vertex(0, 0)], Some(2));
            assert_eq!(d[grid.vertex(0, 1)], Some(1));
            assert_eq!(d[grid.vertex(0, 2)], Some(1));
            assert_eq!(d[grid.vertex(0, 3)], Some(1));
            assert_eq!(d[grid.vertex(0, 4)], Some(2));
            assert_eq!(d[grid.vertex(1, 0)], Some(2));
            assert_eq!(d[grid.vertex(1, 2)], Some(0));
            assert_eq!(d[grid.vertex(1, 4)], Some(2));
            assert_eq!(d[grid.vertex(2, 0)], Some(2));
            assert_eq!(d[grid.vertex(2, 1)], Some(1));
            assert_eq!(d[grid.vertex(2, 2)], Some(1));
            assert_eq!(d[grid.vertex(2, 3)], Some(1));
            assert_eq!(d[grid.vertex(2, 4)], Some(2));
        }

        #[test]
        fn height() {
            let grid = vec![".....", ".#.#.", "....."]
                .into_iter()
                .map(|s| s.chars().collect::<Vec<char>>())
                .collect::<Vec<_>>();
            let grid = Grid::new(&grid, '#');
            assert_eq!(grid.height(), 3);
        }

        #[test]
        fn width() {
            let grid = vec![".....", ".#.#.", "....."]
                .into_iter()
                .map(|s| s.chars().collect::<Vec<char>>())
                .collect::<Vec<_>>();
            let grid = Grid::new(&grid, '#');
            assert_eq!(grid.width(), 5);
        }

        #[test]
        fn in_grid() {
            let grid = vec!["...", "..."]
                .into_iter()
                .map(|s| s.chars().collect::<Vec<char>>())
                .collect::<Vec<_>>();
            let grid = Grid::new(&grid, '#');
            assert_eq!(grid.in_grid(-1, 0), false);
            assert_eq!(grid.in_grid(0, -1), false);
            assert_eq!(grid.in_grid(0, 0), true);
            assert_eq!(grid.in_grid(0, 1), true);
            assert_eq!(grid.in_grid(0, 2), true);
            assert_eq!(grid.in_grid(0, 3), false);
            assert_eq!(grid.in_grid(1, 0), true);
            assert_eq!(grid.in_grid(1, 1), true);
            assert_eq!(grid.in_grid(1, 2), true);
            assert_eq!(grid.in_grid(1, 3), false);
            assert_eq!(grid.in_grid(2, 0), false);
        }

        #[test]
        fn ng_char() {
            let grid = vec!["...", "..."]
                .into_iter()
                .map(|s| s.chars().collect::<Vec<char>>())
                .collect::<Vec<_>>();
            let grid = Grid::new(&grid, '#');
            assert_eq!(grid.ng_char(), '#');
        }
    }
}
