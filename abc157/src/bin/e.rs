use proconio::input;
use proconio::marker::{Chars, Usize1};

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

impl Monoid for u32 {
  fn empty() -> Self {
    0
  }
  fn append(&self, other: &Self) -> Self {
    *self | *other
  }
}

fn main() {
  input! {
    _n: usize,
    s: Chars,
  };

  let st_value = s
    .into_iter()
    .map(|ch| {
      let n: u32 = ch.into();
      let m = n - 97;
      1 << m
    })
    .collect::<Vec<_>>();
  let mut st = SegmentTree::from_slice(&st_value);

  input! { q: usize };
  for _ in 0..q {
    input! { k: usize }
    match k {
      1 => {
        input! { a: Usize1, b: char };
        let b: u32 = b.into();
        let n = b - 97;
        st.update(a, 1 << n);
      }
      2 => {
        input! { a: Usize1, b: usize };
        let x = st.query(a, b);
        let mut res = 0;
        for i in 0..26 {
          if (x >> i) & 1 == 1 {
            res += 1;
          }
        }
        println!("{}", res);
      }
      _ => {}
    }
  }
}
