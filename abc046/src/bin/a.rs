use proconio::input;

fn main() {
  input! {
    a: usize, b: usize, c: usize
  };

  let res = [a, b, c]
    .iter()
    .collect::<std::collections::HashSet<_>>()
    .len();
  println!("{}", res);
}
