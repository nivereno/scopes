use core::time;
use std::{collections::HashMap, thread};

use bimap::BiMap;

use crate::{symbol::{Symbol, KnownSymbol}, timer::Timer};


mod symbol;
mod boot;
mod timer;
mod lexerparser;

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
    test_parse_number.parse(vec!['+' as char, '0' as char, 'x' as char, '1' as char, '2' as char, '3' as char]);
    println!("{:?} {:?}", test_parse_number.digits, test_parse_number.exponent_digits);
    Timer::print_timers(&mut timers, &symbols)
}

