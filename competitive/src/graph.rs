pub struct Graph {
    graph: Vec<Vec<usize>>,
    vn: usize,
}

type Edge = (usize, usize);
impl Graph {
    pub fn new(edges: &[Edge], vn: usize) -> Self {
        let mut graph = vec![Vec::new(); vn];
        for &(u, v) in edges {
            graph[u].push(v);
            graph[v].push(u);
        }
        Self { graph, vn }
    }

    pub fn new_directed(edges: &[Edge], vn: usize) -> Self {
        let mut graph = vec![Vec::new(); vn];
        for &(u, v) in edges {
            graph[u].push(v);
        }
        Self { graph, vn }
    }

    pub fn add_directed_edge(&mut self, e: Edge) {
        self.graph[e.0].push(e.1);
    }

    pub fn add_edge(&mut self, e: Edge) {
        self.graph[e.0].push(e.1);
        self.graph[e.1].push(e.0);
    }

    pub fn shortest_path(&self, start: usize) -> Vec<Option<usize>> {
        let mut cost_list = vec![None; self.vn];
        let mut que = std::collections::VecDeque::new();
        cost_list[start] = Some(0);
        que.push_back(start);
        while let Some(u) = que.pop_front() {
            for &v in &self.graph[u] {
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
}

pub struct WeightedGraph {
    graph: Vec<Vec<(usize, i64)>>,
    vn: usize,
}

type WeightedEdge = (usize, usize, i64);
impl WeightedGraph {
    const INF: i64 = 1 << 60;

    pub fn new(edges: &[WeightedEdge], vn: usize) -> Self {
        let mut graph = vec![Vec::new(); vn];
        for &(u, v, w) in edges {
            graph[u].push((v, w));
            graph[v].push((u, w));
        }
        Self { graph, vn }
    }

    pub fn new_directed(edges: &[WeightedEdge], vn: usize) -> Self {
        let mut graph = vec![Vec::new(); vn];
        for &(u, v, w) in edges {
            graph[u].push((v, w));
        }
        Self { graph, vn }
    }

    pub fn add_directed_edge(&mut self, e: WeightedEdge) {
        self.graph[e.0].push((e.1, e.2));
    }

    pub fn add_edge(&mut self, e: WeightedEdge) {
        self.graph[e.0].push((e.1, e.2));
        self.graph[e.1].push((e.0, e.2));
    }

    pub fn bellman_ford(&self, s: usize) -> Option<Vec<Option<i64>>> {
        let vn = self.vn;
        let inf = Self::INF;
        let mut cost_list = vec![inf; vn];
        cost_list[s] = 0;
        for c in 0..vn {
            for u in 0..vn {
                for &(v, w) in &self.graph[u] {
                    if cost_list[u] == inf {
                        continue;
                    }
                    let new_cost = cost_list[u] + w;
                    if new_cost < cost_list[v] {
                        cost_list[v] = new_cost;
                        if c == vn - 1 {
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
        for u in 0..self.vn {
            for &(v, w) in &self.graph[u] {
                edges.push((v, u, w));
            }
        }
        Self::new_directed(&edges, self.vn)
    }

    pub fn shortest_path(&self, start: usize) -> Vec<Option<i64>> {
        let mut cost_list = vec![Self::INF; self.vn];
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

    pub fn warshall_floyd(&self) -> Vec<Vec<Option<i64>>> {
        let inf = Self::INF;
        let vn = self.vn;
        let mut cost_list = vec![vec![inf; vn]; vn];
        for u in 0..vn {
            for &(v, w) in &self.graph[u] {
                cost_list[u][v] = w;
            }
        }
        for i in 0..vn {
            cost_list[i][i] = 0;
        }
        for k in 0..vn {
            for i in 0..vn {
                for j in 0..vn {
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
            assert_eq!(rev_graph.vn, graph.vn);
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
}
