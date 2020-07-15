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
        av: [isize; n * 3]
    };

    // left
    let mut left_sum = 0;
    let mut left_heap = BinaryHeap::new();
    for &a in &av[0..n] {
        left_sum += a;
        left_heap.push(std::cmp::Reverse(a));
    }
    let mut left_cumax = vec![0; n + 1];
    left_cumax[0] = left_sum;
    for i in n..n * 2 {
        let a = av[i];
        let std::cmp::Reverse(left_min) = *left_heap.peek().unwrap();
        if a > left_min {
            left_heap.pop();
            left_heap.push(std::cmp::Reverse(a));
            left_sum += a - left_min;
        }
        left_cumax[i - (n - 1)] = left_sum;
    }

    // right
    let mut right_sum = 0;
    let mut right_heap = BinaryHeap::new();
    for &a in &av[n * 2..] {
        right_sum += a;
        right_heap.push(a);
    }
    let mut right_cumin = vec![0; n + 1];
    right_cumin[0] = right_sum;
    for i in (n..n * 2).rev() {
        let a = av[i];
        let right_max = *right_heap.peek().unwrap();
        if a < right_max {
            right_heap.pop();
            right_heap.push(a);
            right_sum += a - right_max;
        }
        right_cumin[n * 2 - i] = right_sum;
    }

    // calc
    let mut res = std::isize::MIN;
    for i in 0..n + 1 {
        let score = left_cumax[i] - right_cumin[n - i];
        if score > res {
            res = score;
        }
    }
    println!("{}", res);
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
