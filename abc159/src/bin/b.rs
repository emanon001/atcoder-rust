use proconio::input;

fn main() {
  input! {
    s: String
  };

  let n = s.len() as usize;
  let a = &s[0..((n - 1) / 2)];
  let b = &s[((n + 3) / 2 - 1)..n];
  let res = if s == s.chars().rev().collect::<String>()
    && a == a.chars().rev().collect::<String>()
    && b == b.chars().rev().collect::<String>()
  {
    "Yes"
  } else {
    "No"
  };
  println!("{}", res);
}
