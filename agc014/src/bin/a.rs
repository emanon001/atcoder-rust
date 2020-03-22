use proconio::input;

pub fn prime_factor(n: u64) -> std::collections::HashMap<u64, u64> {
  if n < 2 {
    return std::collections::HashMap::new();
  }
  let mut res = std::collections::HashMap::new();
  let mut n = n;
  let mut i = 2;
  while i * i <= n {
    while n % i == 0 {
      let c = res.entry(i).or_insert(0);
      *c += 1;
      n /= i;
    }
    i += 1;
  }
  res.insert(n, 1);
  res
}

fn main() {
  input! {
    a: i64, b: i64, c: i64
  };

  if a % 2 == 1 || b % 2 == 1 || c % 2 == 1 {
    println!("0");
    std::process::exit(0);
  }

  if a == b && b == c {
    println!("-1");
    std::process::exit(0);
  }

  let d1 = (a - b).abs() as u64;
  let d2 = (b - c).abs() as u64;
  let mut res = 1 << 60;
  if let Some(&c) = prime_factor(d1).get(&2) {
    if c < res {
      res = c;
    }
  }
  if let Some(&c) = prime_factor(d2).get(&2) {
    if c < res {
      res = c;
    }
  }
  println!("{}", res);
}
