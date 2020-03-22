use proconio::input;
use proconio::marker::Chars;

fn main() {
  input! {
    s: Chars,
    w: usize
  };

  let mut res = Vec::new();
  let mut i = 0;
  let len = s.len();
  while i < len {
    res.push(s[i]);
    i += w;
  }
  println!("{}", res.into_iter().collect::<String>());
}
