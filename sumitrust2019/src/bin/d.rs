use proconio::input;

fn is_ok(s: &Vec<u32>, v: Vec<u32>) -> bool {
  let mut i = 0;
  let mut j = 0;
  while i < s.len() && j < v.len() {
    if s[i] == v[j] {
      j += 1;
    }
    i += 1;
  }
  j == v.len()
}

fn main() {
  input! {
    _: usize,
    s: String
  };

  let s = s.chars().map(|x| x.to_digit(10).unwrap()).collect();
  let mut res = 0;
  for a in 0..=9 {
    for b in 0..=9 {
      for c in 0..=9 {
        if is_ok(&s, vec![a, b, c]) {
          res += 1;
        }
      }
    }
  }
  println!("{}", res);
}
