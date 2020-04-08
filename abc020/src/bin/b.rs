use proconio::input;

fn main() {
  input! {
    a: usize, b: usize
  };

  let ab = a.to_string() + &b.to_string();
  let res = i32::from_str_radix(&ab, 10).unwrap() * 2;
  println!("{}", res);
}
