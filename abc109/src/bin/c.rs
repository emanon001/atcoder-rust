use num::*;
use proconio::input;

fn main() {
  input! {
    n: usize, x: i32,
    xs: [i32; n]
  };

  let distances = xs.into_iter().map(|a| abs(a - x)).collect::<Vec<_>>();
  let first = distances[0];
  let res = distances.into_iter().fold(first, |acc, x| acc.gcd(&x));
  println!("{}", res);
}
