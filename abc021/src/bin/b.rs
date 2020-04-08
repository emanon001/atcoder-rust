use proconio::input;

fn main() {
  input! {
    _: usize,
    a: usize, b: usize,
    k: usize,
    pv: [usize; k]
  };

  let mut map = std::collections::HashMap::new();
  for p in pv {
    let c = map.entry(p).or_insert(0);
    *c += 1;
  }
  if map.contains_key(&a) || map.contains_key(&b) {
    println!("NO");
  } else if map.values().any(|&c| c > 1) {
    println!("NO");
  } else {
    println!("YES");
  }
}
