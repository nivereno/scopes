use core::time;
use std::{collections::HashMap, thread};

use bimap::BiMap;

use crate::{symbol::{Symbol, KnownSymbol}, timer::Timer};


mod symbol;
mod boot;
mod timer;

fn main() {
    println!("{}", Symbol(42));
    let args = std::env::args();
    let mut timers = HashMap::new();
    let mut symbols = BiMap::new();
    Timer::new(&mut timers, Symbol(42));
    boot::init(&mut timers, &mut symbols);
    symbols.insert(String::from("Pone"), Symbol(42));
    symbols.insert(String::from("poner"), Symbol(0));
    
    Timer::new(&mut timers, Symbol(0));
    let millis = time::Duration::from_millis(200);
    thread::sleep(millis);
    Timer::print_timers(&mut timers, &symbols)
    //return sc_main();
}

