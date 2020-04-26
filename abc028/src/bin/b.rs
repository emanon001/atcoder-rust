#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

fn main() {
  input! {
    s: Chars
  };

  let mut table = HashMap::new();
  for ch in s {
    let c = table.entry(ch).or_insert(0);
    *c += 1;
  }
  let res = vec!['A', 'B', 'C', 'D', 'E', 'F']
    .into_iter()
    .map(|ch| table.get(&ch).unwrap_or(&0))
    .join(" ");
  println!("{}", res);
}
