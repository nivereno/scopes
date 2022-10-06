



struct NumberParser {
    flags: u16,
    base: i32,
    dot: i32,
    digits: Vec<u8>,
    exponent_digits: Vec<u8>,
}

impl NumberParser {
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
    fn parse(&mut self, input: Vec<char>) -> bool {
        let mut state = State::State_UnknownSign;
        for char in &input {
            while state != State::State_End {
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
                                    if (input[1] == 'a' || input[1] == 'A') && (input[2] == 'n' || input[2] == 'N') {
                                        //input += 3; Not sure what the point would be?
                                        self.flags |= NPF::NPF_NaN as u16;
                                        return true
                                    } else {
                                        return false
                                    }   
                                }
                                
                            }
                            'i' | 'I' => {
                                if input.len() >= 3 {
                                    if (input[1] == 'n' || input[1] == 'N') && (input[2] == 'f' || input[2] == 'F') {
                                    //input += 3; Not sure what the point would be?
                                    self.flags |= NPF::NPF_Inf as u16;
                                    return true
                                } else {
                                    return false
                                }
                                }
                                
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
                        
                    }
                    State::State_ExpectNumber => {
    
                    }
                    default => {
    
                    }
                    State::State_ExpectExponentSign => {
    
                    }
                    State::State_ExpectExponent => {
    
                    }
                }
            }
            
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