use proconio::input;

fn main() {
  input! {
    x: usize, y: usize, a: usize, b: usize, c: usize,
    mut pv: [i64; a],
    mut qv: [i64; b],
    mut rv: [i64; c]
  };

  pv.sort_by_key(|&x| -x);
  qv.sort_by_key(|&x| -x);
  rv.sort_by_key(|&x| -x);
  let mut v = [&pv[0..x], &qv[0..y], &rv[..]].concat();
  v.sort_by_key(|&x| -x);
  let res: i64 = v[..(x + y)].iter().sum();
  println!("{}", res);
}
