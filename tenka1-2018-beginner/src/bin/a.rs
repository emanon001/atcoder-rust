use proconio::input;

fn main() {
  input! {
    s: String
  };

  if s.len() == 2 {
    println!("{}", s);
  } else {
    println!("{}", s.chars().rev().collect::<String>());
  }
}
