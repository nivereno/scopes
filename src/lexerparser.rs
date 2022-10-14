use std::{iter::{Peekable, Map}, collections::{hash_map, HashMap}, str::Chars};

use crate::symbol::Symbol;


pub struct NumberParser {
    flags: u16,
    pub base: i32,
    pub dot: i32,
    pub digits: Vec<u8>,
    pub exponent_digits: Vec<u8>,
}

impl NumberParser {
    pub fn new() -> NumberParser {
        return NumberParser { flags: 0, base: 10, dot: 0, digits: Vec::new(), exponent_digits: Vec::new() }
    }
    fn is_real(&self) -> bool {
        return (self.flags & NPF::NPF_Real as u16) != 0
    }
    fn is_signed(&self) -> bool {
        return (self.flags & NPF::NPF_Sign as u16) != 0
    }
    fn is_negative(&self) -> bool {
        return (self.flags & NPF::NPF_Negative as u16) != 0
    }
    fn is_exponent_negative(&self) -> bool {
        return (self.flags & NPF::NPF_ExponentNegative as u16) != 0
    }
    fn has_exponent(&self) -> bool {
        return (self.flags & NPF::NPF_Exponent as u16) != 0
    }
    fn is_inf(&self) -> bool {
        return (self.flags & NPF::NPF_Inf as u16) != 0
    }
    fn is_nan(&self) -> bool {
        return (self.flags & NPF::NPF_NaN as u16) != 0
    }
    fn exponent_as_int64(&self) -> i64 {
        let mut exp: i64 = 1;
        let mut result: i64 = 0;
        for digit in &self.exponent_digits {
            result += *digit as i64 * exp as i64;
            exp *= 10 as i64;
        }
        return if self.is_exponent_negative() {-result} else {result}
    }
    fn as_double(&self) -> f64 {
        let mut result: f64 = 0.0;
        if self.is_nan() {
            todo!()
        } else if self.is_inf() {
            todo!()
        } else {
            let mut i = self.digits.len() as i32;
            for digit in &self.digits {
                let mut temp = self.dot - i - 1;
                let exp: f64 = (self.base as f64).powf(temp as f64) as f64;
                result += *digit as f64 * exp;
                i -= 1;
            }
            if self.has_exponent() {
                let exponent: i64 = self.exponent_as_int64();
                if self.base == 10 {
                    result *= (10.0 as f64).powf(exponent as f64) as f64;
                } else {
                    result *= (2 as f64).powf(exponent as f64);
                }
            }
        }
        
        return if self.is_negative() {-result} else {result}
    }
    fn some_template_idk(&self) { //there's some template there might not be a fn
        todo!()
    }
    fn as_int64(&self) -> i64 {
        todo!()
    }
    fn as_uint64(&self) -> u64 {
        todo!()
    }
    pub fn parse(&mut self, input: &Vec<char>, index: &mut usize) -> bool {
        let mut state = State::State_UnknownSign;
        while *index < input.len() {
                let char = input[*index];
                match &state {
                    State::State_UnknownSign => {
                        state = State::State_UnknownBase;
                        match char {
                            '+' => {
                                self.flags |= NPF::NPF_Sign as u16;
                            }
                            '-' => {
                                self.flags |= NPF::NPF_Sign as u16 | NPF::NPF_Negative as u16;
                            }
                            _ => {}
                        }
                    }
                    State::State_UnknownBase => {
                        match char {
                            'n' | 'N' => {
                                if input.len() >= 3 {
                                    if input[*index + 1] == 'a' || input[*index + 1] == 'A' && input[*index + 2] == 'n' || input[*index + 2] == 'N' {
                                        *index += 3;
                                        self.flags |= NPF::NPF_NaN as u16;
                                        return true
                                        }
                                    }
                                    return false
                                }
                            'i' | 'I' => {
                                if input.len() >= 3 {
                                    if input[*index + 1] == 'n' || input[*index + 1] == 'N' && input[*index + 2] == 'f' || input[*index + 2] == 'F' {
                                        *index += 3;
                                        self.flags |= NPF::NPF_Inf as u16;
                                        return true
                                        }
                                    }
                                    return false
                            }
                            '0' => {
                                state = State::State_ExpectBase;
                            }
                            _ => {
                                state = State::State_ExpectNumber;
                            }
                        }
                    }
                    State::State_ExpectBase => {
                        state = State::State_ExpectNumber;
                        match char {
                            'x' => {
                                self.base = 16;
                                self.flags |= NPF::NPF_Base as u16;
                            }
                            'b' => {
                                self.base = 2;
                                self.flags |= NPF::NPF_Base as u16;
                            }
                            'o' => {
                                self.base = 8;
                                self.flags |= NPF::NPF_Base as u16;
                            }
                            _ => {
                                self.digits.push(0)
                            }
                        }
                    }
                    State::State_ExpectNumber => {
                        match char {
                            '.' => {
                                if (self.flags & (NPF::NPF_Dot as u16 | NPF::NPF_Exponent as u16)) > 0 {
                                    state = State::State_End;
                                    break;
                                };
                                self.dot = self.digits.len() as i32;
                                self.flags |= NPF::NPF_Dot as u16;
                            }
                            'p' => {
                                if (self.flags & NPF::NPF_Exponent as u16) > 0 {
                                    state = State::State_End;
                                    break;
                                }
                                if self.base != 16 {
                                    state = State::State_End;
                                    break;
                                }
                                state = State::State_ExpectExponentSign;
                                self.flags |= NPF::NPF_ExponentSign as u16;
                            }
                            'e' => {
                                if self.base != 16 {
                                    if (self.flags & NPF::NPF_Exponent as u16) > 0 {
                                        state = State::State_End;
                                        break;
                                    }
                                    if self.digits.is_empty() {
                                        state = State::State_End;
                                        break;
                                    }
                                    state = State::State_ExpectExponentSign;
                                    self.flags |= NPF::NPF_Exponent as u16;
                                }
                            }
                            _ => {
                                let mut digit: u8 = 0;
                                match self.base {
                                    2 => {
                                        if char >= '0' && char <= '1' {
                                            digit = char as u8 - '0' as u8;
                                        } else {state = State::State_End; break;}
                                    }
                                    8 => {
                                        if char >= '0' && char <= '7' {
                                            digit = char as u8 - '0' as u8;
                                        } else {state = State::State_End; break;}
                                    }
                                    10 => {
                                        if char >= '0' && char <= '9' {
                                            digit = char as u8 - '0' as u8;
                                        } else {state = State::State_End; break;}
                                    }
                                    16 => {
                                        if char >= '0' && char <= '9' {
                                            digit = char as u8 - '0' as u8;
                                        } else if char >= 'A' && char <= 'F' {
                                            digit = char as u8 - 'A' as u8 + 10;
                                        } else if char >= 'a' && char <= 'f' {
                                            digit = char as u8 - 'a' as u8 + 10;
                                        } else {state = State::State_End; break;}
                                }
                                _ => {state = State::State_End; break;}
                            }
                                self.digits.push(digit);
                            }
                        }
                    }
                    State::State_ExpectExponentSign => {
                        state = State::State_ExpectExponent;
                        match char {
                            '+' => {
                                self.flags |= NPF::NPF_ExponentSign as u16;
                            }
                            '-' => {
                                self.flags |= NPF::NPF_ExponentSign as u16 | NPF::NPF_ExponentNegative as u16;
                            }
                            _ => {}
                        }
                    }
                    State::State_ExpectExponent => {
                        if char >= '0' && char <= '9' {
                            let temp = char.clone() as u8 - '0' as u8;
                            self.exponent_digits.push(temp);
                        } else {state = State::State_End; break;}
                    }
                    State::State_End => {
                        break;
                    }
                }
                *index += 1;
        }
    
        if (self.flags & NPF::NPF_Dot as u16) == 0 {
            self.dot = self.digits.len() as i32;
        }
        if self.digits.is_empty() {
            return false;
        }
        if (self.flags & NPF::NPF_Exponent as u16) > 0 {
            if self.exponent_digits.is_empty() {
                return false
            }
        }
        return true
    }
}  

