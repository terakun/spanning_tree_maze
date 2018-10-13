extern crate rand;

pub mod unionfind;
pub use self::unionfind::UnionFind;

use self::rand::{Rng, thread_rng};

enum Dir {
    Right = 1,
    Down = 2,
    Left = 4,
    Up = 8,
}

fn set(pass: &mut usize, d: Dir) {
    *pass = *pass | d as usize;
}

fn check(pass: usize, d: Dir) -> bool {
    pass & d as usize != 0
}

pub struct Maze {
    rows: usize,
    cols: usize,
    map: Vec<Vec<bool>>,
}

impl Maze {
    pub fn new(rows: usize, cols: usize, edges: &Vec<Edge>) -> Self {
        let mut dirs: Vec<usize> = Vec::new();
        dirs.resize(rows * cols, 0);
        for edge in edges {
            let i = edge.0;
            let j = edge.1;
            if i + 1 == j {
                set(&mut dirs[i], Dir::Right);
                set(&mut dirs[j], Dir::Left);
            }
            if i + cols == j {
                set(&mut dirs[i], Dir::Down);
                set(&mut dirs[j], Dir::Up);
            }
        }

        let mut map: Vec<Vec<bool>> = Vec::new();
        map.resize(rows * 2 + 1, Vec::new());
        for i in 0..rows * 2 + 1 {
            map[i].resize(cols * 2 + 1, false);
        }

        for r in 0..rows {
            for c in 0..cols {
                map[r * 2 + 1][c * 2 + 1] = true;
                let pass = dirs[r * cols + c];
                if check(pass, Dir::Right) {
                    map[r * 2 + 1][c * 2 + 1 + 1] = true;
                }
                if check(pass, Dir::Down) {
                    map[r * 2 + 1 + 1][c * 2 + 1] = true;
                }
                if check(pass, Dir::Left) {
                    map[r * 2 + 1][c * 2 + 1 - 1] = true;
                }
                if check(pass, Dir::Up) {
                    map[r * 2 + 1 - 1][c * 2 + 1] = true;
                }
            }
        }

        map[1][0] = true;
        map[rows * 2 + 1 - 2][cols * 2 + 1 - 1] = true;

        Maze {
            rows: rows,
            cols: cols,
            map: map,
        }
    }

    pub fn print(&self) {
        for r in 0..self.rows * 2 + 1 {
            for c in 0..self.cols * 2 + 1 {
                if self.map[r][c] {
                    print!(" ");
                } else {
                    print!("#");
                }
            }
            println!("");
        }
    }
}

type Edge = (usize, usize);

pub fn generate_random(r: usize, c: usize) -> Maze {
    let mut edges: Vec<Edge> = Vec::new();
    for y in 0..r {
        for x in 0..c {
            if x > 0 {
                edges.push((y * c + x - 1, y * c + x));
            }
            if x < c - 1 {
                edges.push((y * c + x, y * c + x + 1));
            }
            if y > 0 {
                edges.push(((y - 1) * c + x, y * c + x));
            }
            if y < r - 1 {
                edges.push((y * c + x, (y + 1) * c + x));
            }
        }
    }

    thread_rng().shuffle(&mut edges);
    let mut uf = UnionFind::new(r * c);

    let mut maze_edges: Vec<Edge> = Vec::new();
    for e in edges {
        if !uf.same(e.0, e.1) {
            uf.unite(e.0, e.1);
            maze_edges.push(e);
        }
    }

    Maze::new(r, c, &maze_edges)
}
