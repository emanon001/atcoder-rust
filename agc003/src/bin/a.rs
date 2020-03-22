use proconio::input;
use proconio::marker::Chars;

fn main() {
  input! {
    s: Chars
  };

  let n_c = s.iter().filter(|&ch| ch == &'N').count();
  let w_c = s.iter().filter(|&ch| ch == &'W').count();
  let s_c = s.iter().filter(|&ch| ch == &'S').count();
  let e_c = s.iter().filter(|&ch| ch == &'E').count();
  let res = if ((n_c > 0 && s_c == 0) || (n_c == 0 && s_c > 0))
    || ((w_c > 0 && e_c == 0) || (w_c == 0 && e_c > 0))
  {
    "No"
  } else {
    "Yes"
  };
  println!("{}", res);
}
