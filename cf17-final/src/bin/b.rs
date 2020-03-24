use proconio::input;
use proconio::marker::Chars;

fn main() {
  input! {
    s: Chars
  };

  let n = s.len();
  let mut table = std::collections::HashMap::new();
  for ch in s {
    let x = table.entry(ch).or_insert(0);
    *x += 1;
  }
  if table.len() == 1 {
    println!("{}", if n == 1 { "YES" } else { "NO" });
  } else if table.len() == 2 {
    println!("{}", if n == 2 { "YES" } else { "NO" });
  } else {
    let &min = table.values().min().unwrap();
    let &max = table.values().max().unwrap();
    println!("{}", if min + 1 >= max { "YES" } else { "NO" });
  }
}
