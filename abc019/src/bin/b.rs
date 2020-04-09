use proconio::input;
use proconio::marker::Chars;

fn main() {
  input! {
    s: Chars
  };

  let mut res = "".to_string();
  let mut c = 1;
  let mut prev = s[0];
  for &ch in &s[1..] {
    if ch == prev {
      c += 1;
    } else {
      res = res + &format!("{}{}", prev, c);
      c = 1;
    }
    prev = ch;
  }
  res = res + &format!("{}{}", prev, c);
  println!("{}", res);
}
