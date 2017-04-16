// TODO:
//      tests
//      types: remove u32 and string
//      try to remove for
//

// tokens example...

// This is the first token
//     This is another token, because it's on a different level
//         And another token
//     This is also a different token
//
// A token can contain
// multiple lines
//     This is another token
//     with three
//     lines
//
// Empty lines can be used to
// separate tokens
//     This is a token,
//     that continues
//     here. Next empty line define
//     a token division
//
//     And this is a different one
//     with a couple of lines



use process_line::{LineInfo, process_line};


// ------------------------------------------------------------------
// ------------------------------------------------------------------
//  API

#[derive(Debug)]
pub struct Token {
    pub lines: Vec<String>,
    pub tokens: Vec<Token>,
}




pub fn tokenize(input: &str) -> Result<Vec<Token>, String> {
    let mut parsing_inf = ParsingTokens::new();

    for l in input.lines() {
        parsing_inf = parsing_inf.add_line(&process_line(l))?;
    }

    // parsing_inf.root_token.prune();
    Ok(parsing_inf.root_token.tokens)
}

//  API
// ------------------------------------------------------------------
// ------------------------------------------------------------------




//  implementation



impl Token {
    pub fn new() -> Token {
        Token {
            lines: Vec::new(),
            tokens: Vec::new(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.lines.len() == 0 && self.tokens.len() == 0
    }
}




#[derive(Debug)]
struct ParsingTokens {
    prev_indent_spaces: Vec<u32>,
    root_token: Token,
}

impl ParsingTokens {
    fn new() -> ParsingTokens {
        ParsingTokens {
            prev_indent_spaces: Vec::new(),
            root_token: Token::new(),
        }
    }

    fn add_line(mut self, line: &Option<LineInfo>) -> Result<ParsingTokens, String> {
        match *line {
            None => {
                self.add_empty_token();
                Ok(self)
            }
            Some(ref l) => self.add_processed_line(l),
        }
    }

    fn add_processed_line(mut self, l: &LineInfo) -> Result<ParsingTokens, String> {
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
            None => {
                self.add_line_new_token(l);
                ()
            }
            Some(last_prev) => {
                use std::cmp::Ordering::{Equal, Greater, Less};
                match l.indent.cmp(&last_prev) {
                    Equal => self.add_line_last_token(l),
                    Greater => self.add_line_new_token_deep(l),
                    Less => self.add_line_new_token_shalow(l),
                }
            }
        }
        Ok(self)
    }

    // fn update_index(&mut self, l: &LineInfo) {
    //     self.prev_indent_spaces.push(PrevLevels {
    //         indent_spaces: l.indent,
    //         index: 0,
    //     });
    // }

    fn add_line_new_token(&mut self, l: &LineInfo) {
        // self.root_token.get_token_idx(0)
        // add_token_idx(0, Token::create(&l.content));
        // self.update_index(l);
    }

    fn add_line_last_token(&mut self, l: &LineInfo) {
        // self.root_token.get_last_token_idx(0).lines.push(l.content.clone());
    }

    fn add_line_new_token_deep(&mut self, l: &LineInfo) {
        // self.root_token.get_last_token_idx(0).create_sub(&l.content);
    }









    fn get_last_token_idx(&mut self) -> &mut Token {
        if self.root_token.tokens.is_empty() {
            self.root_token.tokens.push(Token::new());
        }

        match self.root_token.tokens.last_mut() {
            None => panic!("inconsistence!!!"),
            Some(token) => token,
        }
    }



    fn add_empty_token(&mut self) {
        let last_token = self.get_last_token_idx();
        match last_token.is_empty() {
            true => (),
            false => last_token.tokens.push(Token::new()),
        }
    }



    // fn remove_prev_indents_till(&mut self, indent: u32) {
    //     //  look for previous ident level
    //     while self.prevs.len() > 0 {
    //         match self.prevs.last().cloned() {
    //             None => break,
    //             Some(prev) => {
    //                 if indent > prev.indent_spaces {
    //                     self.prevs.pop();
    //                 }
    //             }
    //         }
    //     }
    // }

    fn add_line_new_token_shalow(&mut self, l: &LineInfo) {
        // self.prevs.pop();
        // self.remove_prev_indents_till(l.indent);

        // match self.prevs.last().cloned() {
        //     None => self.add_line_new_token_root(l),
        //     Some(PrevLevels { indent_spaces, .. }) => {
        //         use std::cmp::Ordering::{Equal, Greater, Less};
        //         match indent_spaces.cmp(&&l.indent) {
        //             Equal => (),
        //             Greater => Err("invalid inentation".to_string()),
        //             Less => panic!("Indent < new  were removed!!!"),
        //         }
        //     }
        // }
    }
}
