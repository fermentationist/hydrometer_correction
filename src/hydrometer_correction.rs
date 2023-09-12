use std::io;

const DEFAULT_CALIBRATION_TEMP: f64 = 60.0;

fn split_and_trim<'a> (input_str: &'a str, delim: &'a str) -> Vec<&'a str> {
  input_str.split(delim).collect::<Vec<&str>>().iter().map(|x| {x.trim()}).collect::<Vec<&str>>()
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
  let measured_temp = match temp_str.parse::<f64>() {
    Ok(temp) => temp,
    Err(_) => return None
  };
  let calibration_temp = calib_str.parse::<f64>().unwrap_or(DEFAULT_CALIBRATION_TEMP);
  Some((measured_sg, measured_temp, calibration_temp))
}

pub fn main_loop() {
  const MAIN_MESSAGE: &str = "Please enter a hydrometer reading (SG) with a temperature (in ºF) and an optional calibration temperature (in ºF, default is 60ºF), in the form: <measured gravity> @ <measured temperature> : <calibration_temperature>\nExample: 1.052 @ 93.8 : 68";
  loop {
    println!("{}", MAIN_MESSAGE);
    let mut input: String = String::new();
    let _user_input = io::stdin().read_line(&mut input).expect("Please enter a hydrometer reading (SG) with temperature (in ºF)");
    if input.trim() == "exit" {
      break;
    }
    let (measured_sg, measured_temp, calibration_temp) = match parse_input(&input.trim()){
      Some((sg, t, c)) => (sg, t, c),
      None => continue
    };
    // formula from https://homebrewacademy.com/hydrometer-temperature-correction/
    // formula found in js code, one displayed on webpage doesn't seem to work
    let adjusted = (measured_sg * (1.00130346 - 0.000134722124 * measured_temp + 0.00000204052596 * measured_temp.powi(2) - 0.00000000232820948 * measured_temp.powi(3))) / (1.00130346 - 0.000134722124 * calibration_temp + 0.00000204052596 * calibration_temp.powi(2) - 0.00000000232820948 * calibration_temp.powi(3));
    println!("Adjusted SG: {:.4}\n", adjusted);
  }

}


