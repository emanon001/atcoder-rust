use proconio::input;

fn main() {
  input! {
    n: u64,
    a: u64,
    b: u64
  };
  let ab = a + b;
  let c = n / ab;
  let rest = n % ab;
  let ans = c * a + std::cmp::min(rest, a);
  println!("{}", ans);
}
