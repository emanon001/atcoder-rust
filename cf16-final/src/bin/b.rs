use proconio::input;

pub fn bsearch<F>(ok: i64, ng: i64, pred: F) -> Option<i64>
where
  F: Fn(i64) -> bool,
{
  let orig_ok = ok;
  let mut ok = ok;
  let mut ng = ng;
  while (ok - ng).abs() > 1 {
    let mid = (ok + ng) / 2;
    if pred(mid) {
      ok = mid;
    } else {
      ng = mid;
    }
  }
  if ok == orig_ok {
    None
  } else {
    Some(ok)
  }
}

fn main() {
  input! {
    n: i64
  };

  let m = bsearch(n + 1, 0, |x| (x + 1) * x / 2 >= n).unwrap();
  let diff = ((m + 1) * m / 2) - n;
  for x in 1..=m {
    if x == diff {
      continue;
    }
    println!("{}", x);
  }
}
