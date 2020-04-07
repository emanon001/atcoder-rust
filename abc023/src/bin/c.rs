use proconio::input;
use proconio::marker::Usize1;
use std::collections::{HashMap, HashSet};

fn main() {
  input! {
    r: usize, c: usize, k: usize,
    n: usize,
    pv: [(Usize1, Usize1); n]
  };

  let mut rset = vec![HashSet::new(); r];
  let mut cset = vec![HashSet::new(); c];
  for (i, j) in pv {
    rset[i].insert(j);
    cset[j].insert(i);
  }
  let mut ccmap = HashMap::new();
  for (i, set) in cset.iter().enumerate() {
    let x = ccmap.entry(set.len()).or_insert(HashSet::new());
    x.insert(i);
  }
  let mut res = 0;
  for i in 0..r {
    let rc = rset[i].len();
    if rc > k {
      continue;
    }
    let rest = k - rc;
    // rc + n == k
    if let Some(set) = ccmap.get(&rest) {
      res += set.len() - set.intersection(&rset[i]).count();
    }
    // rc + n + 1 == k
    if let Some(set) = ccmap.get(&(rest + 1)) {
      res += set.intersection(&rset[i]).count();
    }
  }
  println!("{}", res);
}
