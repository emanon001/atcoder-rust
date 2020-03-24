use proconio::input;

fn main() {
  input! {
    s: String
  };

  let i = s.find('C').unwrap_or(s.len());
  let j = s.rfind('F').unwrap_or(0);
  if i < j {
    println!("Yes");
  } else {
    println!("No");
  }
}
