use std::io::{stdin, stdout};
mod bin;

#[cfg(test)]
mod bin_tests;

fn main() {
    bin::main_loop(stdin().lock(), stdout());
}
