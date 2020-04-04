use proconio::input;

fn res(v: Vec<u64>) -> u64 {
  v.into_iter().fold(0 as u64, |acc, x| acc * 10 + x)
}

fn solve(k: usize) -> u64 {
  let mut count = 0;
  let mut que = std::collections::VecDeque::new();
  for v in 1..=9 {
    count += 1;
    if count == k {
      return v;
    }
    que.push_back(vec![v as u64]);
  }
  while let Some(nv) = que.pop_front() {
    let d = nv[nv.len() - 1];
    if d > 0 {
      count += 1;
      let new_v = [nv.clone(), vec![d - 1]].concat();
      if count == k {
        return res(new_v);
      }
      que.push_back(new_v);
    }
    count += 1;
    let new_v = [nv.clone(), vec![d]].concat();
    if count == k {
      return res(new_v);
    }
    que.push_back(new_v);
    if d < 9 {
      count += 1;
      let new_v = [nv, vec![d + 1]].concat();
      if count == k {
        return res(new_v);
      }
      que.push_back(new_v);
    }
  }
  0
}

fn main() {
  input! {
    k: usize
  };

  let res = solve(k);
  println!("{}", res);
}
