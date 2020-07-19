#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

fn main() {
    input! {
        n: usize, k: usize, c: usize,
        s: Chars
    };

    let mut forward_days = HashMap::new();
    let mut last_ng_day = -1;
    let mut cur_day = 1;
    for i in 0..n {
        if cur_day > k {
            break;
        }
        if s[i] == 'x' {
            continue;
        }
        if i as isize <= last_ng_day {
            continue;
        }
        forward_days.insert(cur_day, i);
        cur_day += 1;
        last_ng_day = (i + c) as isize;
    }
    let mut backward_days = HashMap::new();
    let mut last_ng_day = n as isize;
    let mut cur_day = k;
    for i in (0..n).rev() {
        if cur_day == 0 {
            break;
        }
        if s[i] == 'x' {
            continue;
        }
        if i as isize >= last_ng_day {
            continue;
        }
        backward_days.insert(cur_day, i);
        cur_day -= 1;
        last_ng_day = i as isize - c as isize;
    }
    for step in 1..k + 1 {
        let li = forward_days.get(&step).unwrap();
        let ri = backward_days.get(&step).unwrap();
        if li == ri {
            println!("{}", li + 1);
        }
    }
}
