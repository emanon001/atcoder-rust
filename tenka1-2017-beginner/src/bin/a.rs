use proconio::input;

fn main() {
  input! {
    s: String
  };

  let res = s.chars().filter(|ch| *ch == '1').count();
  println!("{}", res);
}
