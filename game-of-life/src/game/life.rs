use crate::game::god::is_alive;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn regenerate_0_cells() {
        let cells = Vec::new();
        assert_eq!(regenerate(&cells), []);
    }

    #[test]
    fn regenerate_1_cell() {
        let mut cells = vec![false; 9];
        cells[4] = true;
        assert_eq!(regenerate(&cells), [false, false, false, false, false, false, false, false, false]);
    }
    #[test]
    fn regenerate_3_cells() {
        let mut cells = vec![false; 400];
        cells[0] = true;
        cells[1] = true;
        cells[2] = true;
        let regenerated = regenerate(&cells);
        assert_eq!(regenerated[0], false);
        assert_eq!(regenerated[1], true);
        assert_eq!(regenerated[2], false);
        assert_eq!(regenerated[20], false);
        assert_eq!(regenerated[21], true);
        assert_eq!(regenerated[23], false);
    }
}

pub fn regenerate(cells: &Vec<bool>) -> Vec<bool> {
  let width = (cells.len() as f64).sqrt().abs() as i32;
  let length = cells.len() as i32;

  let mut generation: Vec<bool>  = Vec::new();
  for (i, &cell) in cells.iter().enumerate() {
    let index = i as i32;
    let mut neighbours: Vec<bool> = vec![];

    let c = (index - width) - 1;
    if c >= 0 {
      neighbours.push(cells[c as usize]);
    }

    let c = index - width;
    if c >= 0 {
      neighbours.push(cells[c as usize]);
    }

    let c = (index - width) + 1;
    if c >= 0 {
      neighbours.push(cells[c as usize]);
    }

    let c = index - 1;
    if c >= 0 {
      neighbours.push(cells[c as usize]);
    }

    let c = index + 1;
    if c >= 0 && c < length {
      neighbours.push(cells[c as usize]);
    }

    let c = index + width - 1;
    if c >= 0 && c < length {
      neighbours.push(cells[c as usize]);
    }

    let c = index + width;
    if c >= 0 && c < length {
      neighbours.push(cells[c as usize]);
    }

    let c = index + width + 1;
    if c >= 0 && c < length {
      neighbours.push(cells[c as usize]);
    }

    let total = neighbours.iter().filter(|&&n| n == true).count() as i32;
    generation.push(is_alive(cell, total));
  }
  return generation;
}
