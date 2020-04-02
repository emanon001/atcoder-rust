use proconio::input;

fn main() {
  input! {
    a: usize, b: usize
  };

  let hour = (a + b) % 24;
  println!("{}", hour);
}
