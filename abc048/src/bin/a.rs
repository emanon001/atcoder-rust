use proconio::input;

fn main() {
  input! {
    _: String,
    s: String,
  };
  let res = s.chars().nth(0).unwrap().to_uppercase();
  println!("A{}C", res);
}
