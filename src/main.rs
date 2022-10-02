use core::time;
use std::{collections::HashMap, thread};

use bimap::BiMap;

use crate::{symbol::{Symbol, KnownSymbol}, timer::Timer};


mod symbol;
mod boot;
mod timer;

fn main() {
    let SCOPES_SYMBOLS = vec![(String::from("TIMER_Main"), Symbol(KnownSymbol::TIMER_Main as u64)), (String::from(""), Symbol(KnownSymbol::SYM_Unnamed as u64)), (String::from("?corrupted?"), Symbol(KnownSymbol::SYM_Corrupted as u64)), (String::from("fn"), Symbol(KnownSymbol::KW_Fn as u64))];
    println!("{}", Symbol(42));
    let args = std::env::args();
    let mut timers = HashMap::new();
    let mut symbols: BiMap<String, Symbol> = SCOPES_SYMBOLS.into_iter().collect();
    //let mut symbols = BiMap::new();
    //symbols.insert(String::from("TIMER_Main"), Symbol(KnownSymbol::TIMER_Main as u64));
    boot::sc_init(&mut timers);
    symbols.insert(String::from("Pone"), Symbol(42));
    symbols.insert(String::from("poner"), Symbol(0));
    Timer::new(&mut timers, Symbol(42));
    Timer::new(&mut timers, Symbol(0));
    let millis = time::Duration::from_millis(200);
    thread::sleep(millis);
    Timer::print_timers(&mut timers, &symbols)
    //return sc_main();
}

