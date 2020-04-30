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
    n: usize,
    dv: [isize; n]
  };

  let cusum = dv
    .iter()
    .scan(0_isize, |acc, &d| {
      *acc += d;
      Some(*acc)
    })
    .collect::<Vec<_>>();
  let max: isize = dv.iter().sum();
  let min = if n == 1 {
    dv[0]
  } else if n == 2 {
    (dv[0] - dv[1]).abs()
  } else {
    let mut res = 1_isize << 60;
    for i in 0..n {
      for j in i + 1..n {
        if j == n - 1 {
          continue;
        }
        let all = cusum[n - 1];
        let a = cusum[i];
        let b = cusum[j] - a;
        let c = all - a - b;
        let m = vec![a, b, c].into_iter().max().unwrap();
        let d = if m >= all - m { m - (all - m) } else { 0 };
        if d < res {
          res = d;
        }
      }
    }
    res
  };
  println!("{}", max);
  println!("{}", min);
}
