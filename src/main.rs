use std::io::Write;
use term::color::{WHITE, GREEN, YELLOW, RED};

#[macro_use]
mod macros;

mod kanjis;
use kanjis::{Kanjis, Kanji, compiled::KANJIS};

mod settings;
mod sections;
mod utils;
use settings::*;
use utils::*;

fn main() {
    sections::menu()
}