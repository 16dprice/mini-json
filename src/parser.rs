use std::collections::HashMap;

use crate::json::{Json, JsonValue};

#[derive(Debug)]
struct Token {
    start: usize,
    length: usize,
}

fn is_digit(c: char) -> bool {
    c >= '0' && c <= '9'
}

#[derive(Debug)]
pub struct Parser {
    source: String,
    start: usize,
    current: usize,
    line: usize,
}

impl Parser {
    pub fn new(source: String) -> Parser {
        Parser {
            source,
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn parse(&mut self) -> Json {
        self.skip_whitespace();

        let c = self.advance();

        if c == '{' {
            return Json::Object(self.parse_object());
        } else if c == '[' {
            return Json::Array(self.parse_array());
        } else {
            self.error("Can't parse non-object or non-array".to_string());
            return Json::Object(HashMap::new());
        }
    }

    fn error(&self, message: String) {
        panic!("[Error at line {}]: {}", self.line, message);
    }

    fn parse_array(&mut self) -> Vec<JsonValue> {
        let mut array = Vec::new();

        self.skip_whitespace();

        while !self.match_end_of_array() {
            match self.parse_value() {
                Ok(value) => {
                    array.push(value);
                }
                Err(error) => {
                    self.error(error);
                }
            }

            self.skip_whitespace();
            if self.peek() == ',' {
                self.advance();
            }
            self.skip_whitespace();
        }

        return array;
    }

    fn parse_object(&mut self) -> HashMap<String, JsonValue> {
        let mut properties = HashMap::new();

        self.skip_whitespace();

        while !self.match_end_of_object() {
            let c = self.advance();

            // TODO: should add error checking here because we always expect double quote?
            if c == '"' {
                self.parse_key(&mut properties);
            }

            self.skip_whitespace();
        }

        return properties;
    }

    fn parse_key(&mut self, properties: &mut HashMap<String, JsonValue>) {
        let key_token = self.parse_string();
        let key_lexeme = self.lexeme_from_token(key_token).to_owned();

        self.skip_whitespace();
        if !self.match_char(':') {
            self.error(format!("Expect colon after key: '{}'", key_lexeme));
        }
        self.skip_whitespace();

        match self.parse_value() {
            Ok(value) => {
                properties.insert(key_lexeme, value);
            }
            Err(error) => {
                self.error(error);
            }
        }
    }

    fn parse_value(&mut self) -> Result<JsonValue, String> {
        let c = self.advance();

        match c {
            '"' => {
                let value_token = self.parse_string();

                return Ok(JsonValue::String(
                    self.lexeme_from_token(value_token).to_owned(),
                ));
            }
            '{' => {
                let value = self.parse_object();
                return Ok(JsonValue::Object(value));
            }
            '[' => {
                let value = self.parse_array();
                return Ok(JsonValue::Array(value));
            }
            't' => {
                self.parse_true();
                return Ok(JsonValue::Boolean(true));
            }
            'f' => {
                self.parse_false();
                return Ok(JsonValue::Boolean(false));
            }
            _ => {
                if !(is_digit(c) || c == '-') {
                    return Err("Unexpected value".to_string());
                }

                let (value_token, is_float) = self.parse_number();

                if is_float {
                    let value = self.lexeme_from_token(value_token).parse::<f64>();

                    match value {
                        Ok(value) => {
                            return Ok(JsonValue::Float(value));
                        }
                        Err(err) => {
                            return Err(err.to_string());
                        }
                    }
                } else {
                    let value = self.lexeme_from_token(value_token).parse::<isize>();

                    match value {
                        Ok(value) => {
                            return Ok(JsonValue::Integer(value));
                        }
                        Err(err) => {
                            return Err(err.to_string());
                        }
                    }
                }
            }
        }
    }

    fn parse_string(&mut self) -> Token {
        self.start = self.current;
        self.advance();

        while !self.is_at_end() && self.peek() != '"' {
            self.advance();
        }

        let token = self.make_token();
        self.advance();

        return token;
    }

    fn parse_number(&mut self) -> (Token, bool) {
        self.start = self.current - 1;

        let mut is_float = false;

        while !self.is_at_end() && is_digit(self.peek()) || self.peek() == '.' {
            let c = self.advance();
            if c == '.' {
                is_float = true;
            }
        }

        let token = self.make_token();
        return (token, is_float);
    }

    fn parse_true(&mut self) {
        let r = self.advance();
        let u = self.advance();
        let e = self.advance();

        if !(r == 'r' && u == 'u' && e == 'e') {
            self.error("Unexpected value".to_string());
        }
    }

    fn parse_false(&mut self) {
        let a = self.advance();
        let l = self.advance();
        let s = self.advance();
        let e = self.advance();

        if !(a == 'a' && l == 'l' && s == 's' && e == 'e') {
            self.error("Unexpected value".to_string());
        }
    }

    fn make_token(&self) -> Token {
        Token {
            start: self.start,
            length: self.current - self.start,
        }
    }

    fn lexeme_from_token(&self, token: Token) -> &str {
        return &self.source[token.start..(token.start + token.length)];
    }

    fn is_at_end(&self) -> bool {
        self.current == self.source.len()
    }

    fn get_char_at_index(&self, index: usize) -> char {
        return self
            .source
            .chars()
            .nth(index)
            .expect(format!("Couldn't get char at index {}", index).as_str());
    }

    fn advance(&mut self) -> char {
        self.current += 1;
        return self.get_char_at_index(self.current - 1);
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }

        return self.get_char_at_index(self.current);
    }

    fn match_char(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        if self.peek() != expected {
            return false;
        }

        self.current += 1;
        return true;
    }

    fn match_end_of_array(&mut self) -> bool {
        if self.is_at_end() {
            return false;
        }
        if self.peek() != ']' {
            return false;
        }

        self.current += 1;

        if self.peek() == ',' {
            self.current += 1;
        }

        return true;
    }

    fn match_end_of_object(&mut self) -> bool {
        if self.is_at_end() {
            return false;
        }
        if self.peek() != '}' {
            return false;
        }

        self.current += 1;

        if self.peek() == ',' {
            self.current += 1;
        }

        return true;
    }

    fn skip_whitespace(&mut self) {
        loop {
            if self.is_at_end() {
                break;
            }

            let c = self.peek();

            match c {
                ' ' | '\r' | '\t' => {
                    self.advance();
                }
                '\n' => {
                    self.line += 1;
                    self.advance();
                }
                _ => {
                    break;
                }
            }
        }
    }
}
