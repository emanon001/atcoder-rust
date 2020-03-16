use itertools::Itertools;
use proconio::input;

fn main() {
  input! {
    n: usize,
    ps: [usize; n],
    qs: [usize; n]
  };

  let mut vec = (1..=n).permutations(n).collect::<Vec<Vec<usize>>>();
  vec.sort();
  let pi = vec.iter().position(|v| v == &ps).unwrap();
  let qi = vec.iter().position(|v| v == &qs).unwrap();
  let res = (pi as isize - qi as isize).abs();
  println!("{}", res);
}
