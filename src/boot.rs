use std::collections::HashMap;

use bimap::BiMap;

use crate::{timer::{Timer, self}, symbol::{Symbol, KnownSymbol}};


fn on_startup(timers: &mut HashMap<Symbol, Timer>) {
    Timer::new(timers, Symbol(KnownSymbol::TIMER_Main as u64));
}

pub fn init(timers: &mut HashMap<Symbol, Timer>, symbols: &mut BiMap<String, Symbol>) {
    let path = std::env::current_dir().unwrap();

    on_startup(timers);

    Symbol::init_symbols(symbols);
    //init_llvm();

    //setup_stdio();


}