use proconio::input;

fn main() {
  input! {
    n: usize,
    alist: [usize; n]
  };

  let mut table = std::collections::HashMap::new();
  for &a in &alist {
    let x = table.entry(a).or_insert(0);
    *x += 1;
  }
  let mut list0 = vec![0; n + 1];
  let mut list1 = vec![0; n + 1];
  let mut sum = 0_i64;
  for (a, c) in table {
    list0[a] = c * (c - 1) / 2 as i64;
    list1[a] = (c - 1) * (c - 2) / 2 as i64;
    sum += list0[a] as i64;
  }
  for k in 0..n {
    let a = alist[k];
    let res = sum - list0[a] + list1[a];
    println!("{}", res);
  }
}
