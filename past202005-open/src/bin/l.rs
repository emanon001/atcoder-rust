#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

fn shift(
    i: usize,
    j: usize,
    htype: usize,
    tv: &[Vec<usize>],
    t_to_htype: &mut HashMap<usize, usize>,
    seconds: &mut Vec<Option<usize>>,
    heap: &mut BinaryHeap<(usize, usize)>,
) {
    let is_ok = j < tv[i].len();
    if is_ok {
        let t = tv[i][j];
        t_to_htype.insert(t, htype);
        heap.push((t, i));
    }
    if htype == 2 {
        seconds[i] = if is_ok { Some(j) } else { None };
    }
}

fn solve() {
    input! {
        n: usize,
        tv: [[usize]; n],
        m: usize,
        av: [usize; m]
    };

    let mut t_to_htype = HashMap::new();
    let mut seconds = vec![None; n];
    let mut heap1 = BinaryHeap::new();
    for i in 0..n {
        let t = tv[i][0];
        heap1.push((t, i));
        t_to_htype.insert(t, 1);
    }
    let mut heap2 = BinaryHeap::new();
    for i in 0..n {
        if tv[i].len() > 1 {
            let t = tv[i][1];
            heap2.push((t, i));
            t_to_htype.insert(t, 2);
            seconds[i] = Some(1);
        }
    }
    for a in av {
        if a == 1 {
            let (t, i) = heap1.pop().unwrap();
            println!("{}", t);
            if let Some(j) = seconds[i] {
                shift(i, j, 1, &tv, &mut t_to_htype, &mut seconds, &mut heap1);
                shift(i, j + 1, 2, &tv, &mut t_to_htype, &mut seconds, &mut heap2);
            }
        } else {
            let (t1, i1) = heap1.pop().unwrap();
            let mut res = t1;
            while let Some((t2, i2)) = heap2.pop() {
                if t_to_htype.get(&t2).unwrap() == &1 {
                    continue;
                }
                if t1 > t2 {
                    // use t1
                    heap2.push((t2, i2));
                    if let Some(second) = seconds[i1] {
                        shift(
                            i1,
                            second,
                            1,
                            &tv,
                            &mut t_to_htype,
                            &mut seconds,
                            &mut heap1,
                        );
                        shift(
                            i1,
                            second + 1,
                            2,
                            &tv,
                            &mut t_to_htype,
                            &mut seconds,
                            &mut heap2,
                        );
                    }
                } else {
                    // use t2
                    heap1.push((t1, i1));
                    let second = seconds[i2].unwrap();
                    shift(
                        i2,
                        second + 1,
                        2,
                        &tv,
                        &mut t_to_htype,
                        &mut seconds,
                        &mut heap2,
                    );
                }
                res = std::cmp::max(t1, t2);
                break;
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
