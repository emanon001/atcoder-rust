use proconio::input;
use proconio::marker::Usize1;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct ModInt(u32);

const MOD: u32 = 1_000_000_007;

impl ModInt {
  pub fn inv(self) -> Self {
    if self.0 == 0 {
      panic!();
    }
    self.pow(MOD - 2)
  }

  pub fn pow(self, e: u32) -> Self {
    if e == 0 {
      return Self::new(1);
    }
    let mut res = self.pow(e >> 1);
    res *= res;
    if e & 1 == 1 {
      res *= self;
    }
    res
  }

  fn new(n: i64) -> Self {
    let mut n = n % (MOD as i64);
    if n.is_negative() {
      n += MOD as i64;
    }
    Self(n as u32)
  }
}

impl From<i32> for ModInt {
  fn from(n: i32) -> Self {
    ModInt::from(n as i64)
  }
}

impl From<i64> for ModInt {
  fn from(n: i64) -> Self {
    Self::new(n)
  }
}

impl From<isize> for ModInt {
  fn from(n: isize) -> Self {
    ModInt::from(n as i64)
  }
}

impl From<u32> for ModInt {
  fn from(n: u32) -> Self {
    ModInt::from(n as u64)
  }
}

impl From<u64> for ModInt {
  fn from(n: u64) -> Self {
    Self::new(n as i64)
  }
}

impl From<usize> for ModInt {
  fn from(n: usize) -> Self {
    ModInt::from(n as u64)
  }
}

impl Into<i32> for ModInt {
  fn into(self) -> i32 {
    self.0 as i32
  }
}

impl Into<i64> for ModInt {
  fn into(self) -> i64 {
    self.0 as i64
  }
}

impl Into<isize> for ModInt {
  fn into(self) -> isize {
    self.0 as isize
  }
}

impl Into<u32> for ModInt {
  fn into(self) -> u32 {
    self.0
  }
}

impl Into<u64> for ModInt {
  fn into(self) -> u64 {
    self.0 as u64
  }
}

impl Into<usize> for ModInt {
  fn into(self) -> usize {
    self.0 as usize
  }
}

impl std::fmt::Display for ModInt {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.0)
  }
}

impl std::ops::Add for ModInt {
  type Output = Self;

  fn add(self, rhs: Self) -> Self {
    Self::new((self.0 + rhs.0) as i64)
  }
}

impl std::ops::AddAssign for ModInt {
  fn add_assign(&mut self, rhs: Self) {
    *self = *self + rhs;
  }
}

impl std::ops::Div for ModInt {
  type Output = Self;

  fn div(self, rhs: Self) -> Self {
    self * rhs.inv()
  }
}

impl std::ops::DivAssign for ModInt {
  fn div_assign(&mut self, rhs: Self) {
    *self = *self / rhs;
  }
}

impl std::ops::Mul for ModInt {
  type Output = Self;

  fn mul(self, rhs: Self) -> Self {
    Self::new((self.0 as i64) * (rhs.0 as i64))
  }
}

impl std::ops::MulAssign for ModInt {
  fn mul_assign(&mut self, rhs: Self) {
    *self = *self * rhs;
  }
}

impl std::ops::Sub for ModInt {
  type Output = Self;

  fn sub(self, rhs: Self) -> Self {
    Self::new((self.0 as i64) - (rhs.0 as i64))
  }
}

impl std::ops::SubAssign for ModInt {
  fn sub_assign(&mut self, rhs: Self) {
    *self = *self - rhs;
  }
}

pub struct Graph {
  graph: Vec<Vec<usize>>,
  v: usize,
}

impl Graph {
  pub const INF: usize = std::usize::MAX;

  pub fn new(edges: &[(usize, usize)], v: usize) -> Self {
    let mut graph = vec![Vec::new(); v];
    for &(u, v) in edges {
      graph[u].push(v);
      graph[v].push(u);
    }
    Self { graph, v }
  }

  pub fn shortest_path_count(&self, start: usize) -> Vec<ModInt> {
    let mut count_list = vec![ModInt::from(0); self.v];
    let mut cost_list = vec![Self::INF; self.v];
    let mut que = std::collections::VecDeque::new();
    count_list[start] = ModInt::from(1);
    cost_list[start] = 0;
    que.push_back((start, 0));
    while let Some((u, c)) = que.pop_front() {
      for &v in &self.graph[u] {
        let new_cost = c + 1;
        if new_cost > cost_list[v] {
          continue;
        }
        if new_cost < cost_list[v] {
          cost_list[v] = new_cost;
          count_list[v] = count_list[u];
          que.push_back((v, new_cost));
        } else {
          let new_count = count_list[v] + count_list[u];
          count_list[v] = new_count;
        }
      }
    }
    count_list
  }
}

fn main() {
  input! {
    n: usize,
    a: Usize1, b: Usize1,
    m: usize,
    edges: [(Usize1, Usize1); m]
  };

  let graph = Graph::new(&edges, n);
  let count_list = graph.shortest_path_count(a);
  println!("{}", count_list[b]);
}
