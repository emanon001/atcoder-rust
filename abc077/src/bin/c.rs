use proconio::input;

fn bsearch<F>(mut ok: i32, mut ng: i32, mut pred: F) -> i32
where
  F: FnMut(i32) -> bool,
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
    a_v: [usize; n],
    mut b_v: [usize; n],
    mut c_v: [usize; n]
  };

  b_v.sort();
  c_v.sort();

  let mut cusum = vec![0; n + 1];
  for i in (0..n).rev() {
    let b = b_v[i];
    let ci = bsearch(n as i32, -1, |j| c_v[j as usize] > b);
    let count = n - ci as usize;
    cusum[i] = cusum[i + 1] + count;
  }

  let mut res = 0;
  for i in 0..n {
    let a = a_v[i];
    let bi = bsearch(n as i32, -1, |i| b_v[i as usize] > a);
    res += cusum[bi as usize];
  }
  println!("{}", res);
}
