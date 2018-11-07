fn main() {
  let mut numbers = vec![];
  for i in 1..1000 {
    numbers.push(i);
  }
  println!("{:?}", numbers);

  let mut numbers1: Vec<i64> = (1..1000).collect();
  println!("{:?}", numbers1);
}