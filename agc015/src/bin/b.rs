use proconio::input;
use proconio::marker::Chars;

fn main() {
  input! {
    s: Chars
  };

  let n = s.len() as u64;
  let mut res = 0_u64;
  for (i, ch) in s.into_iter().enumerate() {
    let i = i as u64;
    if ch == 'U' {
      res += i * 2;
      res += n - i - 1;
    } else {
      res += i;
      res += (n - i - 1) * 2;
    }
  }
  println!("{}", res);
}
