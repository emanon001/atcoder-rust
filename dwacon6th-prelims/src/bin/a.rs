use proconio::input;

fn main() {
  input! {
    n: usize,
    stl: [(String, usize); n],
    x: String
  };

  let mut res = 0;
  let mut sleeping = false;
  for (s, t) in stl {
    if sleeping {
      res += t;
    } else if s == x {
      sleeping = true;
    }
  }
  println!("{}", res);
}
