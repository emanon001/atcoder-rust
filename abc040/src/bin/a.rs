use proconio::input;

fn main() {
  input! {
    n: usize, x: usize
  };

  let res = std::cmp::min(x - 1, n - x);
  println!("{}", res);
}
