use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::{HashMap, HashSet};


fn main() {
  // let path = "words.txt";
  // let input = File::open(path).unwrap();
  // let buffered = BufReader::new(input);
  // for line in buffered.lines() {
  //   println!("{}", line.unwrap());
  // }
  // let lines: Vec<String> = buffered.lines().filter_map(Result::ok).collect();
  // write!(output, "Rust\n❤️\nFun")?;

  let words: Vec<String> = BufReader::new(File::open("words.txt").unwrap()).lines().filter_map(Result::ok).collect();
  let words_length: HashMap<_, usize> = words.iter()
    .map(|word| (word, word.len()))
    .collect();
  let wordslenght: HashMap<usize, _> = words.iter()
    .map(|word| (word.len(), word))
    .collect();

  let mut  wordsLenght: HashMap<usize, _> = HashMap::new();
  for word in words.iter() {
    wordsLenght
      .entry(word.len())
      .or_insert(HashSet::new())
      .insert(word);
  }
  println!("{:?}", words);
  println!("{:?}", words_length);
  println!("{:?}", wordslenght);
  println!("{:?}", wordsLenght);
}