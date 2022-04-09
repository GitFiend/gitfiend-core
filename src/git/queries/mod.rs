use crate::parser::Parser;
use crate::Input;
use crate::{and, take_char_while};

const P_GROUP: Parser<String> = take_char_while!(|c: char| { c != ',' });

pub const P_COMMIT_ROW: Parser<(String,)> = and!(P_GROUP);
