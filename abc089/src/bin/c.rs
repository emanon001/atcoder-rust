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
    n: usize,
    sv: [Chars; n]
  };

  let mut table = HashMap::new();
  for s in sv {
    let p = s[0];
    *table.entry(p).or_insert(0) += 1;
  }
  let mut res = 0_u64;
  let prefix = "MARCH".chars();
  for v in prefix.combinations(3) {
    res += v
      .into_iter()
      .map(|ch| table.get(&ch).unwrap_or(&0))
      .fold(1, |acc, x| acc * x);
  }
  println!("{}", res);
}
