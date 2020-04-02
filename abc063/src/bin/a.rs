use proconio::input;

fn main() {
  input! {
    a: usize, b: usize
  };
  let c = a + b;
  if c >= 10 {
    println!("error");
  } else {
    println!("{}", c);
  }
}
