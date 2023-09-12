use std::io;

const DEFAULT_CALIBRATION_TEMP: f64 = 60.0;
const DIGITS: [char; 11] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '.'];
fn split_and_trim<'a> (input_str: &'a str, delim: &'a str) -> Vec<&'a str> {
  input_str.split(delim).collect::<Vec<&str>>().iter().map(|x| {x.trim()}).collect::<Vec<&str>>()
}

fn convert_to_f(n: f64) -> f64 {
  9.0 / 5.0 * n + 32.0
}

fn string_to_temp(temp_str: &str) -> Option<f64> {
  let lowercase_temp = temp_str.to_lowercase();
  let temp_unit = match lowercase_temp {
    lowercase_temp if lowercase_temp.contains("c") => "c",
    _ => "f"
  };
  let temp_str_digits: String = temp_str.chars().filter(|char| {DIGITS.contains(char)}).collect();
  match temp_str_digits.parse::<f64>() {
    Ok(temp) if temp_unit == "c" => Some(convert_to_f(temp)),
    Ok(temp) => Some(temp),
    Err(_) => None
  }
}

fn parse_input(input_str: &str) -> Option<(f64, f64, f64)> {
  let [sg_str, remaining_str] = split_and_trim(input_str, "@")[..] else { return None; };
  let split_remaining = split_and_trim(remaining_str, ":");
  let [temp_str, calib_str] = match split_remaining[..] {
    [] => return None,
    [temp] => [temp, "60"],
    [temp, calib] => [temp, calib],
    [temp, calib, ..] => [temp, calib]
  };
  let measured_sg = match sg_str.parse::<f64>() {
    Ok(sg) => sg,
    Err(_) => return None
  };
  let measured_temp = match string_to_temp(temp_str) {
    Some(temp) => temp,
    None => return None
  };
  let calibration_temp = match string_to_temp(calib_str) {
    Some(temp) => temp,
    None => DEFAULT_CALIBRATION_TEMP
  };

  Some((measured_sg, measured_temp, calibration_temp))
}

pub fn main_loop() {
  const INITIAL_MESSAGE: &str = "Please enter a hydrometer reading (SG) with a temperature (in ºF) and an optional calibration temperature (in ºF, default is 60ºF), in the form: <measured gravity> @ <measured temperature> : <calibration_temperature>\nExample: 1.052 @ 93.8 : 68";
  const VERBOSE_MESSAGE: &str = "To use, enter your hydrometer reading as a specific gravity (SG) followed by an \"@\" sign, and then the temperature at which you took the reading.\nThe temperature will be assumed to be in ºF, unless you specify ºC by appending a \"C\" to the number, like \"34.3C\", or \"34.3c\".\nAn optional calibration temperature may also be added, if your hydrometer does not use the default of 60ºF, by adding it after the measured temperature, separated by a colon (:), in the form: <measured gravity> @ <measured temperature> : <calibration_temperature>\nExample:\n1.052 @ 93.8 : 68";
  println!("{}\n", INITIAL_MESSAGE);
  loop {
    println!("Enter a specific gravity and a temperature:");
    let mut raw_input: String = String::new();
    let _user_input = io::stdin().read_line(&mut raw_input).expect("Please enter a hydrometer reading (SG) with temperature (in ºF)");
    let lowercased_input = raw_input.trim().to_lowercase();
    let input = lowercased_input.as_str();
    if input == "exit" {
      break;
    }
    if input == "help" || input == "h" || input == "-h" {
      println!("\n{}", VERBOSE_MESSAGE);
      continue;
    }
    let (measured_sg, measured_temp, calibration_temp) = match parse_input(&input){
      Some((sg, t, c)) => (sg, t, c),
      None => continue
    };
    // formula from https://homebrewacademy.com/hydrometer-temperature-correction/
    // formula found in js code, one displayed on webpage doesn't seem to work
    let adjusted = (measured_sg * (1.00130346 - 0.000134722124 * measured_temp + 0.00000204052596 * measured_temp.powi(2) - 0.00000000232820948 * measured_temp.powi(3))) / (1.00130346 - 0.000134722124 * calibration_temp + 0.00000204052596 * calibration_temp.powi(2) - 0.00000000232820948 * calibration_temp.powi(3));
    println!("Adjusted SG: {:.4}\n", adjusted);
  }

}


