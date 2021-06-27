use cargo_snippet::snippet;

// graph

#[snippet("graph")]
#[derive(Clone)]
pub struct Graph<Cost>
where
    Cost: PartialOrd + Ord + Copy + num::traits::NumAssign,
{
    graph: Vec<Vec<(usize, Cost)>>,
    vc: usize,
    inf: Cost,
}

#[snippet("graph")]
#[derive(Clone)]
pub struct Edge<Cost>
where
    Cost: PartialOrd + Ord + Copy + num::traits::NumAssign,
{
    from: usize,
    to: usize,
    cost: Cost
}

#[snippet("graph")]
impl<Cost> From<(usize, usize, Cost)> for Edge<Cost>
where
    Cost: PartialOrd + Ord + Copy + num::traits::NumAssign,
 {
    fn from(e: (usize, usize, Cost)) -> Edge<Cost> {
        Edge {
            from: e.0,
            to: e.1,
            cost: e.2
        }
    }
}

#[snippet("graph")]
impl<Cost> From<(usize, usize)> for Edge<Cost>
where
    Cost: PartialOrd + Ord + Copy + num::traits::NumAssign,
{
    fn from(e: (usize, usize)) -> Edge<Cost> {
        Edge {
            from: e.0,
            to: e.1,
            cost: Cost::one()
        }
    }
}

