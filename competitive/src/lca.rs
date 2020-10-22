use crate::monoid::Monoid;
use crate::segment_tree::SegmentTree;
use cargo_snippet::snippet;

#[snippet("lca")]
#[snippet(include = "monoid")]
#[snippet(include = "segment_tree")]
#[derive(Debug, Copy, Clone)]
pub struct LcaDepth {
    depth: usize,
    idx: usize,
}

#[snippet("lca")]
impl Monoid for LcaDepth {
    fn mempty() -> Self {
        Self {
            depth: std::usize::MAX,
            idx: 0,
        }
    }

    fn mappend(&self, other: &Self) -> Self {
        if self.depth <= other.depth {
            *self
        } else {
            *other
        }
    }
}

#[snippet("lca")]
pub struct Lca {
    vs: Vec<usize>,
    vdepth: Vec<usize>,
    vidx: Vec<usize>,
    vn: usize,
    st: SegmentTree<LcaDepth>,
}

#[snippet("lca")]
impl Lca {
    pub fn new(edges: Vec<(usize, usize)>, vn: usize, root: usize) -> Self {
        let mut graph = vec![Vec::new(); vn];
        for (u, v) in edges {
            graph[u].push(v);
            graph[v].push(u);
        }
        let mut vidx = vec![0; vn];
        let mut vs = vec![0; vn * 2 - 1];
        let mut vdepth = vec![0; vn * 2 - 1];
        Self::traverse(
            root,
            root,
            0,
            &mut 0,
            &graph,
            &mut vidx,
            &mut vs,
            &mut vdepth,
        );
        let lca_depth = vdepth
            .iter()
            .copied()
            .enumerate()
            .map(|(i, d)| LcaDepth { depth: d, idx: i })
            .collect::<Vec<_>>();
        let st = SegmentTree::from(lca_depth);
        Self {
            vs,
            vdepth,
            vidx,
            vn,
            st,
        }
    }

    /// 0-origin
    pub fn depth(&self, u: usize) -> usize {
        if u >= self.vn {
            panic!("u >= self.vn");
        }
        let i = self.vidx[u];
        self.vdepth[i]
    }

    pub fn query(&self, u: usize, v: usize) -> usize {
        let ui = self.vidx[u];
        let vi = self.vidx[v];
        let LcaDepth { idx, .. } = self
            .st
            .query(std::cmp::min(ui, vi), std::cmp::max(ui, vi) + 1);
        self.vs[idx]
    }

    fn traverse(
        u: usize,
        parent: usize,
        depth: usize,
        vid: &mut usize,
        graph: &[Vec<usize>],
        vids: &mut [usize],
        vs: &mut [usize],
        vdepth: &mut [usize],
    ) {
        vids[u] = *vid;
        vs[*vid] = u;
        vdepth[*vid] = depth;
        *vid += 1;
        for &v in &graph[u] {
            if v != parent {
                Self::traverse(v, u, depth + 1, vid, graph, vids, vs, vdepth);
                vs[*vid] = u;
                vdepth[*vid] = depth;
                *vid += 1;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    mod lca {
        use super::super::Lca;

        #[test]
        fn test_depth() {
            let edges = vec![(0, 1), (0, 2), (1, 3), (3, 4), (1, 5), (2, 6)];
            let lca = Lca::new(edges, 7, 0);
            assert_eq!(lca.depth(0), 0);
            assert_eq!(lca.depth(1), 1);
            assert_eq!(lca.depth(2), 1);
            assert_eq!(lca.depth(3), 2);
            assert_eq!(lca.depth(4), 3);
            assert_eq!(lca.depth(5), 2);
            assert_eq!(lca.depth(6), 2);
        }

        #[test]
        fn test_query() {
            let edges = vec![(0, 1), (0, 2), (1, 3), (3, 4), (1, 5), (2, 6)];
            let lca = Lca::new(edges, 7, 0);
            assert_eq!(lca.query(0, 0), 0);
            assert_eq!(lca.query(0, 1), 0);
            assert_eq!(lca.query(0, 2), 0);
            assert_eq!(lca.query(1, 2), 0);
            assert_eq!(lca.query(3, 5), 1);
            assert_eq!(lca.query(4, 5), 1);
            assert_eq!(lca.query(4, 2), 0);
            assert_eq!(lca.query(4, 6), 0);
        }
    }
}
