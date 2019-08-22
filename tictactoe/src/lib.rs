#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_matrix_should_return_false () {
      let matrix: [String; 9] = Default::default();
      assert_eq!(check_results(&matrix), false);
    }

    #[test]
    fn row_matrix_should_return_true () {
      let mut matrix: [String; 9] = Default::default();
      matrix[0 as usize] = "x".to_string();
      matrix[1 as usize] = "x".to_string();
      matrix[2 as usize] = "x".to_string();
      assert_eq!(check_results(&matrix), true);
    }

    #[test]
    fn column_matrix_should_return_true () {
      let mut matrix: [String; 9] = Default::default();
      matrix[0 as usize] = "x".to_string();
      matrix[3 as usize] = "x".to_string();
      matrix[6 as usize] = "x".to_string();
      assert_eq!(check_results(&matrix), true);
    }

    #[test]
    fn draw_matrix_should_return_true () {
      let matrix: [String; 9] = [
        "x".to_string(), "o".to_string(), "x".to_string(),
        "o".to_string(), "x".to_string(), "o".to_string(), 
        "x".to_string(), "x".to_string(), "o".to_string()
      ];
      assert_eq!(check_results(&matrix), true);
    }
}

const WINNING_SEQUENCES: [[i8; 3]; 8] = [[0, 1, 2], [3, 4, 5], [6, 7, 8], [0, 3, 6], [1, 4, 7], [2, 5, 8], [0, 4, 8], [2, 4, 6]];

pub fn check_results (matrix: &[String; 9]) -> bool {
  let mut game_over = false;
  for sequence in WINNING_SEQUENCES.iter() {
    if (
        !matrix[sequence[0] as usize].is_empty() &&
        !matrix[sequence[1] as usize].is_empty() &&
        !matrix[sequence[2] as usize].is_empty()
      ) &&
      (
        matrix[sequence[0] as usize] == matrix[sequence[1] as usize] &&
        matrix[sequence[1] as usize] == matrix[sequence[2] as usize]) {
      println!("Player {} wins!", matrix[sequence[0] as usize]);
      game_over = true;
    } else {
      let default = matrix.iter().position(|r| r.is_empty());
      if !default.is_some() {
        println!("Its a draw!");
        game_over = true;
      } else {
        game_over = game_over || false;
      }
    }
  }
  return game_over;
}