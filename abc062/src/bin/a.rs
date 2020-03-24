use proconio::input;

fn main() {
  input! {
    x: usize, y: usize
  };

  let mut t = std::collections::HashMap::new();
  for x in vec![1, 3, 5, 7, 8, 10, 12] {
    t.insert(x, 1);
  }
  for x in vec![4, 6, 9, 11] {
    t.insert(x, 2);
  }
  t.insert(2, 3);
  let res = if t.get(&x) == t.get(&y) { "Yes" } else { "No" };
  println!("{}", res);
}
