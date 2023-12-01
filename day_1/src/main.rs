/*
 * AoC: Day 1: Trebuchet?!
 * URL: https://adventofcode.com/2023/day/1
 * */
fn get_calibration_value_part_1(input: &str) -> u32 {
  let mut values: Vec<u32> = Vec::new();

  for line in input.lines() {
    let numbers: Vec<u32> = line.chars().filter_map(|c| c.to_digit(10)).collect();

    values.push(
      format!("{}{}", numbers.first().unwrap(), numbers.last().unwrap())
        .parse()
        .unwrap(),
    )
  }

  values.into_iter().sum()
}

fn get_calibration_value_part_2(input: &str) -> u32 {
  todo!()
}

fn main() {
  let input = std::fs::read_to_string("input.txt").unwrap();

  let part_1 = get_calibration_value_part_1(&input);

  println!("Part 1: {}", part_1);

  let part_2 = get_calibration_value_part_2(&input);

  println!("Part 2: {}", part_2);
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_get_calibration_value_part_1() {
    let input = "ninefourone1\n28gtbkszmrtmnineoneightmx\n";
    let value = get_calibration_value_part_1(input);

    assert_eq!(value, 39);
  }
}
