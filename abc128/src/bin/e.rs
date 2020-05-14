use proconio::input;
use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug)]
enum Event {
  // time, x, s, t
  Add(isize, isize, isize, isize),
  // time, x, s, t
  Remove(isize, isize, isize, isize),
  // time
  Walk(isize),
}

fn main() {
  input! {
    n: usize, q: usize,
    mut stxv: [(isize, isize, isize); n],
    dv: [isize; q]
  };

  let mut events = stxv
    .into_iter()
    .flat_map(|(s, t, x)| vec![Event::Add(s - x, x, s, t), Event::Remove(t - x, x, s, t)])
    .chain(dv.into_iter().map(|d| Event::Walk(d)))
    .collect::<Vec<_>>();

  events.sort_by_key(|e| match e {
    Event::Add(t, _, _, _) => (*t, 0),
    Event::Remove(t, _, _, _) => (*t, 1),
    Event::Walk(t) => (*t, 2),
  });

  let mut res = BTreeMap::new();
  let mut set = BTreeSet::new();
  for e in events {
    match e {
      Event::Add(_, x, s, t) => {
        set.insert((x, s, t));
      }
      Event::Remove(_, x, s, t) => {
        set.remove(&(x, s, t));
      }
      Event::Walk(t) => {
        let mut iter = set.iter();
        if let Some(&x) = iter.next() {
          res.insert(t, x.0);
        } else {
          res.insert(t, -1);
        };
      }
    };
  }
  for (_, x) in res.into_iter() {
    println!("{}", x);
  }
}
