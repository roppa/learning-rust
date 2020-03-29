
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn alive_with_0_neighbours() {
        assert_eq!(is_alive(true, 0), false);
    }

    #[test]
    fn alive_with_1_neighbour() {
        assert_eq!(is_alive(true, 1), false);
    }

    #[test]
    fn alive_with_2_neighbours() {
        assert_eq!(is_alive(true, 2), true);
    }

    #[test]
    fn alive_with_3_neighbours() {
        assert_eq!(is_alive(true, 3), true);
    }

    #[test]
    fn alive_with_4_neighbours() {
        assert_eq!(is_alive(true, 4), false);
    }

    #[test]
    fn dead_with_0_neighbours() {
        assert_eq!(is_alive(false, 0), false);
    }

    #[test]
    fn dead_with_3_neighbours() {
        assert_eq!(is_alive(false, 3), true);
    }
}

pub fn is_alive(alive: bool, neighbours: i32) -> bool {
  return alive && neighbours == 2 || neighbours == 3
}
