use proconio::input;

fn main() {
  input! {
    n: usize,
    s: String
  };

  if s == "b" {
    println!("0");
    std::process::exit(0);
  }

  let mut v = std::collections::VecDeque::from(vec!['b']);
  for i in 0..n {
    match i % 3 {
      0 => {
        v.push_front('a');
        v.push_back('c');
      }
      1 => {
        v.push_front('c');
        v.push_back('a');
      }
      2 => {
        v.push_front('b');
        v.push_back('b');
      }
      _ => {}
    }
    if v.iter().collect::<String>() == s {
      println!("{}", i + 1);
      std::process::exit(0);
    }
  }
  println!("-1");
}
