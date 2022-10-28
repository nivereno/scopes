use core::time;
use std::{collections::HashMap, thread};

use bimap::BiMap;
use num;
use crate::{symbol::{Symbol, KnownSymbol}, timer::Timer};

mod symbol;
mod boot;
mod timer;
mod lexerparser;
mod valueref;

fn main() {
    let args = std::env::args();
    let mut timers = HashMap::new();
    let mut symbols = BiMap::new();
    Timer::new(&mut timers, Symbol(42));
    boot::init(&mut timers, &mut symbols);
    symbols.insert(String::from("Pone"), Symbol(42));
    symbols.insert(String::from("poner"), Symbol(0));
    Timer::new(&mut timers, Symbol(0));
    thread::sleep(time::Duration::from_millis(200));
    let mut test_parse_number = lexerparser::NumberParser::new();
    let mut some_string: Vec<u8> = vec!(b'+', b'0', b'x', b'1', b'2', b'3');
    let mut index = 0;
    test_parse_number.parse(&some_string, &mut index);
    println!("{:?} {:?}", test_parse_number.digits, test_parse_number.exponent_digits);
    Timer::print_timers(&mut timers, &symbols)
}

