use proconio::input;

fn main() {
  input! {
    x: usize, y: usize, a: usize, b: usize, c: usize,
    p_l: [isize; a],
    q_l: [isize; b],
    r_l: [isize; c]
  };

  let mut pq_que = std::collections::BinaryHeap::new();
  let mut p_l = p_l;
  p_l.sort_by_key(|&x| -x);
  for &p in &p_l[..x] {
    pq_que.push(std::cmp::Reverse(p));
  }
  let mut q_l = q_l;
  q_l.sort_by_key(|&x| -x);
  for &q in &q_l[..y] {
    pq_que.push(std::cmp::Reverse(q));
  }

  let mut r_l = r_l;
  r_l.sort_by_key(|&x| -x);
  for r in r_l {
    if let Some(&std::cmp::Reverse(x)) = pq_que.peek() {
      if r > x {
        pq_que.pop();
        pq_que.push(std::cmp::Reverse(r));
      }
    }
  }
  let res = pq_que
    .into_iter()
    .fold(0_i64, |acc, std::cmp::Reverse(x)| acc + (x as i64));
  println!("{}", res);
}
