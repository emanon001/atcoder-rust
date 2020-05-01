#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;
use whiteread;

fn main() {
  let sv: Vec<String> = whiteread::parse_line().unwrap();
  input! {
    n: usize,
    tv: [String; n]
  };

  let mut res = Vec::new();
  for s in sv {
    let sc = s.chars().collect::<Vec<_>>();
    let filtered = tv.iter().any(|t| {
      if s.len() != t.len() {
        return false;
      }
      let tc = t.chars().collect::<Vec<_>>();
      for i in 0..s.len() {
        if tc[i] != '*' && sc[i] != tc[i] {
          return false;
        }
      }
      true
    });
    res.push(if filtered { "*".repeat(s.len()) } else { s });
  }
  println!("{}", res.join(" "));
}
