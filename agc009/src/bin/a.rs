use proconio::input;

fn main() {
  input! {
    n: usize,
    ab_l: [(u64, u64); n]
  };

  let mut res = 0_u64;
  for (a, b) in ab_l.into_iter().rev() {
    let x = (a + res) % b;
    if x > 0 {
      res += b - x;
    }
  }
  println!("{}", res);
}
