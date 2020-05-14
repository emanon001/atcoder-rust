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
    x: usize, y: usize, z: usize, k: usize,
    mut av: [u64; x],
    mut bv: [u64; y],
    mut cv: [u64; z],
  };

  av.sort_by_key(|&a| -(a as isize));
  bv.sort_by_key(|&b| -(b as isize));
  cv.sort_by_key(|&c| -(c as isize));

  let mut c = 0;
  let mut heap = BinaryHeap::new();
  heap.push((av[0] + bv[0] + cv[0], 0, 0, 0));
  let mut used = HashSet::new();
  used.insert((0, 0, 0));
  while let Some((p, ai, bi, ci)) = heap.pop() {
    println!("{}", p);
    c += 1;
    if c >= k {
      break;
    }
    let mut push = |ai: usize, bi: usize, ci: usize| {
      let p = av[ai] + bv[bi] + cv[ci];
      if !used.contains(&(ai, bi, ci)) {
        heap.push((p, ai, bi, ci));
        used.insert((ai, bi, ci));
      }
    };
    // ai + 1, bi, ci
    if ai + 1 < x {
      let next_ai = ai + 1;
      let next_bi = bi;
      let next_ci = ci;
      push(next_ai, next_bi, next_ci);
    }
    // ai, bi + 1, ci
    if bi + 1 < x {
      let next_ai = ai;
      let next_bi = bi + 1;
      let next_ci = ci;
      push(next_ai, next_bi, next_ci);
    }
    // ai, bi, ci + 1
    if ci + 1 < x {
      let next_ai = ai;
      let next_bi = bi;
      let next_ci = ci + 1;
      push(next_ai, next_bi, next_ci);
    }
  }
}
