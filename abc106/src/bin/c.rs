use proconio::input;
use proconio::marker::Chars;

fn main() {
  input! {
    s: Chars,
    k: u64
  };

  let prefix_one_count = s.iter().take_while(|&ch| *ch == '1').count();
  if prefix_one_count as u64 >= k {
    println!("1");
  } else {
    println!("{}", s[prefix_one_count]);
  }
}
