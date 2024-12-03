use std::fs;
use std::error::Error;

pub fn get_ex1_input() -> Result<(Vec<i32>, Vec<i32>), Box<dyn Error>> {
  let ex1_file = fs::read_to_string("src/inputs/ex_1.txt")?;

  Ok(ex1_file
    .lines()
    .filter_map(split_numbers)
    .unzip())

}

fn split_numbers(line: &str) -> Option<(i32, i32)> {
  let mut splitted_numbers = line.split_whitespace();

  if let (Some(n1_str), Some(n2_str)) = (splitted_numbers.next(), splitted_numbers.next()) {
    if let (Ok(n1), Ok(n2)) = (n1_str.parse::<i32>(), n2_str.parse::<i32>()) {
      return Some((n1, n2));
    }
  }

  None

}

pub fn historian_hysteria(list1: &Vec<i32>, list2: &Vec<i32>) -> i32 {
  
  let mut sorted_list1: Vec<i32> = list1.clone();
  sorted_list1.sort();

  let mut sorted_list2: Vec<i32> = list2.clone();
  sorted_list2.sort();

  let mut distance: i32 = 0; 

  for i in 0..sorted_list1.len() {
    distance += (sorted_list1[i] - sorted_list2[i]).abs();
  }

  distance

}