use proconio::input;

fn main() {
  input! {
    a: char, b: char
  };
  let res = if (a == 'H' && b == 'H') || (a == 'D' && b == 'D') {
    "H"
  } else {
    "D"
  };
  println!("{}", res);
}
