use proconio::input;

fn is_acgt(s: &str) -> bool {
  let acgt = vec!['A', 'C', 'G', 'T'];
  s.chars().all(|x| acgt.contains(&x))
}

fn main() {
  input! {
    s: String
  };

  let mut res = 0;
  for i in 0..s.len() {
    for j in i..s.len() {
      let len = j - i + 1;
      if is_acgt(&s[i..=j]) && len > res {
        res += 1;
      }
    }
  }
  println!("{}", res);
}
