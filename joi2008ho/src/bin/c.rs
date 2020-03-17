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
    n: usize, m: usize,
    mut ps: [i64; n]
  };

  let one_ps = ps.iter().copied().collect::<Vec<_>>();
  let mut two_ps = Vec::new();
  for &p1 in &ps {
    for &p2 in &ps {
      two_ps.push(p1 + p2);
    }
  }
  two_ps.sort();
  let mut res = 0;
  for &p1 in &one_ps {
    if p1 > m as i64 {
      continue;
    }
    if p1 > res {
      res = p1;
    }
    let i = bsearch(-1, two_ps.len() as i64, |i| {
      p1 + two_ps[i as usize] <= m as i64
    });
    if let Some(i) = i {
      let p3 = p1 + two_ps[i as usize];
      if p3 > res {
        res = p3;
      }
    }
  }
  for &p2 in &two_ps {
    if p2 > m as i64 {
      continue;
    }
    if p2 > res {
      res = p2;
    }
    let i = bsearch(-1, two_ps.len() as i64, |i| {
      p2 + two_ps[i as usize] <= m as i64
    });
    if let Some(i) = i {
      let p4 = p2 + two_ps[i as usize];
      if p4 > res {
        res = p4;
      }
    }
  }
  println!("{}", res);
}
