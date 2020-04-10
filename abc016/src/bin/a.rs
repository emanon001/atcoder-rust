use proconio::input;

fn main() {
  input! {
    m: usize, d: usize
  };

  let res = if m % d == 0 { "YES" } else { "NO" };
  println!("{}", res);
}
