use proconio::input;

fn main() {
  input! {
    n: usize,
    al: [u64; 3 * n]
  };

  let mut al = al;
  al.sort_by_key(|&a| -(a as i64));
  let res = al.into_iter().skip(1).step_by(2).take(n).sum::<u64>();
  println!("{}", res);
}
