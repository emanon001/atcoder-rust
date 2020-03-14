use proconio::input;

fn divisors_count(n: i32) -> i32 {
  let mut res = 0;
  let mut x = 1;
  while x * x <= n {
    if n % x == 0 {
      res += 1;
      if n / x != x {
        res += 1;
      }
    }
    x += 1;
  }
  res
}

fn main() {
  input! {
    n: i32
  };
  let ans = (1..=n)
    .filter(|x| x % 2 == 1)
    .filter(|x| divisors_count(*x) == 8)
    .count();
  println!("{}", ans);
}
