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
    n: i64, a: i64, b: i64,
    hv: [i64; n]
  };

  let ng = 0;
  let ok = 1_000_000_000 / b + 10;
  let res = bsearch(ok, ng, |x| {
    let bd = x * b;
    let ad = a - b;
    let mut c = 0;
    for &h in &hv {
      if h > bd {
        let rest = h - bd;
        c += (rest + ad - 1) / ad;
      }
    }
    c <= x
  })
  .unwrap();
  println!("{}", res);
}
