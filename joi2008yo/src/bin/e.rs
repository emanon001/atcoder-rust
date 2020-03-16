use proconio::input;

fn main() {
  input! {
    r: usize, c: usize,
    grid: [[u32; c]; r]
  };

  let mut res = 0;
  for bits in 0..(1 << r) {
    let mut sum = 0;
    for col in 0..c {
      let mut back_count = 0;
      for row in 0..r {
        let reversed = (bits >> row) & 1 == 1;
        let v = grid[row][col];
        if (reversed && v == 1) || (!reversed && v == 0) {
          back_count += 1;
        }
      }
      let front_count = r - back_count;
      sum += std::cmp::max(back_count, front_count);
    }
    if sum > res {
      res = sum;
    }
  }
  println!("{}", res);
}
