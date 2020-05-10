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

  let mut lc1 = 0_isize;
  let mut rc1 = 0_isize;
  let mut lc2 = 0_isize;
  let mut rc2 = 0_isize;
  for s in sv {
    let mut lc = 0;
    let mut rc = 0;
    for ch in s {
      match ch {
        '(' => {
          lc += 1;
        }
        ')' => {
          if lc > 0 {
            lc -= 1;
          } else {
            rc += 1;
          }
        }
        _ => unreachable!(),
      }
    }
    if lc > 0 && rc > 0 {
      lc1 += lc;
      rc1 += rc;
    } else {
      lc2 += rc;
      rc2 += lc;
    }
  }
  let res = if lc2 == rc2 {
    if lc1 == 0 && rc1 == 0 {
      "Yes"
    } else {
      "No"
    }
  } else if lc2 > rc2 {
    let lc = lc2 - rc2;
    if lc == rc1 && lc1 == 0 {
      "Yes"
    } else {
      "No"
    }
  } else {
    // lc2 < rc2
    let rc = rc2 - lc2;
    if rc == lc1 && rc1 == 0 {
      "Yes"
    } else {
      "No"
    }
  };
  println!("{}", res);
}
