use proconio::input;

fn main() {
  input! {
    h: usize, w: usize,
    grid: [[String; w]; h]
  };

  for i in 0..h {
    for j in 0..w {
      if grid[i][j] == "snuke" {
        let ch: char = (0x41_u8 + j as u8).into();
        println!("{}{}", ch, i + 1);
        std::process::exit(0);
      }
    }
  }
}
