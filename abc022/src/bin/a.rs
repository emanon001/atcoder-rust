use proconio::input;

fn is_ok(w: isize, s: isize, t: isize) -> bool {
  w >= s && w <= t
}

fn main() {
  input! {
    n: usize, s: isize, t: isize,
    w: isize,
    av: [isize; n - 1]
  };

  let mut weight = w;
  let mut res = 0;
  for a in vec![vec![0], av].concat() {
    weight += a;
    if is_ok(weight, s, t) {
      res += 1;
    }
  }
  println!("{}", res);
}
