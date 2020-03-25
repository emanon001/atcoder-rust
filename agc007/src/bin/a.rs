use proconio::input;
use proconio::marker::Chars;

fn main() {
  input! {
    h: usize, w: usize,
    grid: [Chars; h]
  };

  let mut count = 0;
  for i in 0..h {
    for j in 0..w {
      if grid[i][j] == '#' {
        count += 1;
      }
    }
  }

  let res = if count == h + w - 1 {
    "Possible"
  } else {
    "Impossible"
  };
  println!("{}", res);
}
