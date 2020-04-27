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
    wv: [Chars; n]
  };

  let mut table = HashMap::new();
  let pairs = vec![
    ('z', 'r'),
    ('b', 'c'),
    ('d', 'w'),
    ('t', 'j'),
    ('f', 'q'),
    ('l', 'v'),
    ('s', 'x'),
    ('p', 'm'),
    ('h', 'k'),
    ('n', 'g'),
  ];
  for (i, pair) in pairs.into_iter().enumerate() {
    table.insert(pair.0, i);
    table.insert(pair.1, i);
  }

  let res = wv
    .into_iter()
    .flat_map(|chars| {
      let res = chars
        .into_iter()
        .flat_map(|ch| table.get(&ch.to_ascii_lowercase()).map(|ch| ch.to_string()))
        .join("");
      if res.len() == 0 {
        None
      } else {
        Some(res)
      }
    })
    .join(" ");
  println!("{}", res);
}
