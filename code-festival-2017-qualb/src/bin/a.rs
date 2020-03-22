use proconio::input;

fn main() {
  input! {
    s: String
  };

  let res = &s[0..s.len() - 8];
  println!("{}", res);
}
