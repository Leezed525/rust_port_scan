mod config;

use std::cmp::PartialEq;
use std::env;
use std::collections;
use std::fmt::{Display, Formatter};

pub use config::ScannerConfig;

enum ParserType {
    String,
    Int,
    Bool,
    Float,
    List,
    Dict,
}

#[derive(Clone)]
enum ParserValue {
    String(String),
    Int(i32),
    Bool(bool),
    Float(f32),
    List(Vec<ParserValue>),
    Dict(collections::HashMap<String, ParserValue>),
    Empty,
}
impl Display for ParserValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ParserValue::String(value) => write!(f, "{},type:{}", value, "String"),
            ParserValue::Int(value) => write!(f, "{},type:{}", value, "Int"),
            ParserValue::Bool(value) => write!(f, "{},type:{}", value, "Bool"),
            ParserValue::Float(value) => write!(f, "{},type:{}", value, "Float"),
            ParserValue::List(value) => {
                let mut result = String::new();
                for item in value.iter() {
                    result.push_str(&format!("{}, ", item));
                }
                write!(f, "{},type:{}", result, "List")
            }
            ParserValue::Dict(value) => {
                let mut result = String::new();
                for (key, item) in value.iter() {
                    result.push_str(&format!("{}: {}, ", key, item));
                }
                write!(f, "{},type:{}", result, "Dict")
            }
            ParserValue::Empty => write!(f, "Empty"),
        }
    }
}

impl ParserValue {
    fn set_value(&mut self, value: ParserValue) {
        *self = value;
    }

    pub fn get_value(&self) -> Result<&dyn std::any::Any, &'static str> {
        match self {
            ParserValue::String(s) => Ok(s as &dyn std::any::Any),
            ParserValue::Int(i) => Ok(i as &dyn std::any::Any),
            ParserValue::Bool(b) => Ok(b as &dyn std::any::Any),
            ParserValue::Float(f) => Ok(f as &dyn std::any::Any),
            ParserValue::List(v) => Ok(v as &dyn std::any::Any),
            ParserValue::Dict(d) => Ok(d as &dyn std::any::Any),
            ParserValue::Empty => Err("Empty value encountered"),
        }
    }
}


struct ParserItem {
    name: String,
    parser_type: ParserType,
    value: ParserValue,
    required: bool,
    default: ParserValue,
    description: String,
}

impl ParserItem {
    fn new(name: String, parser_type: ParserType, required: bool, default: ParserValue, description: String) -> ParserItem {
        ParserItem {
            name,
            parser_type,
            value: ParserValue::Empty,
            required,
            default,
            description,
        }
    }
}


pub struct Parser {
    items: Vec<ParserItem>,
}

impl Parser {
    fn new() -> Parser {
        Parser {
            items: Vec::new(),
        }
    }

    fn add_item(&mut self, item: ParserItem) {
        self.items.push(item);
    }

    pub fn print(&self) {
        for item in self.items.iter() {
            println!("{}: {}", item.name, item.value);
        }
    }

    pub fn get_value(&self, name: &str) -> &dyn std::any::Any {
        for item in self.items.iter() {
            if item.name == name {
                return item.value.get_value().unwrap();
            }
        }
        panic!("Key:{} not found", name);
    }
}


pub fn get_args() -> collections::VecDeque<String> {
    let args = env::args().collect();
    args
}

impl PartialEq for ParserValue {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (ParserValue::String(a), ParserValue::String(b)) => a == b,
            (ParserValue::Int(a), ParserValue::Int(b)) => a == b,
            (ParserValue::Bool(a), ParserValue::Bool(b)) => a == b,
            (ParserValue::Float(a), ParserValue::Float(b)) => a == b,
            (ParserValue::List(a), ParserValue::List(b)) => a == b,
            (ParserValue::Dict(a), ParserValue::Dict(b)) => a == b,
            (ParserValue::Empty, ParserValue::Empty) => true,
            _ => false,
        }
    }
}

fn check_config(mut config: Parser) -> Result<Parser, String> {
    for mut item in config.items.iter_mut() {
        if item.value == ParserValue::Empty {
            if item.required {
                println!("{} is required", item.name);
                return Err("Required value not found".to_string());
            }
            item.value.set_value(item.default.clone());
        }
    }
    Ok(config)
}


pub fn get_config_value(mut config: Parser) -> Result<Parser, String> {
    let args = get_args();
    for (i, arg) in args.iter().enumerate() {
        if arg.starts_with("--") {
            let name = arg.trim_start_matches("--");
            if i + 1 >= args.len() {
                return Err(format!("No value found for {}", name));
            }
            let value = args[i + 1].clone();

            config.items.iter_mut().for_each(|mut item| {
                if item.name == name {
                    match item.parser_type {
                        ParserType::String => item.value.set_value(ParserValue::String(value.clone())),
                        ParserType::Int => item.value.set_value(ParserValue::Int(value.parse().unwrap())),
                        ParserType::Bool => item.value.set_value(ParserValue::Bool(value.parse().unwrap())),
                        ParserType::Float => item.value.set_value(ParserValue::Float(value.parse().unwrap())),
                        _ => {}
                    }
                }
            });
        }
    }
    check_config(config)
}


pub fn get_scanner_config() -> Parser {
    let mut scanner_config = Parser::new();
    scanner_config.add_item(ParserItem::new("ip".to_string(), ParserType::String, true, ParserValue::Empty, "IP address to scan".to_string()));
    scanner_config.add_item(ParserItem::new("start".to_string(), ParserType::Int, false, ParserValue::Int(0), "Start port to scan".to_string()));
    scanner_config.add_item(ParserItem::new("end".to_string(), ParserType::Int, false, ParserValue::Int(65535), "End port to scan".to_string()));
    scanner_config.add_item(ParserItem::new("duration".to_string(), ParserType::Int, false, ParserValue::Int(100), "Duration of the scan in milliseconds".to_string()));
    scanner_config
}