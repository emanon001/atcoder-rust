use proconio::input;
use proconio::marker::Chars;

fn main() {
  input! {
    s: Chars
  };

  let alpha = "abcdefghijklmnopqrstuvwxyz".chars();
  if s.len() < 26 {
    let set = s.iter().collect::<std::collections::HashSet<_>>();
    for a in alpha {
      if !set.contains(&a) {
        println!("{}{}", s.iter().collect::<String>(), a);
        std::process::exit(0);
      }
    }
  }

  let mut k = -1;
  let mut ch = 'z';
  for i in (0..26).rev() {
    for j in (0..i).rev() {
      if s[j] < s[i] {
        if j as isize > k {
          k = j as isize;
          ch = s[i];
        } else if j as isize == k && s[i] <= ch {
          ch = s[i];
        }
      }
    }
  }
  if k >= 0 {
    println!("{}{}", s[..(k as usize)].iter().collect::<String>(), ch);
  } else {
    println!("-1");
  }
}
