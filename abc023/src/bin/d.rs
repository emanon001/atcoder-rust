use proconio::input;
use std::collections::HashMap;

fn bsearch<F>(mut ok: i64, mut ng: i64, mut pred: F) -> i64
where
  F: FnMut(i64) -> bool,
{
  while (ok - ng).abs() > 1 {
    let mid = (ok + ng) / 2;
    if pred(mid) {
      ok = mid;
    } else {
      ng = mid;
    }
  }
  ok
}

fn main() {
  input! {
    n: usize,
    baloons: [(i64, i64); n]
  };

  let ok_h = 1_000_000_000_i64 + (1_000_000_000 * 100_000) + 1;
  let res = bsearch(ok_h, 0, |max_h| {
    let mut table = HashMap::new();
    for &(h, s) in &baloons {
      if h > max_h {
        return false;
      }
      let key = (max_h - h) / s;
      let v = table.entry(key).or_insert(0);
      *v += 1;
    }
    let mut vec = table.into_iter().collect::<Vec<_>>();
    vec.sort_by(|x, y| x.0.cmp(&y.0));
    let mut elapsed_sec = -1;
    for (limit, c) in vec {
      elapsed_sec += c;
      if elapsed_sec > limit {
        return false;
      }
    }
    true
  });
  println!("{}", res);
}
