use proconio::input;

fn main() {
  input! {
    s: String
  };

  let ans = if s.contains("A") && s.contains("B") {
    "Yes"
  } else {
    "No"
  };
  println!("{}", ans);
}
