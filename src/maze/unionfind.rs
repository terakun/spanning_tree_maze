
pub struct UnionFind {
    par: Vec<usize>,
}

impl UnionFind {
    pub fn new(n: usize) -> Self {
        UnionFind { par: (0..n).collect() }
    }

    pub fn root(&mut self, x: usize) -> usize {
        let parx = self.par[x];
        if parx == x {
            x
        } else {
            self.par[x] = self.root(parx);
            self.par[x]
        }
    }

    pub fn same(&mut self, x: usize, y: usize) -> bool {
        self.root(x) == self.root(y)
    }

    pub fn unite(&mut self, x: usize, y: usize) -> bool {
        let xroot = self.root(x);
        let yroot = self.root(y);
        if xroot == yroot {
            true
        } else {
            self.par[xroot] = yroot;
            false
        }
    }
}
