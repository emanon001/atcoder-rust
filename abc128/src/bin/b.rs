use proconio::input;

fn main() {
  input! {
    n: usize,
    spv: [(String, isize); n]
  };

  let mut v = spv.into_iter().enumerate().collect::<Vec<_>>();
  v.sort_by_key(|(_, sp)| (sp.0.clone(), -sp.1));
  for (i, _) in v {
    println!("{}", i + 1);
  }
}
