use proconio::input;
use proconio::marker::Usize1;

fn main() {
  input! {
    n: usize, m: usize,
    abv: [(Usize1, Usize1); m]
  };

  let mut friend_set = vec![std::collections::HashSet::new(); n];
  for (a, b) in abv {
    friend_set[a].insert(b);
    friend_set[b].insert(a);
  }
  for i in 0..n {
    let mut res = std::collections::HashSet::new();
    for &f in &friend_set[i] {
      for &f2 in &friend_set[f] {
        res.insert(f2);
      }
    }
    for f in &friend_set[i] {
      res.remove(f);
    }
    res.remove(&i);
    println!("{}", res.len());
  }
}
