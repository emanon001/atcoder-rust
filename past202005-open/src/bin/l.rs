#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

fn insert_second(
    i: usize,
    j: usize,
    tv: &[Vec<usize>],
    seconds: &mut [Option<usize>],
    set: &mut BTreeSet<(usize, usize)>,
) {
    if j < tv[i].len() {
        let t = tv[i][j];
        set.insert((t, i));
        seconds[i] = Some(j);
    } else {
        seconds[i] = None;
    }
}

fn solve() {
    input! {
        n: usize,
        tv: [[usize]; n],
        m: usize,
        av: [usize; m]
    };

    let mut seconds = vec![None; n];
    let mut set1 = BTreeSet::new();
    for i in 0..n {
        let t = tv[i][0];
        set1.insert((t, i));
    }
    let mut set2 = BTreeSet::new();
    for i in 0..n {
        if tv[i].len() > 1 {
            let t = tv[i][1];
            set2.insert((t, i));
            seconds[i] = Some(1);
        }
    }
    for a in av {
        if a == 1 {
            let &(t, i) = set1.iter().last().unwrap();
            set1.remove(&(t, i));
            println!("{}", t);
            if let Some(j) = seconds[i] {
                // set2 -> set1
                let t = tv[i][j];
                set2.remove(&(t, i));
                set1.insert((t, i));
                // tv -> set2
                insert_second(i, j + 1, &tv, &mut seconds, &mut set2);
            }
        } else {
            let &(t1, i1) = set1.iter().last().unwrap();
            let mut res = t1;
            if let Some(&(t2, i2)) = set2.iter().last() {
                res = std::cmp::max(t1, t2);
                if t1 > t2 {
                    // use t1
                    let t = t1;
                    let i = i1;
                    set1.remove(&(t, i));
                    if let Some(j) = seconds[i] {
                        // set2 -> set1
                        let t = tv[i][j];
                        set2.remove(&(t, i));
                        set1.insert((t, i));
                        // tv -> set2
                        insert_second(i, j + 1, &tv, &mut seconds, &mut set2);
                    }
                } else {
                    // use t2
                    let t = t2;
                    let i = i2;
                    set2.remove(&(t, i));
                    // tv -> set2
                    let j = seconds[i].unwrap();
                    insert_second(i, j + 1, &tv, &mut seconds, &mut set2);
                }
            } else {
                set1.remove(&(t1, i1));
            }
            println!("{}", res);
        }
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
