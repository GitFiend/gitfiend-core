use crate::parser::standard_parsers::UNTIL_LINE_END;
use crate::parser::Parser;
use crate::{and, many, until_str};
use crate::{map, Input};
use std::collections::HashMap;

pub const P_CONFIG: Parser<HashMap<String, String>> = map!(
  many!(and!(until_str!("="), UNTIL_LINE_END)),
  |result: Vec<(String, String)>| { result.into_iter().collect() }
);
