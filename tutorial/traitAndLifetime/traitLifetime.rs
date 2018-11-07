fn main() {
  let list = vec![23, 54, 56, 34];
  let result = largest(&list);

  println!("Largest num is {}", result);

  let list_chars = vec!['a', 'b', 'c'];
  println!("largest char is {}", largest(&list_chars));
}

// fn largest(list: &[i32]) -> i32 {
//   let mut largest = list[0];
//   for &n in list {
//     if n > largest {
//       largest = n;
//     }
//   }
//   largest
// }

// fn largetChar(list: &[char]) -> char {
//   let mut largest = list[0];
//   for &n in list {
//     if n > largest {
//       largest = n;
//     }
//   }
//   largest
// }

fn largest<T: PartialOrd+Copy>(list: &[T]) -> T {
  let mut largest = list[0];
  for &n in list {
    if n > largest {
      largest = n;
    }
  }
  largest
}