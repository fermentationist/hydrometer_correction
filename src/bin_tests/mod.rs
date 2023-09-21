use super::bin::{main_loop, TITLE, INITIAL_MESSAGE, VERBOSE_MESSAGE, PROMPT, BAD_INPUT_MESSAGE};

mod test_data;

mod main_loop_tests {
  use super::*;

  // test helper function
  fn output_test(input: &str, expected: &str) {
    let input_w_exit = input.to_string() + "\nexit";
    let reader = input_w_exit.as_bytes();
    let mut writer = Vec::new();
    main_loop(reader, &mut writer);
    let output = String::from_utf8(writer).unwrap();
    // check if output CONTAINS expected
    assert!(output.contains(expected), "Test failed for input: {}\nExpected: {}\nActual: {}", input, expected, output);
  }

  // TESTS 
  #[test]
  fn initial_message() {
    const EXPECTED_OUTPUT: [&str; 3] = [TITLE, INITIAL_MESSAGE, PROMPT];
    for expected in EXPECTED_OUTPUT {
      output_test("", expected);
    }
  }

  #[test]
  fn help() {
    const HELP_TEST_DATA: [(&str, &str); 10] = [
      ("help", VERBOSE_MESSAGE),
      ("h", VERBOSE_MESSAGE),
      ("-h", VERBOSE_MESSAGE),
      ("--help", VERBOSE_MESSAGE),
      ("HELP", VERBOSE_MESSAGE),
      ("H", VERBOSE_MESSAGE),
      ("-H", VERBOSE_MESSAGE),
      ("--HELP", VERBOSE_MESSAGE),
      ("Help", VERBOSE_MESSAGE),
      ("--Help", VERBOSE_MESSAGE),
    ];
    for (input, expected) in HELP_TEST_DATA {
      output_test(input, expected);
    }
  }

  #[test]
  fn bad_input() {
    output_test("bad input", BAD_INPUT_MESSAGE);
  }

  #[test]
  fn cli_tests() {
    // Map through cli test inputs and add "Adjusted SG: " to expected outputs
    let inputs = test_data::CLI_TEST_INPUTS.iter().map(|(input, expected)| {
      let formatted = format!("Adjusted SG: {}", &expected);
      (*input, formatted)
    }).collect::<Vec<(&str, String)>>();

    for (input, expected) in inputs {
      output_test(input, &expected);
    }
  }
}
