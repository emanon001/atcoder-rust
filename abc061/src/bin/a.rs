use proconio::input;

fn main() {
  input! {
    a: isize, b: isize, c: isize
  };

  let res = if c >= a && c <= b { "Yes" } else { "No" };
  println!("{}", res);
}
