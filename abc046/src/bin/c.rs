use proconio::input;

fn main() {
  input! {
    n: usize,
    tav: [(u64, u64); n]
  };

  let mut tc = 1_u64;
  let mut ac = 1_u64;
  for (t, a) in tav {
    let x = std::cmp::max(
      if tc % t == 0 { tc / t } else { tc / t + 1 },
      if ac % a == 0 { ac / a } else { ac / a + 1 },
    );
    tc = t * x;
    ac = a * x;
  }
  println!("{}", tc + ac);
}
