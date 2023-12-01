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

/// Represents a scanner to process text strings and calculate a calibration value.
///
/// `Scanner` processes an input string, identifying numbers represented as digits or words,
/// and calculates a total value based on specific rules.
///
/// # Fields
///
/// * `source` - Input string to be processed.
/// * `current` - Index of the current character in the input string.
/// * `start` - Start index for the current parse operation.
/// * `total` - Total calculated calibration value.
struct Scanner {
  source: String,
  current: usize,
  start: usize,
  total: u32,
}

impl Scanner {
  /// Creates a new instance of `Scanner` with the given input string.
  ///
  /// # Arguments
  ///
  /// * `source` - Input string that the `Scanner` will process.
  pub fn new(source: String) -> Self {
    Self {
      source,
      current: 0,
      start: 0,
      total: 0,
    }
  }
  /// Advance to the next character in the input string.
  ///
  /// Returns the current character before advancing.
  /// If the end of the string is reached, returns '\0'.
  fn advance(&mut self) -> char {
    self.current += 1;
    self.source.chars().nth(self.current - 1).unwrap_or('\0')
  }

  /// Checks if the substring from the current index starts with `rest`.
  ///
  /// # Arguments
  ///
  /// * `rest` - The substring to check.
  ///
  /// # Returns
  ///
  /// True if the substring begins with `rest`; otherwise, false.
  fn check_word(&self, rest: &str) -> bool {
    self
      .source
      .get(self.current..self.current + rest.len())
      .map_or(false, |substr| substr.starts_with(rest))
  }

  /// Converts a word identified by its first letter to its corresponding numeric value.
  ///
  /// # Arguments
  ///
  /// * `c` - Initial character of the word to convert.
  ///
  /// # Returns
  ///
  /// The numeric value corresponding to the word; if not recognized, returns 0.
  fn convert_word_to_number(&self, c: char) -> u32 {
    match c {
      'o' => {
        if self.check_word("ne") {
          return 1;
        }

        0
      }
      't' => {
        if self.check_word("wo") {
          return 2;
        }

        if self.check_word("hree") {
          return 3;
        }

        0
      }
      'f' => {
        if self.check_word("our") {
          return 4;
        }

        if self.check_word("ive") {
          return 5;
        }

        0
      }
      's' => {
        if self.check_word("ix") {
          return 6;
        }
        if self.check_word("even") {
          return 7;
        }

        0
      }
      'e' => {
        if self.check_word("ight") {
          return 8;
        }

        0
      }
      'n' => {
        if self.check_word("ine") {
          return 9;
        }

        0
      }
      _ => 0,
    }
  }

  /// Processes the input string and calculates the total calibration value.
  ///
  /// Reads the input string, identifying and converting numbers and accumulating a calibration total.
  ///
  /// # Returns
  ///
  /// The total calibration value as a `u32`.
  pub fn scanner(&mut self) -> u32 {
    let mut values: Vec<u32> = Vec::new();

    loop {
      self.start = self.current;

      let c = self.advance();

      if c == '\n' {
        if values.is_empty() {
          continue;
        }

        self.total += format!("{}{}", values.first().unwrap(), values.last().unwrap())
          .parse::<u32>()
          .unwrap();

        values.clear();
      }

      if c == '\0' {
        break;
      }

      if c.is_numeric() {
        values.push(format!("{}", c).parse().unwrap());
      }

      if c.is_alphabetic() {
        let number = self.convert_word_to_number(c);

        if number == 0 {
          continue;
        }

        values.push(number);
      }
    }

    self.total
  }
}

/// Calculates the total calibration value according to Part 2 of the challenge.
///
/// Uses the `Scanner` structure to process the input and calculate the total.
///
/// # Arguments
///
/// * `input` - The input string containing the data for the calibration calculation.
///
/// # Returns
///
/// The total calibration value as a `u32`.
fn get_calibration_value_part_2(input: &str) -> u32 {
  let mut scanner = Scanner::new(input.to_string());

  scanner.scanner()
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

  // Part 2

  #[test]
  fn test_simple_numbers_part_2() {
    assert_eq!(get_calibration_value_part_2("ninefourone1\n"), 91);
  }

  #[test]
  fn test_mixed_numbers_and_words_part_2() {
    assert_eq!(get_calibration_value_part_2("53sevenvvqm\n"), 57);
  }

  #[test]
  fn test_long_string_with_multiple_numbers_part_2() {
    assert_eq!(
      get_calibration_value_part_2("kscpjfdxp895foureightckjjl1\n"),
      81
    );
  }

  #[test]
  fn test_single_digit_numbers_part_2() {
    assert_eq!(get_calibration_value_part_2("72fivebt9ndgq\n"), 79);
  }

  #[test]
  fn test_numbers_with_letters_in_between_part_2() {
    assert_eq!(
      get_calibration_value_part_2("28gtbkszmrtmnineoneightmx\n"),
      28
    );
  }

  #[test]
  fn test_only_letters_part_2() {
    assert_eq!(
      get_calibration_value_part_2("four66jqrbtqcsxjtqjvfjhl1\n"),
      41
    );
  }

  #[test]
  fn test_numbers_at_the_end_part_2() {
    assert_eq!(
      get_calibration_value_part_2("rgxjrsldrfmzq25szhbldzqhrhbjpkbjlsevenseven\n"),
      27
    );
  }

  #[test]
  fn test_multiple_lines_part_2() {
    let input = "slkjvk4threesevenznjqmmfive\n61ppgrkmkfhteightone1\n";
    assert_eq!(get_calibration_value_part_2(input), 106);
  }

  #[test]
  fn test_empty_string_part_2() {
    assert_eq!(get_calibration_value_part_2(""), 0);
  }

  #[test]
  fn test_no_numbers_part_2() {
    assert_eq!(
      get_calibration_value_part_2("no numbers here\nanother line\n"),
      0
    );
  }
}
