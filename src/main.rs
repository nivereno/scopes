use core::time;
use std::{collections::HashMap, thread, fs::File};

use bimap::BiMap;
use num;
use crate::{symbol::{Symbol, KnownSymbol, SymbolMap}, timer::Timer, lexerparser::LexerParser};
use flate2;

mod types;
mod anchor;
mod symbol;
mod boot;
mod timer;
mod lexerparser;
mod valueref;
mod list;
mod config;
mod cache;
mod typename_type;
mod pointer_type;
mod function_type;
mod gc;
mod qualify_type;
mod refer_qualifier;
mod qualifier;
mod all_types;
mod unique_qualifiers;
mod image_type;
use anchor::Anchor;

fn main() {
    //let mut types = types::B_Types::default();
    //let mut types = types::B_Types::new(&mut types);

    let mut file = File::open("S.txt").unwrap();
    let U8vec = std::fs::read("S.txt").unwrap();
    let mut m = HashMap::new();
    let mut lexerparser = LexerParser::new(&U8vec, &mut file, 0, 0, &mut m);
    let mut placeholdermap = SymbolMap{map: BiMap::new(), num_symbols: 0};
    let list = lexerparser.parse(&mut placeholdermap).unwrap();


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