#[repr(u16)]
pub enum NPF {
    NPF_Sign             = (1 << 0),
    NPF_Negative         = (1 << 1),
    NPF_Base             = (1 << 2),
    NPF_Dot              = (1 << 3),
    NPF_Exponent         = (1 << 4),
    NPF_ExponentSign     = (1 << 5),
    NPF_ExponentNegative = (1 << 6),
    NPF_Inf              = (1 << 7),
    NPF_NaN              = (1 << 8),
    NPF_Real = (1 << 3) | (1 << 4) | (1 << 7) | (1 << 8)
}

#[derive(PartialEq)]
enum State {
    State_UnknownSign = 0,
    State_UnknownBase = 1,
    State_ExpectBase = 2,
    State_ExpectNumber = 3,
    State_ExpectExponentSign = 4,
    State_ExpectExponent = 5,
    State_End,
}

enum RN {
    RN_Invalid = 0,
    RN_Untyped = 1,
    RN_Typed = 2,
}

struct ListBuilder { //probably unnecessery

}

enum Token {

}

fn get_token_name() {

}

struct LexerParser {
    token: Token,
    base_offset: i32,
    //file: &SourceFile,
    //input_stream: std::slice::Iter<char>,
    //eof: std::slice::Iter<char>,
    //cursor: std::slice::Iter<char>,
    //next_cursor: std::slice::Iter<char>,
    lineno: i32,
    next_lineno: i32,
    //line: std::slice::Iter<char>,
    //Next_line: std::slice::Iter<char>,

    //string: std::slice::Iter<char>,
    string_len: i32,

    //value: ValueRef,
    //prefix_Symbol_map: HashMap<Symbol, ConstIntRef>
}

impl LexerParser {


    fn is_suffix() {

    }

    fn verify_good_taste() {

    }

    fn new() {

    }
    fn offset() {

    }
    fn column() {

    }
    fn next_column() {

    }
    fn anchor() {

    }
    fn next() {

    }
    fn chars_left() {

    }
    fn is_eof() {

    }
    fn newline() {

    }
    fn select_string() {

    }
    fn read_single_symbol() {

    }
    fn read_symbol() {

    }
    fn read_symbol_or_prefix(){

    }
    fn read_string() {

    }
    fn read_block() {

    }
    fn read_block_string() {

    }
    fn read_comment() {

    }
    fn has_suffix() {

    }
    fn select_integer_suffix() {

    }
    fn select_real_suffix() {

    }
    fn read_number(input: &Vec<char>) {
        let mut number = NumberParser::new();
        let mut index = 0;
        if (!number.parse(input, &mut index) /*|| ||*/ ) {

        }
    }
    fn next_token() {

    }
    fn read_token() {

    }
    fn get_symbol() {

    }
    fn get_string() {

    }
    fn get_unescaped_string(){

    }
    fn get_block_string() {

    }
    fn get_number() {

    }
    fn get() {

    }
    fn parse_list() {

    }
    fn parse_prefix_string() {

    }
    fn parse_any() {

    }
    fn parse_naked() {

    }
    fn parse() {

    }
    
}