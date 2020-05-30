#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

pub struct Bit {
    n: usize,
    data: Vec<i64>,
}

// [0, n)
impl Bit {
    pub fn new(n: usize) -> Self {
        Self {
            n,
            data: vec![0; n + 1],
        }
    }

    // 0-origin
    pub fn add(&mut self, i: usize, x: i64) {
        if i >= self.n {
            panic!();
        }
        let mut i = i + 1;
        while i <= self.n {
            self.data[i] += x;
            i += ((i as isize) & -(i as isize)) as usize;
        }
    }

    // [0, i)
    pub fn sum(&self, i: usize) -> i64 {
        if i > self.n {
            panic!();
        }
        let mut i = i;
        let mut res = 0;
        while i > 0 {
            res += self.data[i];
            i -= ((i as isize) & -(i as isize)) as usize;
        }
        res
    }
}

#[derive(Debug)]
enum Input {
    Add(usize, usize),
    Sub(usize, usize, usize),
    Calc(usize),
}

fn main() {
    input! {
        n: usize, w: usize, c: usize,
        lrpv: [(usize, usize, usize); n]
    };

    let mut inputs = Vec::new();
    inputs.push(Input::Calc(c + 1));
    let mut coords = BTreeSet::new();
    coords.insert(c + 1);
    coords.insert(w + 1);
    for &(l, r, p) in &lrpv {
        inputs.push(Input::Add(l + 1, p));
        inputs.push(Input::Sub(r + c + 1, l + 1, p));
        inputs.push(Input::Calc(std::cmp::min(r + c + 1, w + 1)));
        coords.insert(l + 1);
        coords.insert(r + c + 1);
    }
    let coord_table = coords
        .into_iter()
        .enumerate()
        .map(|(i, x)| (x, i))
        .collect::<HashMap<usize, usize>>();
    inputs.sort_by_key(|i| match i {
        Input::Add(x, _) => (*x, 1),
        Input::Sub(x, _, _) => (*x, 2),
        Input::Calc(x) => (*x, 3),
    });
    let mut bit = Bit::new(coord_table.len());
    let mut res = std::i64::MAX;
    for i in inputs {
        match i {
            Input::Add(x, p) => {
                let &x = coord_table.get(&x).unwrap();
                let p = p as i64;
                bit.add(x, p);
            }
            Input::Sub(_, x, p) => {
                let &x = coord_table.get(&x).unwrap();
                let p = p as i64;
                bit.add(x, -p);
            }
            Input::Calc(x) => {
                let &x = coord_table.get(&x).unwrap();
                let sum = bit.sum(x);
                if sum < res {
                    res = sum;
                }
            }
        }
    }
    println!("{}", res);
}
