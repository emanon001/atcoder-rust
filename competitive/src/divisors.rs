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

#[cfg(test)]
mod tests {
  use super::divisors;

  #[test]
  fn test_devisors() {
    assert!(divisors(0).is_empty());
    assert_eq!(sort(divisors(1)), vec![1]);
    assert_eq!(sort(divisors(2)), vec![1, 2]);
    assert_eq!(sort(divisors(4)), vec![1, 2, 4]);
    assert_eq!(sort(divisors(15)), vec![1, 3, 5, 15]);
  }

  fn sort(mut v: Vec<u64>) -> Vec<u64> {
    v.sort();
    v
  }
}
