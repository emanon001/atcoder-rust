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
    n: u64
  };

  let mut res = 1;
  let pf = prime_factor(n);
  if pf.len() == 1 {
    for (&k, &v) in &pf {
      let mut x = k;
      let mut y = 1;
      while x <= n && y <= v {
        if n % x == 0 {
          res += 1;
        }
        x *= x;
        y += 1;
      }
    }
  }
  res += 1;

  println!("{}", res);
}
