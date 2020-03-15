use proconio::input;

fn main() {
  input! {
    n: usize,
    ab_list: [(i64, i64); n]
  };

  let mut a_list: Vec<i64> = ab_list.iter().copied().map(|(a, _)| a).collect();
  let mut b_list: Vec<i64> = ab_list.iter().copied().map(|(_, b)| b).collect();
  a_list.sort();
  b_list.sort();
  let start = a_list[n / 2];
  let goal = b_list[n / 2];
  let mut res = 0_i64;
  for (a, b) in ab_list {
    res += (start - a).abs() + (a - b).abs() + (b - goal).abs();
  }
  println!("{}", res);
}
