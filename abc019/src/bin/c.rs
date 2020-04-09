use proconio::input;

fn main() {
  input! {
    n: usize,
    av: [usize; n]
  };

  let mut set = std::collections::HashSet::new();
  for a in av {
    let mut x = a;
    while x % 2 == 0 {
      x /= 2;
    }
    set.insert(x);
  }
  println!("{}", set.len());
}
