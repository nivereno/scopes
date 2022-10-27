use std::{iter::{Peekable, Map}, collections::{hash_map, HashMap}, str::Chars, fs::File, io::Read, any::Any, fmt::Display, f64::{NAN, INFINITY}};
use anyhow::{anyhow, Result, Ok};
use crate::{symbol::{Symbol, SymbolMap}, valueref::{Value}};
use crate::lexerparser;
use crate::valueref;

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

#[derive(Clone)]
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
    //file: &'a File,
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

    value: Value,
    list: &'a mut Vec<LexerParser<'a>>
    //prefix_Symbol_map: HashMap<Symbol, ConstIntRef>https
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

    fn new(_list: &'a mut Vec<LexerParser<'a>>, source: &'a Vec<u8>, _file: &'a mut File, offset: usize, length: usize) -> LexerParser<'a> {
        //let mut source = Vec::new();
        //_file.read_to_end(&mut source);
        let input_stream = 0 + offset;
        let end: usize;
        if length > 0 {
            end = input_stream + length;
        } else {
            end = source.len();
        }

        return LexerParser { token: Token::tok_eof, base_offset: offset, source: source, input_stream: input_stream, eof: end, cursor: input_stream, next_cursor: input_stream, lineno: 1, next_lineno: 1, line: input_stream, next_line: input_stream, string: 0, string_len: 0, value: Value::None, list: _list }
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
    fn anchor() {
        return 
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
    fn select_integer_suffix(&mut self) -> Result<bool> {
        if !self.has_suffix() {
            return Ok(false);
        }
        
        if let Value::isize(i) = self.value {
            if self.is_suffix(b":i8") {self.value.anchor(); self.value = Value::i8(i as i8);}
            else if self.is_suffix(b":i16") {self.value.anchor(); self.value = Value::i16(i as i16);}
            else if self.is_suffix(b":i32") {self.value.anchor(); self.value = Value::i32(i as i32);}
            else if self.is_suffix(b":i64") {self.value.anchor(); self.value = Value::i64(i as i64);}
            else if self.is_suffix(b":u8") {self.value.anchor(); self.value = Value::u8(i as u8);}
            else if self.is_suffix(b":u16") {self.value.anchor(); self.value = Value::u16(i as u16);}
            else if self.is_suffix(b":u32") {self.value.anchor(); self.value = Value::u32(i as u32);}
            else if self.is_suffix(b":u64") {self.value.anchor(); self.value = Value::u64(i as u64);}
            else if self.is_suffix(b":char") {self.value.anchor(); self.value = Value::char(i as u8 as char);}
            else if self.is_suffix(b":isize") {self.value.anchor(); self.value = Value::isize(i as isize);}
            else if self.is_suffix(b":usize") {self.value.anchor(); self.value = Value::usize(i as usize);}
            else if self.is_suffix(b":f32") {self.value.anchor(); self.value = Value::f32(i as f32);}
            else if self.is_suffix(b":f64") {self.value.anchor(); self.value = Value::f64(i as f64);}
            else {return Err(anyhow!("ParserInvalidIntegerSuffix"));} //ParserInvalidIntegerSuffix
        } else {
            return Err(anyhow!("TODO"));
        }
        return Ok(true)
    }
    fn select_real_suffix(&mut self) -> Result<bool> {
        if !self.has_suffix() {
            return Ok(false);
        }

        if let Value::f64(i) = self.value {
            if self.is_suffix(b":f32") {}
            else if self.is_suffix(b":f64") {}
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
            self.value = Value::f64(number.as_double() as f64);
            self.value.anchor();
        } else if number.is_signed() {
            let val = number.as_int64();
            if val >= -0x80000000 && val <= 0x7fffffff {
                self.value = Value::i32(val as i32);
            } else {
                self.value = Value::i64(val as i64);
            }
        } else {
            let val = number.as_uint64();
            if val <= 0x7fffffff {
                self.value = Value::i32(val as i32)
            } else if val <= 0xffffffff {
                self.value = Value::u32(val as u32)
            } else if val <= 0x7fffffffffffffff {
                self.value = Value::i64(val as i64)
            } else {
                self.value = Value::u64(val as u64)
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
        if self.value != Value::None {
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