use std::{collections::HashMap, fmt, fs};

use crate::parser::Parser;

#[derive(Debug)]
pub enum JsonValue {
    String(String),
    Integer(isize),
    Float(f64),
    Boolean(bool),
    Object(HashMap<String, JsonValue>),
    Array(Vec<JsonValue>),
}

#[derive(Debug)]
pub enum Json {
    Object(HashMap<String, JsonValue>),
    Array(Vec<JsonValue>),
}

pub fn parse_from_file(file_path: &str) -> std::io::Result<Json> {
    let source = fs::read_to_string(file_path)?;
    return Ok(parse_from_string(source));
}

pub fn parse_from_string(source: String) -> Json {
    let mut parser = Parser::new(source);
    return parser.parse();
}

fn print_value(depth: i32, json_value: &JsonValue, f: &mut fmt::Formatter) -> fmt::Result {
    match json_value {
        JsonValue::Boolean(val) => {
            write!(f, "{val}")?;
        }
        JsonValue::Float(val) => {
            write!(f, "{val}")?;
        }
        JsonValue::Integer(val) => {
            write!(f, "{val}")?;
        }
        JsonValue::String(val) => {
            write!(f, "\"{val}\"")?;
        }
        JsonValue::Array(array) => {
            writeln!(f, "[")?;

            for val in array {
                for _ in 0..depth {
                    write!(f, "  ")?;
                }

                print_value(depth + 1, val, f)?;

                writeln!(f, ",")?;
            }

            for _ in 0..depth - 1 {
                write!(f, "  ")?;
            }
            write!(f, "]")?;
        }
        JsonValue::Object(object) => {
            writeln!(f, "{{")?;

            for (key, val) in object {
                for _ in 0..depth {
                    write!(f, "  ")?;
                }
                write!(f, "\"{key}\": ")?;

                print_value(depth + 1, val, f)?;

                writeln!(f, ",")?;
            }

            for _ in 0..depth - 1 {
                write!(f, "  ")?;
            }
            write!(f, "}}")?;
        }
    }

    return Ok(());
}

impl fmt::Display for Json {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            Json::Object(object) => {
                writeln!(f, "{{")?;

                for (key, val) in object {
                    write!(f, "  \"{key}\": ")?;
                    print_value(2, val, f)?;
                    writeln!(f, ",")?;
                }

                writeln!(f, "}}")?;
            }
            Json::Array(array) => {
                writeln!(f, "[")?;

                for val in array {
                    write!(f, "  ")?;
                    print_value(2, val, f)?;
                    writeln!(f, ",")?;
                }

                writeln!(f, "]")?;
            }
        }

        return Ok(());
    }
}
