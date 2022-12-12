use std::collections::HashMap;

use bimap::BiMap;

use crate::{timer::{Timer, self}, symbol::{Symbol, KnownSymbol}, config};


fn on_startup(timers: &mut HashMap<Symbol, Timer>) {
    Timer::new(timers, Symbol(KnownSymbol::TIMER_Main as u64));
}

fn on_shutdown(timers: &mut HashMap<Symbol, Timer>, symbols: &mut BiMap<String, Symbol>) {
    if config::SCOPES_PRINT_TIMERS != 0 {
        Timer::print_timers(timers, symbols);
        // Print largest stack size I guess
    }
}
static mut signal_abort: bool = false;
fn abort(timers: &mut HashMap<Symbol, Timer>, symbols: &mut BiMap<String, Symbol>, abort: &bool) {
    on_shutdown(timers, symbols);
    if *abort {
        std::process::abort();
    } else {
        std::process::exit(1);
    }
}

fn f_exit(c: i32, timers: &mut HashMap<Symbol, Timer>, symbols: &mut BiMap<String, Symbol>) {
    on_shutdown(timers, symbols);
    std::process::exit(c);
}

fn load_custom_core() {

}

//------------------------------------------------------------------------------
// SCOPES CORE
//------------------------------------------------------------------------------

/* this function looks for a header at the end of the compiler executable
   that indicates a scopes core.

   the header has the format (core-size <size>), where size is a i32 value
   holding the size of the core source file in bytes.

   the compiler uses this function to override the default scopes core 'core.sc'
   located in the compiler's directory.

   to later override the default core file and load your own, cat the new core
   file behind the executable and append the header, like this:

   $ cp scopes myscopes
   $ cat mycore.sc >> myscopes
   $ echo "(core-size " >> myscopes
   $ wc -c < mycore.sc >> myscopes
   $ echo ")" >> myscopes

   */


//------------------------------------------------------------------------------
// MAIN
//------------------------------------------------------------------------------




pub fn init(timers: &mut HashMap<Symbol, Timer>, symbols: &mut BiMap<String, Symbol>) {
    let path = std::env::current_dir().unwrap();

    on_startup(timers);

    Symbol::init_symbols(symbols);
    //init_llvm();

    //setup_stdio();


}