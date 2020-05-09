use proconio::input;
use proconio::marker::Usize1;

pub struct Bit {
  n: usize,
  data: Vec<i64>,
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
  pub fn add(&mut self, i: usize, x: i64) {
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
  pub fn sum(&self, i: usize) -> i64 {
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

enum Event {
  // l, r
  Edge(usize, usize),
  // i, l, r
  Query(usize, usize, usize),
}

fn main() {
  input! {
    n: usize, m: usize, q: usize,
    lrv: [(Usize1, Usize1); m],
    prv: [(Usize1, Usize1); q]
  };

  let mut events = lrv
    .into_iter()
    .map(|(l, r)| Event::Edge(l, r))
    .chain(
      prv
        .into_iter()
        .enumerate()
        .map(|(i, (l, r))| Event::Query(i, l, r)),
    )
    .collect::<Vec<_>>();
  events.sort_by_key(|e| match e {
    Event::Edge(_, r) => (*r, 1),
    Event::Query(_, _, r) => (*r, 2),
  });
  let mut bit = Bit::new(n);
  let mut res = vec![0; q];
  for e in events {
    match e {
      Event::Edge(l, _) => {
        bit.add(l, 1);
      }
      Event::Query(i, l, r) => {
        let c = bit.sum(r) - if l == 0 { 0 } else { bit.sum(l - 1) };
        res[i] = c;
      }
    }
  }
  for x in res {
    println!("{}", x);
  }
}
