mod impl_token;
mod process_line;
mod parsing_lines;

use parsing_lines::ParsingLines;
use process_line::process_line;


// ------------------------------------------------------------------
// ------------------------------------------------------------------
//  API

#[derive(Debug, PartialEq)]
pub struct Token {
    pub lines: Vec<String>,
    pub tokens: Vec<Token>,
}

#[derive(Debug, PartialEq)]
pub struct Error {
    pub line: u32,
    pub desc: String,
}

pub fn tokenize(input: &str) -> Result<Vec<Token>, Error> {
    let mut parsing_lines = ParsingLines::new();

    for l in input.lines() {
        parsing_lines.add_opt_line(&process_line(l))?;
    }

    Ok(parsing_lines.add_tokens
        .get_tokens_and_close())
}

//  API
// ------------------------------------------------------------------
// ------------------------------------------------------------------
