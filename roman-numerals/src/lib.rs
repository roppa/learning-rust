#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_empty_for_0 () {
      assert_eq!(numeralise(0), "");
    }

    #[test]
    fn returns_i_for_1 () {
      assert_eq!(numeralise(1), "I");
    }

    #[test]
    fn returns_ii_for_2 () {
      assert_eq!(numeralise(2), "II");
    }

    #[test]
    fn returns_iii_for_3 () {
      assert_eq!(numeralise(3), "III");
    }

    #[test]
    fn returns_iv_for_4 () {
      assert_eq!(numeralise(4), "IV");
    }

    #[test]
    fn returns_v_for_5 () {
      assert_eq!(numeralise(5), "V");
    }

    #[test]
    fn returns_vi_for_6 () {
      assert_eq!(numeralise(6), "VI");
    }

    #[test]
    fn returns_vii_for_7 () {
      assert_eq!(numeralise(7), "VII");
    }

    #[test]
    fn returns_viii_for_8 () {
      assert_eq!(numeralise(8), "VIII");
    }

    #[test]
    fn returns_ix_for_9 () {
      assert_eq!(numeralise(9), "IX");
    }

    #[test]
    fn returns_x_for_10 () {
      assert_eq!(numeralise(10), "X");
    }

    #[test]
    fn returns_xi_for_11 () {
      assert_eq!(numeralise(11), "XI");
    }

    #[test]
    fn returns_xii_for_12 () {
      assert_eq!(numeralise(12), "XII");
    }

    #[test]
    fn returns_xiii_for_13 () {
      assert_eq!(numeralise(13), "XIII");
    }

    #[test]
    fn returns_xiv_for_14 () {
      assert_eq!(numeralise(14), "XIV");
    }

    #[test]
    fn returns_xv_for_15 () {
      assert_eq!(numeralise(15), "XV");
    }

    #[test]
    fn returns_xvi_for_16 () {
      assert_eq!(numeralise(16), "XVI");
    }

    #[test]
    fn returns_xvii_for_17 () {
      assert_eq!(numeralise(17), "XVII");
    }

    #[test]
    fn returns_xviii_for_18 () {
      assert_eq!(numeralise(18), "XVIII");
    }

    #[test]
    fn returns_xix_for_19 () {
      assert_eq!(numeralise(19), "XIX");
    }

    #[test]
    fn returns_xx_for_20 () {
      assert_eq!(numeralise(20), "XX");
    }

    #[test]
    fn returns_xxi_for_21 () {
      assert_eq!(numeralise(21), "XXI");
    }

    #[test]
    fn returns_xxxi_for_31 () {
      assert_eq!(numeralise(31), "XXXI");
    }

    #[test]
    fn returns_xxxix_for_39 () {
      assert_eq!(numeralise(39), "XXXIX");
    }

    #[test]
    fn returns_xxxxi_for_42 () {
      assert_eq!(numeralise(42), "XXXXII");
    }

    #[test]
    fn returns_l_for_50 () {
      assert_eq!(numeralise(50), "L");
    }

    #[test]
    fn returns_lvii_for_57 () {
      assert_eq!(numeralise(57), "LVII");
    }

    #[test]
    fn returns_xcix_for_99 () {
      assert_eq!(numeralise(99), "XCIX");
    }

    #[test]
    fn returns_c_for_100 () {
      assert_eq!(numeralise(100), "C");
    }

    #[test]
    fn returns_ccciv_for_304 () {
      assert_eq!(numeralise(304), "CCCIV");
    }

    #[test]
    fn returns_cd_for_400 () {
      assert_eq!(numeralise(400), "CD");
    }

    #[test]
    fn returns_cd_for_401 () {
      assert_eq!(numeralise(401), "CDI");
    }

    #[test]
    fn returns_d_for_500 () {
      assert_eq!(numeralise(500), "D");
    }

    #[test]
    fn returns_dccc_for_800 () {
      assert_eq!(numeralise(800), "DCCC");
    }

    #[test]
    fn returns_cm_for_900 () {
      assert_eq!(numeralise(900), "CM");
    }

    #[test]
    fn returns_cmxcix_for_999 () {
      assert_eq!(numeralise(999), "CMXCIX");
    }

    #[test]
    fn returns_m_for_m () {
      assert_eq!(numeralise(1000), "M");
    }

    #[test]
    fn returns_mmm_for_m () {
      assert_eq!(numeralise(3000), "MMM");
    }
}

fn numeralise(number: u32) -> String {
  let mut numeral = String::from("");
  let mut counter = number;
  while counter > 0 {
    if counter < 4 {
      numeral.push_str("I");
      counter -= 1;
    } else if counter == 4 {
      numeral.push_str("IV");
      counter -= 4;
    } else if counter == 9 {
      numeral.push_str("IX");
      counter -= 9;
    } else if counter >= 1000 {
      numeral.push_str("M");
      counter -= 1000;
    }  else if counter >= 900 {
      numeral.push_str("CM");
      counter -= 900;
    } else if counter >= 500 {
      numeral.push_str("D");
      counter -= 500;
    } else if counter >= 400 {
      numeral.push_str("CD");
      counter -= 400;
    } else if counter >= 100 {
      numeral.push_str("C");
      counter -= 100;
    } else if counter >= 90 {
      numeral.push_str("XC");
      counter -= 90;
    } else if counter >= 50 {
      numeral.push_str("L");
      counter -= 50;
    } else if counter >= 10 {
      numeral.push_str("X");
      counter -= 10;
    } else if counter >= 5 {
      numeral.push_str("V");
      counter -= 5;
    }
  }
  return numeral.to_string();
}
