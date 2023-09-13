# hydrometer_correction

## Description
This is my first Rust project. It is a simple command line tool to correct hydrometer readings for temperature.

Hydrometers measure the density of a liquid relative to water. This is useful for homebrewing, as it allows you to measure the amount of sugar in a solution. However, hydrometers are calibrated to measure the density of water at a specific temperature, usually 60°F or 68°F. If you measure the density of a solution at a different temperature, you will need to correct the reading to get the actual density at the calibration temperature.

## Usage

You will need to have Rust installed on your system. If you do not have Rust installed, you can find instructions [here](https://www.rust-lang.org/tools/install).

1. Download the repository: `git clone https://github.com/fermentationist/hydrometer_correction.git`.
2. Change into the directory: `cd hydrometer_correction`.
3. You may either:
    - Run the program with `cargo run`.
    - Build the program with `cargo build --release`, and then run the binary - `./target/release/hydrometer_correction`. This binary can also be moved and run from anywhere on your system, or even copied to another system (without Rust) and run there.
    - Install the program with `cargo install --path .` and then run with the command `hydrometer_correction` from anywhere.
4. Follow the prompts to use. You may also type `help` or `h` at any time, for more detailed instructions.
---

### License

#### Copyright © 2023 [Dennis Hodges](https://dennis-hodges.com)

Permission to use, copy, modify, and/or distribute this software for any purpose with or without fee is hereby granted, provided that the above copyright notice and this permission notice appear in all copies.

THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.

Source: http://opensource.org/licenses/ISC