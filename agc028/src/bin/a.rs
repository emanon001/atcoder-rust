use num::*;
use proconio::input;
use proconio::marker::Chars;

fn main() {
  input! {
    n: usize, m: usize,
    s: Chars,
    t: Chars,
  };

  let lcm = n.lcm(&m);
  let gcd = n.gcd(&m);
  let a = n / gcd;
  let b = m / gcd;
  let mut i = 0;
  let mut j = 0;
  while i < n && j < m {
    if s[i] != t[j] {
      println!("-1");
      std::process::exit(0);
    }
    i += a;
    j += b;
  }
  println!("{}", lcm);
}
