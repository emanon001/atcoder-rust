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
    s: Chars,
    t: Chars,
  };

  let mut s_to_t = HashMap::new();
  let mut t_to_s = HashMap::new();
  for i in 0..s.len() {
    let sc = s[i];
    let tc = t[i];
    match (s_to_t.get(&sc), t_to_s.get(&tc)) {
      (Some(ch1), Some(ch2)) => {
        if *ch1 != tc || *ch2 != sc {
          println!("No");
          std::process::exit(0);
        }
      }
      (None, None) => {
        s_to_t.insert(sc, tc);
        t_to_s.insert(tc, sc);
      }
      _ => {
        println!("No");
        std::process::exit(0);
      }
    };
  }
  println!("Yes");
}