#[snippet("graph")]
impl<Cost> Graph<Cost>
where
    Cost: PartialOrd + Ord + Copy + num::traits::NumAssign,
{
    pub fn new(edges: Vec<impl Into<Edge<Cost>>>, vc: usize, inf: Cost) -> Self {
        let mut graph = vec![Vec::new(); vc];
        for e in edges {
            let e = e.into();
            graph[e.from].push((e.to, e.cost));
            graph[e.to].push((e.from, e.cost));
        }
        Self { graph, vc, inf }
    }

    pub fn new_empty(vc: usize, inf: Cost) -> Self {
        let graph = vec![Vec::new(); vc];
        Self { graph, vc, inf }
    }

    pub fn new_directed(edges: Vec<impl Into<Edge<Cost>>>, vc: usize, inf: Cost) -> Self {
        let mut graph = vec![Vec::new(); vc];
        for e in edges {
            let e = e.into();
            graph[e.from].push((e.to, e.cost));
        }
        Self { graph, vc, inf }
    }

    pub fn add_directed_edge(&mut self, e: impl Into<Edge<Cost>>) {
        let e = e.into();
        self.graph[e.from].push((e.to, e.cost));
    }

    pub fn add_edge(&mut self, e: impl Into<Edge<Cost>>) {
        let e = e.into();
        self.graph[e.from].push((e.to, e.cost));
        self.graph[e.to].push((e.from, e.cost));
    }

    pub fn rev(&self) -> Graph<Cost> {
        let mut edges: Vec<Edge<Cost>> = Vec::new();
        for u in 0..self.vc {
            for &(v, w) in &self.graph[u] {
                edges.push((v, u, w).into());
            }
        }
        Self::new_directed(edges, self.vc, self.inf)
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

    pub fn vertex_count(&self) -> usize {
        self.vc
    }

    fn optionalize(&self, v: Vec<Cost>) -> Vec<Option<Cost>> {
        v.into_iter()
            .map(|x| if x == self.inf { None } else { Some(x) })
            .collect::<Vec<_>>()
    }
}

#[snippet("bellman_ford")]
impl<Cost> Graph<Cost>
where
    Cost: PartialOrd + Ord + Copy + num::traits::NumAssign,
{
    pub fn bellman_ford(&self, s: usize) -> Option<Vec<Option<Cost>>> {
        let vc = self.vc;
        let mut cost_list = vec![self.inf; vc];
        cost_list[s] = Cost::zero();
        for c in 0..vc {
            for u in 0..vc {
                for &(v, w) in &self.graph[u] {
                    if cost_list[u] == self.inf {
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
}

#[snippet("prim")]
impl<Cost> Graph<Cost>
where
    Cost: PartialOrd + Ord + Copy + num::traits::NumAssign,
{
    pub fn prim(&self) -> Cost {
        let mut used = std::collections::HashSet::new();
        let mut heap = std::collections::BinaryHeap::new();

        let mut res = Cost::zero();
        heap.push(std::cmp::Reverse((Cost::zero(), 0)));
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
}

#[snippet("shortest_path")]
impl<Cost> Graph<Cost>
where
    Cost: PartialOrd + Ord + Copy + num::traits::NumAssign,
{
    pub fn shortest_path(&self, start: usize) -> Vec<Option<Cost>> {
        let mut cost_list = vec![self.inf; self.vc];
        let mut heap = std::collections::BinaryHeap::new();

        cost_list[start] = Cost::zero();
        heap.push(std::cmp::Reverse((Cost::zero(), start)));

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
}

#[snippet("shortest_path1")]
impl<Cost> Graph<Cost>
where
    Cost: PartialOrd + Ord + Copy + num::traits::NumAssign,
{
    pub fn shortest_path_1(&self, start: usize) -> Vec<Option<Cost>> {
        let mut cost_list = vec![None; self.vc];
        let mut que = std::collections::VecDeque::new();
        cost_list[start] = Some(Cost::zero());
        que.push_back(start);
        while let Some(u) = que.pop_front() {
            for &(v, w) in &self.graph[u] {
                if !w.is_one() {
                    panic!("weight is not 1");
                }
                if cost_list[v].is_some() {
                    continue;
                }
                let new_cost = cost_list[u].unwrap() + Cost::one();
                cost_list[v] = Some(new_cost);
                que.push_back(v);
            }
        }
        cost_list
    }
}

#[snippet("shortest_path01")]
impl<Cost> Graph<Cost>
where
    Cost: PartialOrd + Ord + Copy + num::traits::NumAssign,
{
    pub fn shortest_path_01(&self, start: usize) -> Vec<Option<Cost>> {
        let mut cost_list = vec![self.inf; self.vc];
        let mut que = std::collections::VecDeque::new();

        cost_list[start] = Cost::zero();
        que.push_front((start, Cost::zero()));

        while let Some((u, cost)) = que.pop_front() {
            if cost > cost_list[u] {
                continue;
            }
            for &(v, w) in &self.graph[u] {
                if !w.is_zero() && !w.is_one() {
                    panic!("weight is not 01");
                }
                let new_cost = cost + w;
                if new_cost < cost_list[v] {
                    cost_list[v] = new_cost;
                    if w.is_zero() {
                        que.push_front((v, new_cost));
                    } else {
                        que.push_back((v, new_cost));
                    }
                }
            }
        }
        self.optionalize(cost_list)
    }
}

#[snippet("traveling_salesman")]
impl<Cost> Graph<Cost>
where
    Cost: PartialOrd + Ord + Copy + num::traits::NumAssign,
{
    pub fn traveling_salesman(&self, start: usize) -> Cost {
        let mut dp = vec![vec![None; self.vc]; 1 << self.vc];
        let fin = (1 << self.vc) - 1;
        self.traveling_salesman_impl(0, start, &mut dp, start, fin)
    }

    fn traveling_salesman_impl(
        &self,
        state: usize,
        u: usize,
        dp: &mut [Vec<Option<Cost>>],
        start: usize,
        fin: usize,
    ) -> Cost {
        if let Some(res) = dp[state][u] {
            return res;
        }
        if state == fin && u == start {
            let res = Cost::zero();
            dp[state][u] = Some(res);
            return res;
        }
        let mut res = self.inf;
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
}

#[snippet("warshall_floyd")]
impl<Cost> Graph<Cost>
where
    Cost: PartialOrd + Ord + Copy + num::traits::NumAssign,
{
    pub fn warshall_floyd(&self) -> Vec<Vec<Option<Cost>>> {
        let vc = self.vc;
        let mut cost_list = vec![vec![self.inf; vc]; vc];
        for u in 0..vc {
            for &(v, w) in &self.graph[u] {
                cost_list[u][v] = w;
            }
        }
        for i in 0..vc {
            cost_list[i][i] = Cost::zero();
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
}

#[snippet("scc")]
impl<Cost> Graph<Cost>
where
    Cost: PartialOrd + Ord + Copy + num::traits::NumAssign,
{
    pub fn scc(&self) -> Vec<Vec<usize>> {
        let vc = self.vc;
        // 番号を記録 1..=N
        let mut id = 0;
        let mut ids = vec![0; vc];
        let mut used = vec![false; vc];
        for u in 0..vc {
            if used[u] {
                continue;
            }
            self.scc_dfs1(u, &mut id, &mut ids, &mut used);
        }
        let mut u_with_id = ids.into_iter().enumerate().collect::<Vec<_>>();
        u_with_id.sort_by_key(|(_, id)| -(*id as isize));
        // グループ分け
        let rev_graph = self.rev();
        let mut groups = Vec::new();
        let mut used = vec![false; vc];
        for (u, _) in u_with_id {
            if used[u] {
                continue;
            }
            let mut group = Vec::new();
            rev_graph.scc_dfs2(u, &mut group, &mut used);
            groups.push(group);
        }
        groups
    }

    fn scc_dfs1(&self, u: usize, id: &mut usize, ids: &mut Vec<usize>, used: &mut Vec<bool>) {
        used[u] = true;
        for &(v, _) in &self.graph[u] {
            if used[v] {
                continue;
            }
            self.scc_dfs1(v, id, ids, used);
        }
        *id += 1;
        ids[u] = *id;
    }

    fn scc_dfs2(&self, u: usize, group: &mut Vec<usize>, used: &mut Vec<bool>) {
        group.push(u);
        used[u] = true;
        for &(v, _) in &self.graph[u] {
            if used[v] {
                continue;
            }
            self.scc_dfs2(v, group, used);
        }
    }
}

// grid

#[snippet("grid")]
#[snippet(include = "graph")]
pub struct Grid<T>
where
    T: PartialEq + Eq + Copy,
{
    grid: Vec<Vec<T>>,
    h: usize,
    w: usize,
    ng: Option<T>,
}

#[snippet("grid")]
impl<T> Grid<T>
where
    T: PartialEq + Eq + Copy,
{
    pub fn new(grid: Vec<Vec<T>>, ng: impl Into<Option<T>>) -> Self {
        assert!(grid.len() > 0);
        let h = grid.len();
        let w = grid[0].len();
        let ng: Option<T> = ng.into();
        Self { grid, h, w, ng }
    }

    pub fn to_graph<Cost, F>(&self, inf: Cost, generator: F) -> Graph<Cost>
    where
        Cost: PartialOrd + Ord + Copy + num::traits::NumAssign,
        F: Fn(&Grid<T>, usize, usize) -> Vec<GridDestination<Cost>>,
    {
        let mut edges: Vec<Edge<Cost>> = Vec::new();
        for i in 0..self.h {
            for j in 0..self.w {
                let from = self.vertex(i, j);
                for (pos, w) in generator(&self, i, j) {
                    let to = self.vertex(pos.0, pos.1);
                    edges.push((from, to, w).into());
                }
            }
        }
        Graph::new_directed(edges, self.h * self.w, inf)
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

    pub fn cell(&self, i: usize, j: usize) -> T {
        self.grid[i][j]
    }

    pub fn ng(&self) -> Option<T> {
        self.ng
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
pub type GridDestination<Cost> = (GridPos, Cost);

#[snippet("grid")]
pub fn gen_grid_destinations<T, Cost>(
    grid: &Grid<T>,
    i: usize,
    j: usize,
    directions: &[(isize, isize)],
) -> Vec<GridDestination<Cost>>
where
    T: PartialEq + Eq + Copy,
    Cost: PartialOrd + Ord + Copy + num::traits::NumAssign,
{
    let mut dest = Vec::new();
    if grid.ng().is_some() && grid.cell(i, j) == grid.ng().unwrap() {
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
        if grid.ng().is_some() && grid.cell(new_i, new_j) == grid.ng().unwrap() {
            continue;
        }
        dest.push(((new_i, new_j), Cost::one()));
    }
    dest
}

#[cfg(test)]
mod tests {
    mod graph {
        use super::super::Graph;
        use std::collections::*;

        #[test]
        fn test_new_noweight() {
            let edges = vec![
                (0, 1),
                (1, 2)
            ];
            let graph = Graph::new(edges, 3, 1_i64 << 60);
            // コスト0で生成されていることを確認する
            let d = graph.shortest_path(0);
            assert_eq!(d[0], Some(0));
            assert_eq!(d[1], Some(1));
            assert_eq!(d[2], Some(2));
        }

        #[test]
        fn test_bellman_ford() {
            let edges = vec![(0, 1, 1), (0, 2, 2), (1, 3, 3), (2, 3, 3)];
            // 頂点4には到達しない
            let graph = Graph::new(edges, 5, 1_i64 << 60);
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
        fn test_bellman_ford_has_negative_weight() {
            let edges = vec![(0, 1, 1), (1, 2, -3), (1, 3, 3), (2, 0, 2), (2, 3, 3)];
            let graph = Graph::new_directed(edges, 4, 1_i64 << 60);
            let res = graph.bellman_ford(0);
            assert!(res.is_some());
            let res = res.unwrap();
            assert_eq!(res[0], Some(0));
            assert_eq!(res[1], Some(1));
            assert_eq!(res[2], Some(-2));
            assert_eq!(res[3], Some(1));
        }

        #[test]
        fn test_bellman_ford_has_negative_loop() {
            let edges = vec![(0, 1, 1), (1, 2, -4), (1, 3, 3), (2, 0, 2), (2, 3, 3)];
            let graph = Graph::new_directed(edges, 4, 1_i64 << 60);
            let res = graph.bellman_ford(0);
            assert!(res.is_none());
        }

        #[test]
        fn test_prim() {
            let edges = vec![(0, 1, 1), (0, 2, 5), (0, 3, 2), (1, 3, 1), (2, 3, 3)];
            let graph = Graph::new(edges, 4, 1_i64 << 60);
            assert_eq!(graph.prim(), 5);
        }

        #[test]
        fn test_reachable_vertexes() {
            let edges = vec![(0, 1, 1), (1, 2, 1), (1, 3, 1)];
            let graph = Graph::new_directed(edges, 4, 1_i64 << 60);
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
        fn test_rev() {
            let edges = vec![(0, 1, 1), (0, 2, 2), (1, 2, 3)];
            let graph = Graph::new_directed(edges, 4, 1_i64 << 60);
            let rev_graph = graph.rev();
            assert_eq!(rev_graph.vertex_count(), graph.vertex_count());
            assert_eq!(rev_graph.graph.len(), graph.graph.len());
            assert_eq!(rev_graph.graph[0], vec![]);
            assert_eq!(rev_graph.graph[0], vec![]);
            assert_eq!(rev_graph.graph[1], vec![(0, 1)]);
            assert_eq!(rev_graph.graph[2], vec![(0, 2), (1, 3)]);
        }

        #[test]
        fn test_shortest_path() {
            let edges = vec![
                (0, 1, 1),
                (0, 2, 2),
                (1, 3, 5),
                (1, 4, 1),
                (2, 3, 3),
                (3, 4, 2),
            ];
            // 頂点5には到達しない
            let graph = Graph::new(edges, 6, 1_i64 << 60);
            let res = graph.shortest_path(0);
            assert_eq!(res[0], Some(0));
            assert_eq!(res[1], Some(1));
            assert_eq!(res[2], Some(2));
            assert_eq!(res[3], Some(4));
            assert_eq!(res[4], Some(2));
            assert_eq!(res[5], None);
        }

        #[test]
        fn test_shortest_path_01() {
            let edges = vec![
                (0, 1, 1),
                (0, 2, 0),
                (1, 3, 0),
                (1, 4, 1),
                (2, 3, 0),
                (3, 4, 1),
            ];
            // 頂点5には到達しない
            let graph = Graph::new(edges, 6, 1_i64 << 60);
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
            let mut graph = Graph::new_empty( 3, 1_i64 << 60);
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
        fn test_vertex_count() {
            let graph = Graph::new_empty( 5, 1_i64 << 60);
            assert_eq!(graph.vertex_count(), 5);
        }

        #[test]
        fn test_warshall_floyd() {
            let edges = vec![
                (0, 1, 1),
                (0, 2, 2),
                (1, 3, 5),
                (1, 4, 1),
                (2, 3, 3),
                (3, 4, 2),
            ];
            let graph = Graph::new(edges, 6, 1_i64 << 60);
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

        #[test]
        fn test_scc() {
            let edges = vec![
                (0, 1, 1),
                (1, 0, 1),
                (1, 2, 1),
                (3, 2, 1),
                (3, 0, 1),
                (0, 3, 1),
            ];
            let graph = Graph::new_directed(edges, 4, 1_i64 << 60);
            let mut res = graph.scc();
            res.sort();
            assert_eq!(res.len(), 2);
            let g1 = res[0].iter().copied().collect::<HashSet<usize>>();
            assert_eq!(g1, vec![0, 1, 3].into_iter().collect::<HashSet<_>>());
            let g2 = res[1].iter().copied().collect::<HashSet<usize>>();
            assert_eq!(g2, vec![2].into_iter().collect::<HashSet<_>>());
        }
    }

    mod grid {
        use super::super::{gen_grid_destinations, Grid, ALL_DIRS, UDLR_DIRS};

        #[test]
        fn test_to_graph_udlr_dirs() {
            let grid = vec!["...#.", ".#.#.", ".#..."]
                .into_iter()
                .map(|s| s.chars().collect::<Vec<char>>())
                .collect::<Vec<_>>();
            let grid = Grid::new(grid, '#');
            let graph = grid.to_graph(1_i64 << 60, |grid, i, j| {
                gen_grid_destinations(grid, i, j, &UDLR_DIRS)
            });
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
        fn test_to_graph_all_dirs() {
            let grid = vec![".....", ".#.#.", "....."]
                .into_iter()
                .map(|s| s.chars().collect::<Vec<char>>())
                .collect::<Vec<_>>();
            let grid = Grid::new(grid, '#');
            let graph = grid.to_graph(1_i64 << 60, |grid, i, j| {
                gen_grid_destinations(grid, i, j, &ALL_DIRS)
            });
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
        fn test_height() {
            let grid = vec![".....", ".#.#.", "....."]
                .into_iter()
                .map(|s| s.chars().collect::<Vec<char>>())
                .collect::<Vec<_>>();
            let grid = Grid::new(grid, '#');
            assert_eq!(grid.height(), 3);
        }

        #[test]
        fn test_width() {
            let grid = vec![".....", ".#.#.", "....."]
                .into_iter()
                .map(|s| s.chars().collect::<Vec<char>>())
                .collect::<Vec<_>>();
            let grid = Grid::new(grid, '#');
            assert_eq!(grid.width(), 5);
        }

        #[test]
        fn test_in_grid() {
            let grid = vec!["...", "..."]
                .into_iter()
                .map(|s| s.chars().collect::<Vec<char>>())
                .collect::<Vec<_>>();
            let grid = Grid::new(grid, '#');
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
        fn test_ng_some() {
            let grid = vec!["...", "..."]
                .into_iter()
                .map(|s| s.chars().collect::<Vec<char>>())
                .collect::<Vec<_>>();
            let grid = Grid::new(grid, '#');
            assert_eq!(grid.ng(), Some('#'));
        }

        #[test]
        fn test_ng_none() {
            let grid = vec!["...", "..."]
                .into_iter()
                .map(|s| s.chars().collect::<Vec<char>>())
                .collect::<Vec<_>>();
            let grid = Grid::new(grid, None);
            assert_eq!(grid.ng(), None);
        }
    }
}
