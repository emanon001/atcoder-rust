#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;
use std::time::{Instant, Duration};
use whiteread::{parse_line};

struct Solver {
    now: Instant,
}

impl Solver {
    fn new() -> Self {
        let now = Instant::now();
        Self {
            now
        }
    }

    fn solve(&mut self) {
        for _ in 0..1000 {
            let duration = Instant::now() - self.now;
            let stop = duration >= Duration::from_millis(1990);
            if stop { return };
            let mut path = Vec::new();
            let (si, sj, ti, tj): (usize, usize, usize, usize) = parse_line().unwrap();
            // 縦方向
            if si <= ti {
                // ↓ 
                let mut i = si;
                while i < ti {
                    path.push('D');
                    i += 1;
                }
            } else {
                // ↑ 
                let mut i = si;
                while i > ti {
                    path.push('U');
                    i -= 1;
                }
            }
            // 横方向
            if sj <= tj {
                // →
                let mut j = sj;
                while j < tj {
                    path.push('R');
                    j += 1;
                }
            } else {
                // ←
                let mut j = sj;
                while j > tj {
                    path.push('L');
                    j -= 1;
                }
            }
            println!("{}", path.into_iter().join(""));
            let _: i64 = parse_line().unwrap();
        }
    }
}

fn main() {
    std::thread::Builder::new()
        .name("big stack size".into())
        .stack_size(256 * 1024 * 1024)
        .spawn(|| {
            let mut solver = Solver::new();
            solver.solve();
        })
        .unwrap()
        .join()
        .unwrap();
}
