use proconio::input;

fn main() {
  input! {
    a: String
  };

  if a == "a" {
    println!("-1");
  } else {
    println!("a");
  }
}
