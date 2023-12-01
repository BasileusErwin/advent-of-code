/*
 * AoC: Day 1: Trebuchet?!
 * URL: https://adventofcode.com/2023/day/1
 * */

/*
* Calcula el valor total de calibración según las reglas especificadas.
*
* Esta función procesa un string multilínea, donde cada línea representa un conjunto de datos.
* Para cada línea, extrae todos los dígitos numéricos, forma un nuevo número utilizando
* el primer y último dígito de estos, y acumula estos números en un vector.
* Finalmente, suma todos los valores en el vector para obtener el valor total de calibración.
*
* # Arguments
*
* `input` - Una referencia a un string que contiene el conjunto de datos de entrada,
*           donde cada línea es separada por saltos de línea.
* # Returns
* Retorna el valor total de calibración como un `u32`.
*
* # Examples
*
* ```
* let input = "12ab\n34cd\n56ef";
* let total = get_calibration_value_part_1(input);
* assert_eq!(total, 127); // 12 + 34 + 56 = 127
* ```
* # Panics
* La función puede entrar en pánico si alguna línea no contiene dígitos,
* ya que `unwrap` es llamado en un `Option` que podría ser `None`.
*/
/// Calculates the total calibration value according to the specified rules.
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
