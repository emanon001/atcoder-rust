use proconio::input;

fn main() {
  input! {
    a: usize, b: usize, c: usize
  };
  let count5 = vec![a, b, c].into_iter().filter(|&x| x == 5).count();
  let count7 = vec![a, b, c].into_iter().filter(|&x| x == 7).count();
  let res = if count5 == 2 && count7 == 1 {
    "YES"
  } else {
    "NO"
  };
  println!("{}", res);
}
