use proconio::input;

fn main() {
  input! {
    n: usize,
    al: [f32; n]
  };

  let sum = al.iter().fold(0_f32, |acc, a| acc + a);
  let avg = sum / (n as f32);
  let mut absl = al
    .iter()
    .enumerate()
    .map(|(i, &a)| ((a - avg).abs(), i))
    .collect::<Vec<_>>();
  absl.sort_by(|a, b| a.partial_cmp(b).unwrap());
  println!("{}", absl[0].1);
}
