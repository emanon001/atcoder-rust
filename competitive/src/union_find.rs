pub struct UnionFind {
    n: usize,
    root: Vec<usize>,
    rank: Vec<usize>,
    size: Vec<usize>,
}

impl UnionFind {
    pub fn new(n: usize) -> Self {
        let root = (0..n).collect();
        let rank = vec![0; n];
        let size = vec![1; n];
        Self {
            n,
            root,
            rank,
            size,
        }
    }

    pub fn find(&mut self, x: usize) -> usize {
        if x >= self.n {
            panic!();
        }
        if self.root[x] == x {
            x
        } else {
            let root = self.find(self.root[x]);
            self.root[x] = root;
            root
        }
    }

    pub fn unite(&mut self, x: usize, y: usize) {
        if x >= self.n || y >= self.n {
            panic!();
        }
        let x_root = self.find(x);
        let y_root = self.find(y);
        if x_root == y_root {
            return;
        }
        if self.rank[x_root] < self.rank[y_root] {
            self.root[x_root] = y_root;
            self.size[y_root] += self.size[x_root];
        } else {
            self.root[y_root] = x_root;
            self.size[x_root] += self.size[y_root];
            if self.rank[x_root] == self.rank[y_root] {
                self.rank[x_root] += 1;
            }
        }
    }

    pub fn size(&mut self, x: usize) -> usize {
        if x >= self.n {
            panic!();
        }
        let x_root = self.find(x);
        self.size[x_root]
    }

    pub fn is_same(&mut self, x: usize, y: usize) -> bool {
        if x >= self.n || y >= self.n {
            panic!();
        }
        self.find(x) == self.find(y)
    }
}

#[cfg(test)]
mod tests {
    use super::UnionFind;

    #[test]
    fn new() {
        let mut uf = UnionFind::new(2);
        assert!(uf.find(0) != uf.find(1));
        assert_eq!(uf.size(0), 1);
        assert_eq!(uf.size(1), 1);
        assert!(!uf.is_same(0, 1));
    }

    #[test]
    fn find() {
        let mut uf = UnionFind::new(3);
        assert!(uf.find(0) != uf.find(1));
        assert!(uf.find(1) != uf.find(2));
        uf.unite(0, 1);
        assert!(uf.find(0) == uf.find(1));
        assert!(uf.find(1) != uf.find(2));
    }

    #[test]
    fn unite() {
        let mut uf = UnionFind::new(3);
        assert!(uf.find(0) != uf.find(1));
        assert!(uf.find(1) != uf.find(2));
        uf.unite(0, 0);
        assert!(uf.find(0) != uf.find(1));
        assert!(uf.find(1) != uf.find(2));
        uf.unite(0, 1);
        assert!(uf.find(0) == uf.find(1));
        assert!(uf.find(1) != uf.find(2));
        uf.unite(1, 2);
        assert!(uf.find(0) == uf.find(1));
        assert!(uf.find(1) == uf.find(2));
    }

    #[test]
    fn size() {
        let mut uf = UnionFind::new(3);
        assert_eq!(uf.size(0), 1);
        assert_eq!(uf.size(1), 1);
        assert_eq!(uf.size(2), 1);
        uf.unite(0, 1);
        assert_eq!(uf.size(0), 2);
        assert_eq!(uf.size(1), 2);
        assert_eq!(uf.size(2), 1);
        uf.unite(1, 2);
        assert_eq!(uf.size(0), 3);
        assert_eq!(uf.size(1), 3);
        assert_eq!(uf.size(2), 3);
    }

    #[test]
    fn is_same() {
        let mut uf = UnionFind::new(3);
        assert!(!uf.is_same(0, 1));
        assert!(!uf.is_same(1, 2));
        uf.unite(0, 1);
        assert!(uf.is_same(0, 1));
        assert!(!uf.is_same(1, 2));
        uf.unite(1, 2);
        assert!(uf.is_same(0, 1));
        assert!(uf.is_same(1, 2));
    }
}
