use proconio::input;

pub fn divisors(n: u64) -> Vec<u64> {
  let mut res = Vec::new();
  let mut x = 1;
  while x * x <= n {
    if n % x == 0 {
      res.push(x);
      let y = n / x;
      if y != x {
        res.push(y);
      }
    }
    x += 1;
  }
  res
}

fn main() {
  input! {
    n: usize
  };

  let res = (1..=n)
    .filter(|&a| a % 2 == 1 && divisors(a as u64).len() == 8)
    .count();
  println!("{}", res);
}
