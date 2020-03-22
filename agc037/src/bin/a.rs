use proconio::input;

fn main() {
  input! {
    s: String
  };

  let mut res = 0;
  let mut prev = Vec::new();
  let mut cur = Vec::new();
  for ch in s.chars() {
    cur.push(ch);
    if cur != prev {
      res += 1;
      prev = cur;
      cur = Vec::new();
    }
  }
  println!("{}", res);
}
