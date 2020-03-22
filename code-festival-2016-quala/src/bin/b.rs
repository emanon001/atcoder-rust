use proconio::input;
use proconio::marker::Usize1;

fn main() {
  input! {
    n: usize,
    al: [Usize1; n]
  };

  let mut res = 0;
  for i in 0..n {
    if i == al[al[i]] {
      res += 1;
    }
  }
  println!("{}", res / 2);
}
