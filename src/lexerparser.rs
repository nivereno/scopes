use std::{iter::{Peekable, Map}, collections::{hash_map, HashMap}, str::Chars, fs::File, io::Read, any::Any, fmt::Display, f64::{NAN, INFINITY}};
use anyhow::{anyhow, Result, Ok};
use num::{Num, NumCast, ToPrimitive};
use crate::{symbol::{Symbol, SymbolMap}, valueref::{Value, ValueRef}, Anchor};
use crate::lexerparser;
use crate::valueref;
use crate::num;

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
            result = NAN;
        } else if self.is_inf() {
            result = INFINITY;
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
    //fn as_integer<T>(&self) { //there's some template there might not be a fn
    //    let i = self.dot;
    //   assert!(i <= self.digits.len());
    //}
    fn as_int64(&self) -> i64 {
        let mut i = self.dot as usize;
        assert!(i <= self.digits.len());
        let mut exp: i64 = 1;
        let mut result: i64 = 0;
        while i > 0 {
            result += self.digits[i] as i64 * exp;
            exp *= self.base as i64;
            i -= 0;
        }
        if self.is_negative() {
            return -result;
        }
        return result;
    }
    fn as_uint64(&self) -> u64 {
        let mut i = self.dot as usize;
        assert!(i <= self.digits.len());
        let mut exp: u64 = 1;
        let mut result: u64 = 0;
        while i > 0 {
            result += self.digits[i] as u64 * exp;
            exp *= self.base as u64;
            i -= 0;
        }
        return result;
    }
    pub fn parse(&mut self, input: &Vec<u8>, index: &mut usize) -> bool {
        let mut state = State::State_UnknownSign;
        while *index < input.len() {
                let char = input[*index] as char;
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
                                    if input[*index + 1] == b'a' || input[*index + 1] == b'A' && input[*index + 2] == b'n' || input[*index + 2] == b'N' {
                                        *index += 3;
                                        self.flags |= NPF::NPF_NaN as u16;
                                        return true
                                        }
                                    }
                                    return false
                                }
                            'i' | 'I' => {
                                if input.len() >= 3 {
                                    if input[*index + 1] == b'n' || input[*index + 1] == b'N' && input[*index + 2] == b'f' || input[*index + 2] == b'F' {
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

#[derive(Clone, PartialEq)]
#[repr(u8)]
enum Token {
    tok_none = b'1',
    tok_eof = b'0',
    tok_open = b'(',
    tok_close = b')',
    tok_square_open = b'[',
    tok_square_close = b']',
    tok_curly_open = b'{',
    tok_curly_close = b'}',
    tok_string = b'"',
    tok_block_string = b'B',
    tok_syntax_quote = b'\'',
    tok_ast_quote = b'`',
    tok_symbol = b'S',
    tok_string_prefix = b'p',
    tok_escape = b'\\',
    tok_statement = b';',
    tok_number = b'N',
}

fn get_token_name() {

}

struct LexerParser<'a> {
    token: Token,
    base_offset: usize,
    file: &'a File,
    source: &'a Vec<u8>,
    input_stream: usize,
    eof: usize,
    cursor: usize,
    next_cursor: usize,
    lineno: usize,
    next_lineno: usize,
    line: usize,
    next_line: usize,

    string: usize,
    string_len: usize,

    value: ValueRef,
    list: &'a mut Vec<ValueRef>,
    prefix_symbol_map: &'a mut HashMap<Symbol, ValueRef>
}
fn is_token_terminator(char: u8) -> bool {
    match char {
        b'(' | b')' | b'[' | b']' | b'{' | b'}' | b'\"' | b'\'' | b';' | b'#' | b',' => return true,
        _ => return false
    }
}

impl <'a>LexerParser<'a> {
    fn is_suffix(&self, suffix: &[u8]) -> bool {
        let mut temp = self.string;
        for c in suffix {
            if *c != self.source[temp] {
                return false
            };
            temp += 1;
        }
        return true
    }

    fn verify_good_taste(&mut self, c: u8) -> Result<()> {
        if c == b'\t' {
            self.next_token();
            return Err(anyhow!("ParserBadTaste"));
        }
        return Ok(())
    }

    fn new(_list: &'a mut Vec<ValueRef>, source: &'a Vec<u8>, _file: &'a mut File, offset: usize, length: usize, _prefix_symbol_map: &'a mut HashMap<Symbol, ValueRef>) -> LexerParser<'a> {
        //let mut source = Vec::new();
        //_file.read_to_end(&mut source);
        let input_stream = 0 + offset;
        let end: usize;
        if length > 0 {
            end = input_stream + length;
        } else {
            end = source.len();
        }

        return LexerParser { token: Token::tok_eof, base_offset: offset, file: _file, source: source, input_stream: input_stream, eof: end, cursor: input_stream, next_cursor: input_stream, lineno: 1, next_lineno: 1, line: input_stream, next_line: input_stream, string: 0, string_len: 0, value: ValueRef{value: Value::None, anchor: Anchor::Anchor{}}, list: _list, prefix_symbol_map: _prefix_symbol_map }
    }
    fn offset(&self) -> usize {
        return self.base_offset + (self.cursor - self.input_stream)
    }
    fn column(&self) -> usize {
        return self.cursor - self.line + 1
    }
    fn next_column(&self) -> usize {
        return self.next_cursor - self.next_line + 1
    }
    fn anchor(&self) -> Anchor::Anchor {
        return Anchor::Anchor::from(Symbol(1), self.lineno, self.column(), self.offset()); //TODO Pass in file or filepath?
        todo!()
    }
    fn next(&mut self) -> Result<u8> {
        let c = self.source[self.next_cursor];
        self.verify_good_taste(c)?;
        self.next_cursor += 1;
        return Ok(c)
    }
    fn chars_left(&self) -> usize {
        return self.eof - self.next_cursor
    }
    fn is_eof(&self) -> bool {
        return self.next_cursor == self.eof
    }
    fn newline(&mut self) {
        self.next_lineno = self.next_lineno + 1;
        self.next_line = self.next_cursor;
    }
    fn select_string(&mut self) {
        self.string = self.cursor;
        self.string_len = self.next_cursor - self.cursor;
    }
    fn read_single_symbol(&mut self) {
        self.select_string();
    }
    fn read_symbol(&mut self) -> Result<()> {
        let mut escape = false;
        loop {
            if self.is_eof() {
                break;
            }
            let c = self.next()?;
            if escape {
                if c == b'\n' {
                    self.newline();
                }
                escape = false;
            } else if c == b'\\' {
                escape = true;
            } else if c.is_ascii_whitespace() || is_token_terminator(c) {
                self.next_cursor -= 1;
                break;
            }
        }
        self.select_string();
        return Ok(())
    }
    
    fn read_symbol_or_prefix(&mut self) -> Result<()> {
        self.token = Token::tok_symbol;
        let mut escape = false;
        loop {
            if self.is_eof() {
                break;
            }
            let c = self.next()?;
            if escape {
                if c == b'\n' {
                    self.newline();
                }
                escape = false;
            } else if c == b'\\' {
                escape = true;
            } else if c.is_ascii_whitespace() || is_token_terminator(c) {
                if c == b'"' {
                    self.token = Token::tok_string_prefix;
                }
                self.next_cursor -= 1;
                break;
            }
        }
        self.select_string();
        return Ok(())
    }
    fn read_string(&mut self, terminator: u8) -> Result<()> {
        let mut escape = false;
        loop {
            if self.is_eof() {
                return Err(anyhow!("ParserUnterminatedSequence"));
            }
            let c = self.next()?;
            if c == b'\n' && !escape {
                // 0.10
                //newline();
                // 0.11
                return Err(anyhow!("ParserUnexpectedLineBreak"));
            }
            if escape {
                escape = false;
            } else if c == b'\\' {
                escape = true;
            } else if c == terminator {  //TODO value::Block::terminator
                break;
            }
        }
        self.select_string();
        return Ok(())
    }
    fn read_block(&mut self) -> Result<()> {
        let indent = 0; //TODO platform dependent?
        let col = self.column() + indent;
        loop {
            if self.is_eof() {
                break;
            }
            let next_col = self.next_column();
            let c = self.next()?;
            if c == b'\n' {
                self.newline();
            } else if !c.is_ascii_whitespace() && next_col <= col {
                self.next_cursor -= 1;
                break;
            }
        }
        return Ok(())
    }
    fn read_block_string(&mut self) -> Result<()> {
        self.next()?;
        self.next()?;
        self.next()?;
        self.read_block()?;
        self.select_string();
        return Ok(())
    }
    fn read_comment(&mut self) -> Result<()> {
        self.read_block()?;
        return Ok(())
    }
    fn has_suffix(&self) -> bool {
        return self.string_len >= 1 && self.source[self.string] == b':'
    }
    fn select_integer_suffix(&mut self) -> Result<bool> { //TODO maybe replace helper with a macro... A wonky implimentation(Might be more efficient to use unsafe/union)
        if !self.has_suffix() {
            return Ok(false);
        }
        //if let Value::isize(i) = self.value {
            //} else {
        //    return Err(anyhow!("TODO"));
        //}
        match self.value.value {
            Value::f64(i) => return self.select_integer_suffix_helper(i),
            Value::f32(i) => return self.select_integer_suffix_helper(i),
            Value::i64(i) => return self.select_integer_suffix_helper(i),
            Value::i32(i) => return self.select_integer_suffix_helper(i),
            Value::i16(i) => return self.select_integer_suffix_helper(i),
            Value::i8(i) => return self.select_integer_suffix_helper(i),
            Value::u64(i) => return self.select_integer_suffix_helper(i),
            Value::u32(i) => return self.select_integer_suffix_helper(i),
            Value::u16(i) => return self.select_integer_suffix_helper(i),
            Value::u8(i) => return self.select_integer_suffix_helper(i),
            Value::char(i) => return self.select_integer_suffix_helper(i as u8),
            Value::isize(i) => return self.select_integer_suffix_helper(i),
            Value::usize(i) => return self.select_integer_suffix_helper(i),
            _ => return Err(anyhow!("TODO")),
    }
        
    }
    fn select_integer_suffix_helper<T: ToPrimitive>(&mut self, i: T) -> Result<bool> { //TODO switch to a match, probably invert the is_suffix
        if self.is_suffix(b":i8") {self.value = ValueRef{anchor: self.anchor(), value: Value::i8(i.to_i8().unwrap())};}
        else if self.is_suffix(b":i16") {self.value = ValueRef{anchor: self.anchor(), value: Value::i16(i.to_i16().unwrap())};}
        else if self.is_suffix(b":i32") {self.value = ValueRef{anchor: self.anchor(), value: Value::i32(i.to_i32().unwrap())};}
        else if self.is_suffix(b":i64") {self.value = ValueRef{anchor: self.anchor(), value: Value::i64(i.to_i64().unwrap())};}
        else if self.is_suffix(b":u8") {self.value = ValueRef{anchor: self.anchor(), value: Value::u8(i.to_u8().unwrap())};}
        else if self.is_suffix(b":u16") {self.value = ValueRef{anchor: self.anchor(), value: Value::u16(i.to_u16().unwrap())};}
        else if self.is_suffix(b":u32") {self.value = ValueRef{anchor: self.anchor(), value: Value::u32(i.to_u32().unwrap())};}
        else if self.is_suffix(b":u64") {self.value = ValueRef{anchor: self.anchor(), value: Value::u64(i.to_u64().unwrap())};}
        else if self.is_suffix(b":char") {self.value = ValueRef{anchor: self.anchor(), value: Value::char(i.to_u8().unwrap() as char)};}
        else if self.is_suffix(b":isize") {self.value = ValueRef{anchor: self.anchor(), value: Value::isize(i.to_isize().unwrap())};}
        else if self.is_suffix(b":usize") {self.value = ValueRef{anchor: self.anchor(), value: Value::usize(i.to_usize().unwrap())};}
        else if self.is_suffix(b":f32") {self.value = ValueRef{anchor: self.anchor(), value: Value::f32(i.to_f32().unwrap())};}
        else if self.is_suffix(b":f64") {self.value = ValueRef{anchor: self.anchor(), value: Value::f64(i.to_f64().unwrap())};}
        else {return Err(anyhow!("ParserInvalidIntegerSuffix"));} //ParserInvalidIntegerSuffix

        return Ok(true)
    }
    fn select_real_suffix(&mut self) -> Result<bool> {
        if !self.has_suffix() {
            return Ok(false);
        }

        if let Value::f64(i) = self.value.value {
            if self.is_suffix(b":f32") {self.value = ValueRef{anchor: self.anchor(), value: Value::f32(i.to_f32().unwrap())};}
            else if self.is_suffix(b":f64") {self.value = ValueRef{anchor: self.anchor(), value: Value::f64(i.to_f64().unwrap())};}
            else {return Err(anyhow!("ParserInvalidRealSuffix"));}
        } else {
            return Err(anyhow!("TODO"));
        }
        return Ok(true)
    }
    fn read_number(&mut self) -> Result<bool> {
        let mut number = NumberParser::new();
        let mut cend = self.cursor;
        if !number.parse(&self.source, &mut cend) || cend == self.cursor || cend > self.eof {return Ok(false);}
        self.next_cursor = cend;
        if number.is_real() {
            self.value = ValueRef{anchor: self.anchor(), value: Value::f64(number.as_double() as f64)};
        } else if number.is_signed() {
            let val = number.as_int64();
            if val >= -0x80000000 && val <= 0x7fffffff {
                self.value = ValueRef{anchor: self.anchor(), value: Value::i32(val as i32)};
            } else {
                self.value = ValueRef{anchor: self.anchor(), value: Value::i64(val as i64)};
            }
        } else {
            let val = number.as_uint64();
            if val <= 0x7fffffff {
                self.value = ValueRef{anchor: self.anchor(), value: Value::i32(val as i32)};
            } else if val <= 0xffffffff {
                self.value = ValueRef{anchor: self.anchor(), value: Value::u32(val as u32)};
            } else if val <= 0x7fffffffffffffff {
                self.value = ValueRef{anchor: self.anchor(), value: Value::i64(val as i64)};
            } else {
                self.value = ValueRef{anchor: self.anchor(), value: Value::u64(val as u64)};
            }
        }
        if cend == self.eof || self.source[cend].is_ascii_whitespace() || is_token_terminator(self.source[cend]) {
            return Ok(true);
        }
        if self.source[cend] != b':' {
            return Ok(false);
        }
        let _lineno = self.lineno; let _line = self.line; let _cursor = self.cursor;
        self.next_token();
        self.read_symbol()?;
        self.lineno = _lineno; self.line = _line; self.cursor = _cursor;
        if self.value.value != Value::None {
            return Ok(self.select_integer_suffix()?);
        } else {
            return Ok(false)
        }
    }
    pub fn next_token(&mut self) {
        self.lineno = self.next_lineno;
        self.line = self.next_line;
        self.cursor = self.next_cursor;
    }
    fn read_token(&mut self) -> Result<Token> {
        let mut c: u8;
        loop {
            self.next_token();
            if self.is_eof() {self.token = Token::tok_eof; break;}
            c = self.next()?;
            if c == b'\n' {self.newline();}
            if c.is_ascii_whitespace() {continue;}
            if c == b'#' {
                self.read_comment()?; 
                continue;
            } else if c == b'(' {
                self.token = Token::tok_open; break;
            } else if c == b')' {
                self.token = Token::tok_close; break;
            } else if c == b'[' {
                self.token = Token::tok_square_open; break;
            } else if c == b']' {
                self.token = Token::tok_square_close; break;
            } else if c == b'{' {
                self.token = Token::tok_curly_open; break;
            } else if c == b'}' {
                self.token = Token::tok_curly_close; break;
            } else if c == b'\\' {
                self.token = Token::tok_escape; break;
            } else if c == b'"' {
                if self.chars_left() >= 3 && self.source[self.next_cursor] == b'"' && self.source[self.next_cursor + 1] == b'"' && self.source[self.next_cursor + 2] == b'"' {
                    self.token = Token::tok_block_string;
                    self.read_block_string()?;
                    break;
                } else {
                    self.token = Token::tok_string;
                    self.read_string(c)?;
                    break;
                }
            } else if c == b';' {
                self.token = Token::tok_statement; break;
            } else if c == b'\'' {
                self.token = Token::tok_syntax_quote; break;
            } else if c == b'`' {
                self.token = Token::tok_ast_quote; break;
            } else if c == b',' {
                self.token = Token::tok_symbol;
                self.read_single_symbol();
                break;
            } else if self.read_number()? {
                self.token = Token::tok_number; break;
            } else {
                self.read_symbol_or_prefix()?; break;
            }
        }
        return Ok(self.token.clone())
    }
    fn get_symbol(&self, map: &mut SymbolMap) -> Symbol {
        let dest: Vec<u8> = self.source[self.string.. self.string_len].to_vec();
        return SymbolMap::add_symbol(map, String::from_utf8(dest).unwrap())
    }
    fn get_string(&self) -> String {
        let dest: Vec<u8> = self.source[self.string + 1.. self.string_len - 2].to_vec();
        return String::from_utf8(dest).unwrap();
    }
    fn get_unescaped_string(&self) -> String {
        let dest: Vec<u8> = self.source[self.string + 1.. self.string_len - 3].to_vec();
        return String::from_utf8(dest).unwrap();
    }
    fn get_block_string(&self) -> String {
        let strip_col = self.column() + 4;
        let len = self.string_len - 4;
        assert!(len >= 0);
        let mut start = self.string + 4;
        let mut end = start + len;
        let mut last_lf = end;
        while end != start {
            let c = self.source[end - 1];
            if c.is_ascii_whitespace() {break;}
            if c == b'\n' {
                last_lf = end;
            }
            end -= 1;
        }
        end = last_lf;
        let mut p = Vec::new();
        while start != end {
            let c = self.source[start + 1];
            p.push(c);
            if c == b'\n' {
                // strip leftside column
                for _ in 1..strip_col {
                    if start == end {break;}
                    if self.source[start] != b' ' && self.source[start] != b'\t' {break;}
                    start += 1;
                }
            }
        }
        return String::from_utf8(p).unwrap()
    }
    //fn get_number(&self) -> Value<T> {
    //    return (*self.value).clone()
    //}
    //fn get() {}
    fn parse_list(&mut self, map: &mut SymbolMap, end_token: &Token) -> Result<()> {
        let start_anchor = self.anchor();

        self.read_token()?;

        loop {
            if self.token == *end_token {
                break;
            } else if self.token == Token::tok_escape {
                let column = self.column();
                self.read_token()?;
                let v = self.parse_naked(map, column, end_token)?;
                self.list.push(v);
            } else if self.token == Token::tok_eof {
                return Err(anyhow!("ParserUnclosedOpenBracket"))
            } else if self.token == Token::tok_statement {
                //self.list.split(this->anchor());
                self.read_token()?;
            } else {
                let v = &mut self.parse_any(map)?;
                self.list.append(v);
                self.read_token()?;
            }
        }

        return Ok(())
    }
    fn parse_prefixed_string(&self) -> Result<ValueRef> {
        assert!(self.token != Token::tok_eof);
        let anchor = self.anchor();
        match self.token {
            Token::tok_string =>  return Ok(ValueRef{anchor: self.anchor(), value: Value::string(self.get_unescaped_string())}),
            Token::tok_block_string => return Ok(ValueRef{anchor: self.anchor(), value: Value::string(self.get_block_string())}),
            _ => {},
        }
        return Err(anyhow!("ParserUnexpectedToken"))
    }
    fn parse_any(&mut self, map: &mut SymbolMap) -> Result<Vec<ValueRef>> {
        assert!(self.token != Token::tok_eof);
        let anchor = self.anchor();
        match self.token {
            //Token::tok_open => return ValueRef{anchor: anchor, ConstPointer::list_from(self.parse_list(tok_close)?)},
            //Token::tok_square_open => return ValueRef{anchor: anchor, ConstPointer::list_from(List::from(ref(anchor, ConstInt::symbol_from(Symbol(SYM_SquareList))), self.parse_list(tok_square_close)?)))},
            //Token::tok_curly_open => return ValueRef{anchor: anchor, ConstPointer::list_from(List::from(ref(anchor, ConstInt::symbol_from(Symbol(SYM_CurlyList))), self.parse_list(tok_curly_close)?)))},
            Token::tok_close | Token::tok_square_close | Token::tok_curly_close => return Err(anyhow!("ParserStrayClosingBracket")),
            Token::tok_string => return Ok(vec!(ValueRef{anchor: anchor, value: Value::string(self.get_string())})),
            Token::tok_block_string => return Ok(vec!(ValueRef{anchor: anchor, value: Value::string(self.get_block_string())})),
            Token::tok_symbol => return Ok(vec!(ValueRef{anchor: anchor, value: Value::symbol(self.get_symbol(map))})),
            Token::tok_string_prefix => {
                let sym = self.get_symbol(map).clone();
                // cache existing symbols
                let wrapped: ValueRef;
                if let Some(name) = self.prefix_symbol_map.get(&sym).clone() {
                    wrapped = name.clone();
                } else {
                    let name = map.get_mapped_symbol_name(&sym).unwrap().clone();
                    let pref = map.add_symbol(format!("{}{}", "prefix:", name)).clone();
                    let wrappedsym = ValueRef{anchor: anchor, value: Value::symbol(pref)};
                    self.prefix_symbol_map.insert(sym, wrappedsym.clone());
                    wrapped = wrappedsym;
                }
                self.read_token()?;
                let prefix = self.parse_prefixed_string()?;
                return Ok(vec!(prefix, wrapped))
            },
            Token::tok_number => return Ok(vec!(self.value.clone())),
            Token::tok_syntax_quote => {
                self.read_token()?;
                if self.token == Token::tok_eof {
                    return Err(anyhow!("ParserUnterminatedQuote"));
                }
                //return ValueRef(anchor, ConstPointer::list_from(List::from(ref(anchor, ConstInt::symbol_from(Symbol(KW_SyntaxQuote))), SCOPES_GET_RESULT(parse_any()))));
            },
            Token::tok_ast_quote => {
                self.read_token()?;
                if self.token == Token::tok_eof {
                    return Err(anyhow!("ParserUnterminatedQuote"));
                }
                //return ValueRef(anchor, ConstPointer::list_from(List::from(ref(anchor, ConstInt::symbol_from(Symbol(KW_ASTQuote))), SCOPES_GET_RESULT(parse_any()))));
            },
            _ => {},
        }
        /*
        SCOPES_TRACE_PARSER(this->anchor());
        SCOPES_ERROR(ParserUnexpectedToken,
            this->cursor[0], (int)this->cursor[0]);
        */
        return Err(anyhow!("ParserUnexpectedToken"));
    }
    fn parse_naked(&mut self, map: &mut SymbolMap, column: usize, end_token: &Token) -> Result<ValueRef> {
        let mut lineno = self.lineno;
        let mut escape = false;
        let mut subcolumn = 0;
        let anchor = self.anchor();
        let mut unwrap_single = true;

        while self.token != Token::tok_eof {
            if self.token == *end_token {
                break;
            } else if self.token == Token::tok_escape {
                escape = true;
                self.read_token()?;
                if self.lineno <= lineno {
                    return Err(anyhow!("ParserStrayEscapeToken"));
                }
                lineno = self.lineno;
            } else if self.lineno > lineno {
                if subcolumn == 0 {
                    subcolumn = self.column();
                } else if self.column() != subcolumn {
                    return Err(anyhow!("ParserIndentationMismatch"));
                }
                if column != subcolumn {
                    if (column + 4) != subcolumn {
                        return Err(anyhow!("ParserBadIndentationLevel"));
                    }
                }
    
                escape = false;
                lineno = self.lineno;
                // keep adding elements while we're in the same line
                while self.token != Token::tok_eof && self.token != *end_token && self.lineno == lineno {
                    let v = self.parse_naked(map, subcolumn, end_token)?;
                    self.list.push(v);
                }
            } else if self.token == Token::tok_statement {
                self.read_token()?;
                unwrap_single = false;
                if !self.list.is_empty() {
                    break;
                }
            } else {
                let v = &mut self.parse_any(map)?;
                self.list.append(v);
                lineno = self.next_lineno;
                self.read_token()?;
            }
            if !escape || self.lineno > lineno && self.column() <= column {
                break;
            }
        }
        /*
        auto result = builder.get_result();
        if (unwrap_single && result && List::count(result) == 1) {
            return result->at;
        } else {
            return ValueRef(anchor, ConstPointer::list_from(result));
        }
        */
        todo!()
    }
    fn parse(&mut self, map: &mut SymbolMap) -> Result<()> {
        self.read_token()?;
        let mut lineno = 0;
        let anchor = self.anchor();
        let mut escape = false;

        while self.token != Token::tok_eof {
            if self.token == Token::tok_none {
                break;
            } else if self.token == Token::tok_escape {
                escape = true;
                self.read_token()?;
                if self.lineno <= lineno {
                    return Err(anyhow!("ParserStrayEscapeToken"));
                }
                lineno = self.lineno;
            } else if self.lineno > lineno {
                if self.column() != 1 {
                    return Err(anyhow!("ParserIndentationMismatch"));
                }
    
                escape = false;
                lineno = self.lineno;
                // keep adding elements while we're in the same line
                while self.token != Token::tok_eof && self.token != Token::tok_none && self.lineno == lineno {
                    let v = self.parse_naked(map, 1, &Token::tok_none)?;
                    self.list.push(v);
                }
            } else if self.token == Token::tok_statement {
                return Err(anyhow!("ParserStrayStatementToken"));
            } else {
                let v = &mut self.parse_any(map)?;
                self.list.append(v);
                lineno = self.next_lineno;
                self.read_token()?;
            }
        }

        todo!()
    }

}