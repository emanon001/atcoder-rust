#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

fn solve() {
    input! {
        n: usize,
        cols: [[usize]; n],
        m: usize,
        av: [usize; m]
    };

    let mut pset1 = BTreeSet::new();
    for i in 0..n {
        pset1.insert((cols[i][0], (i, 0)));
    }
    let mut pset2 = pset1.clone();
    let mut seconds = vec![None; n];
    for i in 0..n {
        if cols[i].len() >= 2 {
            pset2.insert((cols[i][1], (i, 1)));
            seconds[i] = Some(1);
        }
    }
    for a in av {
        let res = if a == 1 {
            let p = *pset1.iter().next_back().unwrap();
            pset1.remove(&p);
            pset2.remove(&p);
            let (t, (i, _)) = p;
            // 1 <- 2
            if seconds[i].is_some() {
                let sj = seconds[i].unwrap();
                let second_p = (cols[i][sj], (i, sj));
                pset1.insert(second_p);
            }
            // 2 <- 3
            if seconds[i].is_some() && seconds[i].unwrap() + 1 < cols[i].len() {
                let nj = seconds[i].unwrap() + 1;
                let next_p = (cols[i][nj], (i, nj));
                pset2.insert(next_p);
                seconds[i] = Some(nj);
            } else {
                seconds[i] = None;
            }
            t
        } else {
            // 2
            let p = *pset2.iter().next_back().unwrap();
            pset1.remove(&p);
            pset2.remove(&p);
            let (t, (i, j)) = p;
            // 1 <- 2
            if seconds[i].is_some() && seconds[i].unwrap() != j {
                let sj = seconds[i].unwrap();
                let second_p = (cols[i][sj], (i, sj));
                pset1.insert(second_p);
            }
            // 2 <- 3
            if seconds[i].is_some() && seconds[i].unwrap() + 1 < cols[i].len() {
                let nj = seconds[i].unwrap() + 1;
                let next_p = (cols[i][nj], (i, nj));
                pset2.insert(next_p);
                seconds[i] = Some(nj);
            } else {
                seconds[i] = None;
            }
            t
        };
        println!("{}", res);
    }
}

fn main() {
    std::thread::Builder::new()
        .name("big stack size".into())
        .stack_size(256 * 1024 * 1024)
        .spawn(|| {
            solve();
        })
        .unwrap()
        .join()
        .unwrap();
}
