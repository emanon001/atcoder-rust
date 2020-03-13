use proconio::input;

fn main() {
  input! {
    r: usize, g: usize, b: usize
  };

  let n = r * 100 + g * 10 + b;
  let ans = if n % 4 == 0 { "YES" } else { "NO" };
  println!("{}", ans);
}
