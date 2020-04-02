use proconio::input;

fn main() {
  input! {
    s: String
  };
  let res = s.replace(",", " ");
  println!("{}", res);
}
