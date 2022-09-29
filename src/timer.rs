use std::{collections::HashMap};

use bimap::BiMap;

use crate::{symbol::{Symbol, KnownSymbol}, timer};

#[derive(Clone)]
pub struct Timer {
    active: bool,
    start: std::time::Instant,
    time: std::time::Duration,
}

impl Timer {
    pub fn pause(timers: &mut HashMap<Symbol, Timer>, name: &Symbol) {
        match timers.get_mut(name) {
            Some(Timer) => { 
                if Timer.active == true {
                    Timer.time += Timer.start.elapsed();
                    Timer.active = false;
                }
            },
            None => {} //Probably write something to log
        }
    }
    pub fn pause_self(&mut self) {
        if self.active == true {
            self.time += self.start.elapsed();
            self.active = false;
        }
    }
    pub fn resume_self(&mut self) {
        if self.active == false {
            self.start = std::time::Instant::now();
            self.active = true;
        }
    }
    pub fn resume(timers: &mut HashMap<Symbol, Timer>, name: &Symbol) {
        match timers.get_mut(name) {
            Some(Timer) => { 
                if Timer.active == false {
                    Timer.start = std::time::Instant::now();
                    Timer.active = true;
                }
            },
            None => {} //Probably write something to log
        }
    }
    pub fn print_timers(timers: &mut HashMap<Symbol, Timer>, symbols: &BiMap<String, Symbol>) {
        let mut real_sum = std::time::Duration::new(0, 0);
        timer::Timer::pause(timers, &mut Symbol(KnownSymbol::TIMER_Main as u64));
        let mut non_user_sum = timers.get_mut(&mut Symbol(KnownSymbol::TIMER_Main as u64)).unwrap().time;
        timer::Timer::resume(timers, &mut Symbol(KnownSymbol::TIMER_Main as u64));
        for (k, v) in timers {
            v.pause_self();
            let symbol = symbols.get_by_right(&k).unwrap();
            let time = v.time.as_millis();
            print!("key: {symbol} val: {time}\n");
            real_sum += v.time;
            v.resume_self();
        }
        let cumulative_real = real_sum.as_millis();
        let cumulative_user = (real_sum - non_user_sum).as_millis(); 
        print!("cumulative real: {cumulative_real} ms\n");
        print!("cumulative user: {cumulative_user} ms\n");
    }
    pub fn new(timers: &mut HashMap<Symbol, Timer>, name: Symbol) {
        match timers.get_mut(&name) {
            Some(Timer) => {}, //Probably write something to log
            None => {
                timers.insert(name, Timer{active: true, start: std::time::Instant::now(), time: std::time::Duration::new(0, 0)});
            }
        }
    }
}

