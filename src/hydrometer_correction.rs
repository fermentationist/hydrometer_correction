use std::io;

fn parse_input (input_str: &str) -> Option<(f64, f64, f64)> {
  let split: Vec<&str> = input_str.split(" ").collect();
  if split.len() >= 3 && split[1] == "@"  {
    let measured_sg: f64 = match split[0].trim().parse::<f64>() {
      Ok(sg) => sg,
      Err(_) => return None
    };
    
    let measured_temp: f64 = match split[2].trim().parse::<f64>() {
      Ok(temp) => temp,
      Err(_) => return None
    };
    let mut calibration_temp: f64 = 60.0;
    if split.len() == 5 {
      calibration_temp = split[4].trim().parse::<f64>().unwrap_or(calibration_temp);
    }
    return Some((measured_sg, measured_temp, calibration_temp))
  }
  None
}

pub fn main_loop () {
  const MAIN_MESSAGE: &str = "Please enter a hydrometer reading (SG) with a temperature (in ºF) and an optional calibration temperature (in ºF, default is 60ºF), in the form: <measured gravity> @ <measured temperature> : <calibration_temperature>\nExample: 1.052 @ 90.0 : 59";
  loop {
    println!("{}", MAIN_MESSAGE);
    let mut input: String = String::new();
    let _user_input = io::stdin().read_line(&mut input).expect("Please enter a hydrometer reading (SG) with temperature (in ºF)");
    if input == "exit" {
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


