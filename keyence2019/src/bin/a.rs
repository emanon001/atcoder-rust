use proconio::input;

fn main() {
  input! {
    n1: usize, n2: usize, n3: usize, n4: usize
  };
  let mut set = std::collections::HashSet::new();
  set.insert(n1);
  set.insert(n2);
  set.insert(n3);
  set.insert(n4);
  let res = if set.contains(&1) && set.contains(&9) && set.contains(&7) && set.contains(&4) {
    "YES"
  } else {
    "NO"
  };
  println!("{}", res);
}
