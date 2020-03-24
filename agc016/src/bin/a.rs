use proconio::input;

fn main() {
  input! {
    s: String
  };

  let mut res = 1 << 30;
  for ch in "abcdefghijklmnopqrstuvwxyz".chars() {
    if let Some(max) = s.split(ch).map(|w| w.len()).max() {
      if max < res {
        res = max;
      }
    }
  }
  println!("{}", res);
}
