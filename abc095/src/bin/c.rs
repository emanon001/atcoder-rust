use proconio::input;

fn main() {
  input! {
    a: i64, b: i64, c: i64, x: i64, y: i64
  };

  let mut res = std::i64::MAX;
  for cc in 0..=(std::cmp::max(x, y) * 2) {
    let ac = std::cmp::max(x - cc / 2, 0);
    let bc = std::cmp::max(y - cc / 2, 0);
    let sum = a * ac + b * bc + c * cc;
    if sum < res {
      res = sum;
    }
  }
  println!("{}", res);
}
