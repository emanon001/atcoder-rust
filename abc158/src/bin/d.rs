use proconio::input;
use proconio::marker::Chars;

fn main() {
  input! {
    s: Chars,
    q: i32
  };

  let mut deque = s.into_iter().collect::<std::collections::VecDeque<char>>();
  let mut reversed = false;
  for _ in 0..q {
    input! { t: i32 }
    if t == 1 {
      reversed = !reversed;
    } else {
      input! {
        f: i32,
        c: char
      };
      if f == 1 {
        if !reversed {
          deque.push_front(c);
        } else {
          deque.push_back(c);
        }
      } else {
        if !reversed {
          deque.push_back(c);
        } else {
          deque.push_front(c);
        }
      }
    };
  }
  let ans = if reversed {
    deque.into_iter().rev().collect::<String>()
  } else {
    deque.into_iter().collect::<String>()
  };
  println!("{}", ans);
}
