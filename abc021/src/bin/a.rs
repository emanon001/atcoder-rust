use proconio::input;

fn main() {
  input! {
    n: usize
  };

  let two_count = n / 2;
  if n % 2 == 0 {
    println!("{}", two_count);
  } else {
    println!("{}", two_count + 1);
    println!("1");
  }
  for _ in 0..two_count {
    println!("2");
  }
}
