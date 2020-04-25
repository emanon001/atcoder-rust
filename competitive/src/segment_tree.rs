pub trait Monoid {
  fn empty() -> Self;
  fn append(&self, other: &Self) -> Self;
}

pub struct SegmentTree<T>
where
  T: Monoid + Clone,
{
  size: usize,
  data: Vec<T>,
}

impl<T> SegmentTree<T>
where
  T: Monoid + Clone,
{
  pub fn new(size: usize) -> Self {
    let size = Self::normalize_data_size(size);
    let data = vec![T::empty(); size * 2 - 1];
    Self { size, data }
  }

  pub fn from_slice(values: &[T]) -> Self {
    let mut st = SegmentTree::new(values.len());
    for (i, v) in values.into_iter().enumerate() {
      st.data[i + st.size - 1] = v.clone();
    }
    if st.size < 2 {
      return st;
    }
    for i in (0..=(st.size - 2)).rev() {
      st.data[i] = st.data[i * 2 + 1].append(&st.data[i * 2 + 2]);
    }
    st
  }

  // 0-origin
  pub fn update(&mut self, i: usize, v: T) {
    let mut i = i + self.size - 1;
    self.data[i] = v;
    while i > 0 {
      i = (i - 1) / 2;
      self.data[i] = self.data[i * 2 + 1].append(&self.data[i * 2 + 2]);
    }
  }

  // [a, b)
  // 0-origin
  pub fn query(&self, a: usize, b: usize) -> T {
    self.execute_query(a, b, 0, 0, self.size)
  }

  fn normalize_data_size(size: usize) -> usize {
    let mut n = 1;
    while n < size {
      n *= 2;
    }
    n
  }

  fn execute_query(&self, a: usize, b: usize, i: usize, l: usize, r: usize) -> T {
    if r <= a || b <= l {
      return T::empty();
    }
    if a <= l && r <= b {
      return self.data[i].clone();
    }
    let vl = self.execute_query(a, b, i * 2 + 1, l, (l + r) / 2);
    let vr = self.execute_query(a, b, i * 2 + 2, (l + r) / 2, r);
    vl.append(&vr)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  // max
  impl Monoid for usize {
    fn empty() -> Self {
      0
    }
    fn append(&self, other: &Self) -> Self {
      std::cmp::max(*self, *other)
    }
  }

  #[test]
  fn new() {
    let st: SegmentTree<usize> = SegmentTree::new(3);
    assert_eq!(0, st.query(0, 3));
  }

  #[test]
  fn from_slice() {
    let v = vec![1, 3, 2];
    let st = SegmentTree::from_slice(&v);
    assert_eq!(1, st.query(0, 1));
    assert_eq!(3, st.query(0, 2));
    assert_eq!(3, st.query(0, 3));
    assert_eq!(3, st.query(1, 2));
    assert_eq!(3, st.query(1, 3));
    assert_eq!(2, st.query(2, 3));
  }

  #[test]
  fn from_slice_size1() {
    let v = vec![1];
    let st = SegmentTree::from_slice(&v);
    assert_eq!(1, st.query(0, 1));
  }

  #[test]
  fn update_and_query() {
    let mut st: SegmentTree<usize> = SegmentTree::new(3);
    assert_eq!(0, st.query(0, 3));
    st.update(0, 1);
    st.update(1, 3);
    st.update(2, 2);

    assert_eq!(1, st.query(0, 1));
    assert_eq!(3, st.query(0, 2));
    assert_eq!(3, st.query(0, 3));
    assert_eq!(3, st.query(1, 2));
    assert_eq!(3, st.query(1, 3));
    assert_eq!(2, st.query(2, 3));
  }
}
