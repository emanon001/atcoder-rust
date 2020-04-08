use proconio::input;

fn main() {
  input! {
    q: usize,
  };

  let res = if q == 1 { "ABC" } else { "chokudai" };
  println!("{}", res);
}
