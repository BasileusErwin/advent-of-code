/*
 * AoC: Day 1: Trebuchet?!
 * URL: https://adventofcode.com/2023/day/1
 * */

/// Calculate the total calibration value according to part 1.
///
/// This function processes a multi-line string, where each line represents a data set.
/// For each line, it extracts all the numeric digits, forms a new number using
/// the first and last digit of these, and accumulates these numbers into a vector.
/// Finally, it adds up all the values in the vector to get the total calibration value.
///
/// # Arguments
///
/// * `input` - A reference to a string containing the input data set,
/// where each line is separated by line breaks.
///
/// # Returns
///
/// Returns the total calibration value as a `u32`.
///
/// # Examples
///
/// ```
/// let input = "12ab\n34cd\n56ef";
/// let total = get_calibration_value_part_1(input);
/// assert_eq!(total, 127); /// 12 + 34 + 56 = 127
/// ```
///
/// # Panics
///
/// The function may panic if any line contains no digits,
/// since `unwrap` is called on an `Option` which could be `None`.
fn get_calibration_value_part_1(input: &str) -> u32 {
  let mut values: Vec<u32> = Vec::new();

  for line in input.lines() {
    let numbers: Vec<u32> = line.chars().filter_map(|c| c.to_digit(10)).collect();

    if numbers.is_empty() {
      continue;
    }

    values.push(
      format!("{}{}", numbers.first().unwrap(), numbers.last().unwrap())
        .parse()
        .unwrap(),
    )
  }

  values.into_iter().sum()
}

/// Calculate the total calibration value according to part 2.
fn get_calibration_value_part_2(_input: &str) -> u32 {
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
  fn test_simple_numbers() {
    assert_eq!(get_calibration_value_part_1("ninefourone1\n"), 11);
  }

  #[test]
  fn test_mixed_numbers_and_words() {
    assert_eq!(get_calibration_value_part_1("53sevenvvqm\n"), 53);
  }

  #[test]
  fn test_long_string_with_multiple_numbers() {
    assert_eq!(
      get_calibration_value_part_1("kscpjfdxp895foureightckjjl1\n"),
      81
    );
  }

  #[test]
  fn test_single_digit_numbers() {
    assert_eq!(get_calibration_value_part_1("72fivebt9ndgq\n"), 79);
  }

  #[test]
  fn test_numbers_with_letters_in_between() {
    assert_eq!(
      get_calibration_value_part_1("28gtbkszmrtmnineoneightmx\n"),
      28
    );
  }

  #[test]
  fn test_only_letters() {
    assert_eq!(
      get_calibration_value_part_1("four66jqrbtqcsxjtqjvfjhl1\n"),
      61
    );
  }

  #[test]
  fn test_numbers_at_the_end() {
    assert_eq!(
      get_calibration_value_part_1("rgxjrsldrfmzq25szhbldzqhrhbjpkbjlsevenseven\n"),
      25
    );
  }

  #[test]
  fn test_multiple_lines() {
    let input = "slkjvk4threesevenznjqmmfive\n61ppgrkmkfhteightone1\n";
    assert_eq!(get_calibration_value_part_1(input), 105);
  }

  #[test]
  fn test_empty_string() {
    assert_eq!(get_calibration_value_part_1(""), 0);
  }

  #[test]
  fn test_no_numbers() {
    assert_eq!(
      get_calibration_value_part_1("no numbers here\nanother line\n"),
      0
    );
  }
}
