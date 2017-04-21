use process_line::LineInfo;
use impl_token::AddTokens;

use {Error, LineNum, TokenLevel, NSpaces, SLine};




impl LineNum {
    fn inc(&self) -> Self {
        LineNum(self.0 + 1)
    }
}



#[derive(Debug)]
pub struct ParsingLines {
    line_counter: LineNum,
    prev_indent_spaces: Vec<NSpaces>,
    pub add_tokens: AddTokens,
}

impl ParsingLines {
    pub fn new() -> ParsingLines {
        ParsingLines {
            line_counter: LineNum(0),
            prev_indent_spaces: Vec::new(),
            add_tokens: AddTokens::new(),
        }
    }

    pub fn add_opt_line(&mut self, line: &Option<LineInfo>) -> Result<&ParsingLines, Error> {
        self.line_counter.inc();
        match *line {
            None => Ok(self.add_token()),
            Some(ref l) => self.add_line_info(l),
        }
    }

    fn add_line_info(&mut self, l: &LineInfo) -> Result<&ParsingLines, Error> {
        //  if prev_indents.last().is_none()
        //      create a new token same level
        //  else indent spaces > prev_indents.last()
        //      create a subtoken
        //  if indent spaces == prev_indents.last()
        //      insert on current token
        //  else
        //      create a new token same level

        // let last_indent: Option<u32> = *self.prev_indents.last();

        match self.prev_indent_spaces.last().cloned() {
            None => Ok(self.add_first_line(l)),

            Some(last_prev) => {
                use std::cmp::Ordering::{Equal, Greater, Less};
                match l.indent.cmp(&last_prev) {
                    Equal => Ok(self.add_line(&l.content)),
                    Greater => Ok(self.add_subtoken_line(l)),
                    Less => self.add_backtoken_line(&l),
                }
            }
        }
    }

    //  ------------------------------
    fn add_token(&mut self) -> &Self {
        self.add_tokens.add_token();
        self
    }

    fn add_first_line(&mut self, line: &LineInfo) -> &Self {
        self.prev_indent_spaces.push(line.indent);
        self.add_line(&line.content)
    }

    fn add_line(&mut self, content: &SLine) -> &Self {
        //  no index modif
        self.add_tokens
            .add_line(content);
        self
    }

    fn add_subtoken_line(&mut self, line: &LineInfo) -> &Self {
        self.prev_indent_spaces.push(line.indent);
        self.add_tokens
            .add_sub_token()
            .add_line(&line.content);
        self
    }

    fn add_backtoken_line(&mut self, line: &LineInfo) -> Result<&Self, Error> {
        let get_back_level = self.get_back_level_update_prevs(line.indent)?;
        self.add_tokens
            .add_back_token(get_back_level)
            .add_line(&line.content);
        Ok(self)
    }

    fn get_back_level_update_prevs(&mut self, spaces: NSpaces) -> Result<TokenLevel, Error> {
        fn get_error(line_counter: LineNum) -> Error {
            Error {
                line: line_counter,
                desc: "invalid indentation".to_owned(),
            }
        };

        use std::cmp::Ordering::{Equal, Greater, Less};
        let prev_spaces = self.prev_indent_spaces
            .last()
            .cloned()
            .ok_or(get_error(self.line_counter))?;

        match prev_spaces.cmp(&spaces) {
            Equal => Ok(TokenLevel(self.prev_indent_spaces.len() - 1)),
            Greater => {
                self.prev_indent_spaces.pop();
                self.get_back_level_update_prevs(spaces)
            }
            Less => Err(get_error(self.line_counter)),
        }
    }
}
