pub struct Bit {
  n: usize,
  data: Vec<u64>,
}

// [0, n)
impl Bit {
  pub fn new(n: usize) -> Self {
    Self {
      n,
      data: vec![0; n + 1],
    }
  }

  // 0-origin
  pub fn add(&mut self, i: usize, x: u64) {
    if i >= self.n {
      panic!();
    }
    let mut i = i + 1;
    while i <= self.n {
      self.data[i] += x;
      i += ((i as isize) & -(i as isize)) as usize;
    }
  }

  // 0-origin
  pub fn sum(&self, i: usize) -> u64 {
    if i >= self.n {
      panic!();
    }
    let mut i = i + 1;
    let mut res = 0;
    while i > 0 {
      res += self.data[i];
      i -= ((i as isize) & -(i as isize)) as usize;
    }
    res
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_new() {
    let bit = Bit::new(3);
    assert_eq!(0, bit.sum(0));
    assert_eq!(0, bit.sum(1));
    assert_eq!(0, bit.sum(2));
  }

  #[test]
  fn test_add_sum() {
    let mut bit = Bit::new(3);
    assert_eq!(0, bit.sum(0));
    assert_eq!(0, bit.sum(1));
    assert_eq!(0, bit.sum(2));
    bit.add(0, 1);
    assert_eq!(1, bit.sum(0));
    assert_eq!(1, bit.sum(1));
    assert_eq!(1, bit.sum(2));
    bit.add(1, 2);
    assert_eq!(1, bit.sum(0));
    assert_eq!(3, bit.sum(1));
    assert_eq!(3, bit.sum(2));
    bit.add(2, 3);
    assert_eq!(1, bit.sum(0));
    assert_eq!(3, bit.sum(1));
    assert_eq!(6, bit.sum(2));
  }
}
