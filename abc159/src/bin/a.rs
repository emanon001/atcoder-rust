use proconio::input;

fn main() {
  input! {
    n: usize, m: usize
  };

  let a = if n <= 1 { 0 } else { n * (n - 1) / 2 };
  let b = if m <= 1 { 0 } else { m * (m - 1) / 2 };
  let res = a + b;
  println!("{}", res);
}
