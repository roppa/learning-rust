#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_fizzbuzz_string () {
      let fb = fizz_buzz();
      let mut lines = fb.lines();
      assert_eq!(Some("1"), lines.next());
      assert_eq!(Some("2"), lines.next());
      assert_eq!(Some("Fizz"), lines.next());
      assert_eq!(Some("4"), lines.next());
      assert_eq!(Some("Buzz"), lines.next());
      assert_eq!(Some("Fizz"), lines.next());
      assert_eq!(Some("7"), lines.next());
      assert_eq!(Some("8"), lines.next());
      assert_eq!(Some("Fizz"), lines.next());
      assert_eq!(Some("Buzz"), lines.next());
      assert_eq!(Some("11"), lines.next());
      assert_eq!(Some("Fizz"), lines.next());
      assert_eq!(Some("13"), lines.next());
      assert_eq!(Some("14"), lines.next());
      assert_eq!(Some("FizzBuzz"), lines.next());
      assert_eq!(Some("16"), lines.next());
    }
}

pub fn fizz_buzz() -> String {
  let mut result: String = "".to_string();
    for x in 1..101 {
        if x % 3 == 0 && x % 5 == 0 {
          result = format!("{}{}\n", result, "FizzBuzz");
        } else if x % 5 == 0 {
          result = format!("{}{}\n", result, "Buzz");
        } else if x % 3 == 0 {
          result = format!("{}{}\n", result, "Fizz");
        } else {
          result = format!("{}{}\n", result, x.to_string());
        }
    }
  return String::from(result);
}