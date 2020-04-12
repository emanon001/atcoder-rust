#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;

fn count(v: &[usize], i: usize, k: usize) -> usize {
  let c = v[i];
  if k >= v.len() - 1 {
    return c;
  }
  c - (v[k] - v[k + 1])
}

fn main() {
  input! {
    n: usize,
    s: Chars
  };

  let mut rc = vec![0; n + 1];
  let mut gc = vec![0; n + 1];
  let mut bc = vec![0; n + 1];

  for (i, &ch) in s.iter().rev().enumerate() {
    let i = n - i;
    rc[i - 1] = rc[i];
    gc[i - 1] = gc[i];
    bc[i - 1] = bc[i];
    match ch {
      'R' => rc[i - 1] += 1,
      'G' => gc[i - 1] += 1,
      'B' => bc[i - 1] += 1,
      _ => {}
    }
  }
  let mut res = 0;
  for i in 0..n {
    for j in (i + 1)..n {
      if j + 1 == n {
        continue;
      }
      let l = j + (j - i);
      let c = match (s[i], s[j]) {
        ('R', 'G') => count(&bc, j + 1, l),
        ('R', 'B') => count(&gc, j + 1, l),
        ('G', 'R') => count(&bc, j + 1, l),
        ('G', 'B') => count(&rc, j + 1, l),
        ('B', 'R') => count(&gc, j + 1, l),
        ('B', 'G') => count(&rc, j + 1, l),
        _ => 0,
      };
      res += c;
    }
  }
  println!("{}", res);
}
