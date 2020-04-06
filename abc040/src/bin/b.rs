use proconio::input;

fn main() {
  input! {
    n: usize
  };

  let mut res = std::isize::MAX;
  let mut x = 1_isize;
  let n = n as isize;
  while x * x <= n {
    let sum = (x - n / x).abs() + n % x;
    if sum < res {
      res = sum;
    }
    x += 1;
  }
  println!("{}", res);
}
