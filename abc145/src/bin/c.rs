use itertools::Itertools;
use proconio::input;

fn main() {
  input! {
    n: usize,
    coordinates: [(i32, i32); n]
  };

  let mut sum = 0_f64;
  for path in coordinates.into_iter().permutations(n) {
    for i in 0..(n - 1) {
      let c1 = path[i];
      let c2 = path[i + 1];
      sum += ((c1.0 - c2.0).pow(2) as f64 + (c1.1 - c2.1).pow(2) as f64).sqrt();
    }
  }
  let res = sum / ((1..=n).fold(1, |acc, x| acc * x) as f64);
  println!("{}", res);
}
