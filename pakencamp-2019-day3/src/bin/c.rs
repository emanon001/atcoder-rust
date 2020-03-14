use proconio::input;

fn main() {
  input! {
    n: usize, m: usize,
    scores: [[u64; m]; n]
  };

  let mut res = 0_u64;
  for t1 in 0..(m - 1) {
    for t2 in (t1 + 1)..m {
      let mut sum = 0_u64;
      for s in 0..n {
        sum += std::cmp::max(scores[s][t1], scores[s][t2]);
      }
      if sum > res {
        res = sum;
      }
    }
  }
  println!("{}", res);
}
