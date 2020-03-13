use proconio::input;

fn main() {
  input! {
    n: usize,
    mut a: [usize; n]
  };

  a.sort();
  let ans = a[n - 1] - a[0];
  println!("{}", ans);
}
