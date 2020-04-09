use proconio::input;

fn main() {
  input! {
    mut v: [usize; 3]
  };

  v.sort();
  println!("{}", v[1]);
}
