use std::collections::HashMap;

use crate::{timer::{Timer, self}, symbol::{Symbol, KnownSymbol}};


fn on_startup(timers: &mut HashMap<Symbol, Timer>) {
    timer::Timer::new(timers, Symbol(KnownSymbol::TIMER_Main as u64));
    //let main_compile_time = Timer(TIMER_Main);
}

pub fn sc_init(timers: &mut HashMap<Symbol, Timer>) {
    let path = std::env::current_dir().unwrap();

    on_startup(timers);

    //Symbol::_init_symbols();
    //init_llvm();

    //setup_stdio();


}