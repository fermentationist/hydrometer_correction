#[cfg(test)]
mod lib_tests;

pub fn hydrometer_correction(measured_sg: f64, measured_temp: f64, calibration_temp: f64) -> f64 {
  // formula from https://homebrewacademy.com/hydrometer-temperature-correction/
  // formula found in js code, one displayed on webpage doesn't seem to work
  (measured_sg * (1.00130346 - 0.000134722124 * measured_temp + 0.00000204052596 * measured_temp.powi(2) - 0.00000000232820948 * measured_temp.powi(3))) / (1.00130346 - 0.000134722124 * calibration_temp + 0.00000204052596 * calibration_temp.powi(2) - 0.00000000232820948 * calibration_temp.powi(3))
} 

pub fn convert_to_f(n: f64) -> f64 {
  9.0 / 5.0 * n + 32.0
}

