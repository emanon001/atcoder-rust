use proconio::input;
use proconio::marker::Chars;

fn main() {
  input! {
    s: Chars
  };

  let mut res = 0_isize;
  let mut i = 0;
  let mut j = s.len() - 1;
  while i < j {
    if s[i] == s[j] {
      i += 1;
      j -= 1;
    } else if s[i] == 'x' {
      i += 1;
      res += 1;
    } else if s[j] == 'x' {
      j -= 1;
      res += 1;
    } else {
      res = -1;
      break;
    }
  }
  println!("{}", res);
}
