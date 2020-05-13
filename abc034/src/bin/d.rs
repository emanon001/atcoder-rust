use ordered_float::OrderedFloat;
use proconio::input;

pub fn bsearch<F>(ok: f64, ng: f64, pred: F) -> Option<f64>
where
  F: Fn(f64) -> bool,
{
  let orig_ok = ok;
  let mut ok = ok;
  let mut ng = ng;
  while (ok - ng).abs() > 0.000001 {
    let mid = (ok + ng) / 2.0;
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
    n: usize, k: usize,
    wpv: [(isize, isize); n]
  };

  let res = bsearch(-0.000001, 100.000001, |x| {
    let mut v = wpv
      .iter()
      .map(|&(w, p)| {
        let f = (w as f64 * (p as f64 / 100 as f64)) - (w as f64 * (x as f64 / 100 as f64));
        OrderedFloat::from(f)
      })
      .collect::<Vec<_>>();
    v.sort_by_key(|&a| OrderedFloat::from(-a.into_inner()));
    let mut res = 0_f64;
    for i in 0..k {
      res += v[i].into_inner();
    }
    res >= 0.0
  })
  .unwrap();
  println!("{}", res);
}
