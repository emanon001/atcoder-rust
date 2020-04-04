use proconio::input;

fn main() {
  input! {
    n: usize, m: isize,
    av: [isize; n]
  };

  let sum: isize = av.iter().sum();
  let mut c = 0;
  for a in av {
    if a * 4 * m >= sum {
      c += 1;
    }
  }
  let res = if c >= m { "Yes" } else { "No" };
  println!("{}", res);
}
