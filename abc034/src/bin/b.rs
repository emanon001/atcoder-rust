use proconio::input;

fn main() {
  input! {
    n: usize
  };

  let res = if n % 2 == 0 { n - 1 } else { n + 1 };
  println!("{}", res);
}
