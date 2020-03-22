use proconio::input;

fn main() {
  input! {
    s: String
  };

  let res = if s.starts_with("YAKI") { "Yes" } else { "No" };
  println!("{}", res);
}
