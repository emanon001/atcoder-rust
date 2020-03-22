use proconio::input;

fn main() {
  input! {
    n: u64, a: u64, b: u64
  };

  let diff = ((a as i64) - (b as i64)).abs();
  if diff % 2 == 0 {
    println!("{}", diff / 2);
    std::process::exit(0);
  }

  let d = std::cmp::min(std::cmp::min(a, b) - 1, n - std::cmp::max(a, b));
  let res = if diff == 0 {
    d
  } else {
    d + 1 + ((diff as u64 - 1) / 2)
  };
  println!("{}", res);
}
