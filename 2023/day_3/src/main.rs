/*
 * AoC: Day 3: Gear Ratios
 * URL: https://adventofcode.com/2023/day/3
 * */

use std::collections::HashSet;
use std::fs;

/// Represents a number in a specific context, including its value and position in a coordinate system.
///
/// Fields:
/// - `number`: The numeric value.
/// - `x_min`: Minimum index on the X axis.
/// - `x_max`: Maximum index on the X axis.
/// - `y`: Y-axis index.
struct Number {
  number: u32,
  x_min: usize,
  x_max: usize,
  y: usize,
}

impl Number {
  /// Creates a new instance of `Number`.
  ///
  /// Parameters:
  /// - `number`: Numeric value of the number.
  /// - `x_min`: Minimum index on the X axis.
  /// - `x_max`: Maximum index on the X axis.
  /// - `y`: Y-axis index.
  ///
  /// Returns: A new instance of `Number`.
  pub fn new(number: u32, x_min: usize, x_max: usize, y: usize) -> Self {
    Self {
      number,
      x_min,
      x_max,
      y,
    }
  }

  /// Checks if the given coordinates are within the region defined by this number.
  ///
  /// Parameters:
  /// - `x`: Index on the X-axis to check.
  /// - `y`: Index on the Y axis to check.
  ///
  /// Returns: `true` if the coordinates are inside the region, otherwise `false`.
  pub fn check(&self, x: usize, y: usize) -> bool {
    let x_min: usize = self.x_min.saturating_sub(1);
    let x_max: usize = self.x_max + 1;

    let y_min: usize = self.y.saturating_sub(1);
    let y_max: usize = self.y + 1;

    x >= x_min && x <= x_max && y >= y_min && y <= y_max
  }
}

/// Parses an input string and extracts numbers and symbols from it.
///
/// Parameters:
/// - `input`: Input text to parse.
/// - `only_stars`: If `true`, only '*' symbols will be extracted.
///
/// Returns: A tuple containing a vector of `Number` and a set of symbols (coordinates).
fn parse(input: &str, only_stars: bool) -> (Vec<Number>, HashSet<(usize, usize)>) {
  let mut numbers: Vec<Number> = Vec::new();
  let mut symbols: HashSet<(usize, usize)> = HashSet::new();

  for (i, line) in input.lines().enumerate() {
    let mut current_number: u32 = 0;
    let mut number_start_index: usize = 0;

    for (j, ch) in line.trim().chars().enumerate() {
      match ch.to_digit(10) {
        Some(value) => {
          number_start_index += 1;
          current_number = current_number * 10 + value;
        }
        None => {
          if current_number > 0 {
            numbers.push(Number::new(
              current_number,
              j - number_start_index,
              j - 1,
              i,
            ));

            current_number = 0;
            number_start_index = 0;
          }

          if (ch == '*' || !only_stars) && ch != '.' {
            symbols.insert((j, i));
          }
        }
      }
    }

    if current_number > 0 {
      numbers.push(Number::new(
        current_number,
        line.len() - number_start_index - 1,
        line.len() - 1,
        i,
      ));
    }
  }

  (numbers, symbols)
}

/// Calculates the sum of numbers matching certain conditions in the input.
///
/// Parameters:
/// - `input`: Input text to parse.
///
/// Returns: The sum of the numbers that meet the specified conditions.
fn part1(input: &str) -> u32 {
  let (numbers, symbols): (Vec<Number>, HashSet<(usize, usize)>) = parse(input, false);

  numbers
    .iter()
    .flat_map(|number| {
      symbols.iter().filter_map(move |(x, y)| {
        if number.check(*x, *y) {
          Some(number.number)
        } else {
          None
        }
      })
    })
    .sum()
}

/// Calculates an aggregate value based on a specific set of criteria applied to the
/// numbers and symbols in an input string.
///
/// This function takes an input string that is processed to extract numbers and symbols.
/// Then, for each symbol in the input, a specific calculation is performed based on the numbers that meet certain criteria
/// related to the position of the symbol.
/// The final result is the sum of the products of the numbers that meet the criteria for each symbol,
/// but only if exactly two numbers meet this criterion for a given symbol.
///
/// # Parameters
/// 
/// * `input` - A string (`&str`) containing the numbers and symbols to be processed.
///
/// # Returns
/// 
/// An `u32` which is the sum of the products of the numbers that meet the specified criterion
/// for each symbol in the input, provided that exactly two numbers meet this criterion for each symbol.
fn part2(input: &str) -> u32 {
  const REQUIRED_MATCHES: u32 = 2;

  let (numbers, symbols): (Vec<Number>, HashSet<(usize, usize)>) = parse(input, true);

  symbols.iter().fold(0, |acc, &(x, y)| {
    let (product, match_count) = numbers
      .iter()
      .filter(|number| number.check(x, y))
      .fold((1, 0), |(prod, count), number| {
        (prod * number.number, count + 1)
      });

    match (product, match_count) {
      (_, REQUIRED_MATCHES) => acc + product,
      (_, _) => acc,
    }
  })
}

fn main() {
  let input = fs::read_to_string("input.txt").unwrap();

  println!("{}", part1(&input));
  println!("{}", part2(&input));
}
