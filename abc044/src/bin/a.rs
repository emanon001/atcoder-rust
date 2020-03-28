use proconio::input;

fn main() {
  input! {
    n: usize,
    k: usize,
    x: usize,
    y: usize,
  };

  let d = std::cmp::min(n, k);
  let mut res = d * x;
  if n - d > 0 {
    res += (n - d) * y;
  }
  println!("{}", res);
}
