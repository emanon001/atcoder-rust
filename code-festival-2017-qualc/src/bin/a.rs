use proconio::input;

fn main() {
  input! {
    s: String
  };

  let res = if s.contains("AC") { "Yes" } else { "No" };
  println!("{}", res);
}
