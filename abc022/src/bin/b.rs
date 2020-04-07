use proconio::input;

fn main() {
  input! {
    n: usize,
    av: [usize; n]
  };

  let mut map = std::collections::HashMap::new();
  for a in av {
    let c = map.entry(a).or_insert(0_usize);
    *c += 1;
  }
  let res: usize = map.values().into_iter().map(|c| c - 1).sum();
  println!("{}", res);
}
