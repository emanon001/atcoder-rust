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
    d: usize,
    n: usize,
    m: usize,
    shop_pos: [usize; n - 1],
    dest_pos: [usize; m]
  };

  let mut shop_pos = shop_pos.iter().copied().collect::<Vec<usize>>();
  shop_pos.push(0);
  shop_pos.sort();
  let mut res = 0;
  for d_pos in dest_pos {
    let i = bsearch(0, shop_pos.len() as i32, |i| shop_pos[i as usize] < d_pos) as usize;
    let j = if i == n - 1 { 0 } else { i + 1 };
    let distance = vec![
      d_pos - shop_pos[i],
      if j == 0 { d } else { shop_pos[j] } - d_pos,
      (d - d_pos) - if j == 0 { 0 } else { d - shop_pos[j] },
      (d - shop_pos[i]) - (d - d_pos),
    ]
    .into_iter()
    .min()
    .unwrap();
    res += distance;
  }
  println!("{}", res);
}
