use proconio::input;
use proconio::marker::{Chars, Usize1};

fn is_ok(s: &[char]) -> bool {
  !s.iter().collect::<String>().contains("##")
}

fn main() {
  input! {
    _: usize, a: Usize1, b: Usize1, c: Usize1, d: Usize1,
    s: Chars
  };

  if c < b {
    // a < c < b < d
    let res = if is_ok(&s[a..=c]) && is_ok(&s[b..=d]) {
      "Yes"
    } else {
      "No"
    };
    println!("{}", res);
  } else if c < d {
    // a < b < c < d
    let mut s = s;
    let b_ok = is_ok(&s[b..=d]);
    s[b] = '.';
    let a_ok = is_ok(&s[a..=c]);
    let res = if a_ok && b_ok { "Yes" } else { "No" };
    println!("{}", res);
  } else {
    // a < b < d < c
    let mut s = s;
    let b_ok = is_ok(&s[b..=d])
      && s[(b - 1)..=(d + 1)]
        .iter()
        .collect::<String>()
        .contains("...");
    s[b] = '.';
    let a_ok = is_ok(&s[a..=c]);
    let res = if a_ok && b_ok { "Yes" } else { "No" };
    println!("{}", res);
  }
}
