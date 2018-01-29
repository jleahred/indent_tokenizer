mod impl_token;
mod process_line;
mod parsing_lines;

use parsing_lines::ParsingLines;
use process_line::process_line;

use std::ops::Deref;

// ------------------------------------------------------------------
// ------------------------------------------------------------------
//  API

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct LineNum(u32);

#[derive(Debug, PartialEq, Clone, Eq)]
pub struct SLine(String);



#[derive(Debug, PartialEq)]
pub struct Token {
    pub lines: Vec<SLine>,
    pub tokens: Vec<Token>,
}

#[derive(Debug, PartialEq)]
pub struct Error {
    pub line: LineNum,
    pub desc: String,
}

pub fn tokenize(input: &str) -> Result<Vec<Token>, Error> {
    let mut parsing_lines = ParsingLines::new();

    for l in input.lines() {
        parsing_lines.add_opt_line(&process_line(&SLine::from(l)))?;
    }

    Ok(parsing_lines.add_tokens
        .get_tokens_and_close())
}

//  API
// ------------------------------------------------------------------
// ------------------------------------------------------------------



// todo:  pub(crate) when possible
#[derive(Debug, PartialEq, Copy, Clone)]
pub struct TokenLevel(usize);

#[derive(Debug, PartialEq, Eq, Copy, Clone, PartialOrd, Ord)]
pub struct NSpaces(usize);


impl SLine {
    pub fn new() -> Self {
        SLine(String::new())
    }
    pub fn from(s: &str) -> Self {
        SLine(String::from(s))
    }
}

impl Deref for SLine {
    type Target = str;
    fn deref(&self) -> &str {
        &self.0
    }
}



#[cfg(test)]
mod tests;
