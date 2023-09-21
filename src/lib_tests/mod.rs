use super::{hydrometer_correction, convert_to_f};
mod test_data;

#[test]
fn hydrometer_correction_tests() {
  for (sg, temp, cal_temp, expected) in test_data::HYDROMETER_CORRECTION_TEST_DATA {
    let corrected = hydrometer_correction(sg, temp, cal_temp);
    assert!((corrected - expected).abs() < 0.001, "Expected {}, got {}", expected, corrected);
  }
}

#[test]
fn convert_to_f_tests() {
  for (c, f) in test_data::CONVERT_TO_F_TEST_DATA {
    let converted = convert_to_f(c);
    assert!((converted - f).abs() < 0.000001, "Expected {}, got {}", f, converted);
  }
}