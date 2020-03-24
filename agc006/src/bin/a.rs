use proconio::input;

fn main() {
  input! {
    n: usize,
    s: String,
    t: String
  };

  let mut c = 0;
  for i in 1..=n {
    if s[(n - i)..] == t[..i] {
      c = i;
    }
  }
  let res = n * 2 - c;
  println!("{}", res);
}
