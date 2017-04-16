// PLAN ------------------------------------


// init
//                         - lines: []
//                           tokens: []



// add_line("A1")
//                         - lines: [A1]
//                           tokens: []

// add_line("A2")
//                         - lines: [A1, A2]
//                           tokens: []


// add_token()
//                         - lines: [A1, A2]
//                           tokens: []
//                         - lines: []
//                           tokens: []


// add_line("B1")
//                         - lines: [A1, A2]
//                           tokens: []
//                         - lines: [B1]
//                           tokens: []


// add_line("B2")
//                         - lines: [A1, A2]
//                           tokens: []
//                         - lines: [B1, B2]
//                           tokens: []

// add_line("B3")
//                         - lines: [A1, A2]
//                           tokens: []
//                         - lines: [B1, B2, B3]
//                           tokens: []

// add_sub_token()
//                         - lines: [A1, A2]
//                           tokens: []
//                         - lines: [B1, B2, B3]
//                           tokens:
//                             - lines: []
//                               tokens: []



// add_line("BA1")
// add_line("BA2")
// add_line("BA3")
// add_line("BA4")
//                         - lines: [A1, A2]
//                           tokens: []
//                         - lines: [B1, B2, B3]
//                           tokens:
//                             - lines: [BA1, BA2, BA3, BA4]
//                               tokens: []


// add_sub_token()
//                         - lines: [A1, A2]
//                           tokens: []
//                         - lines: [B1, B2, B3]
//                           tokens:
//                             - lines: [BA1, BA2, BA3, BA4]
//                               tokens:
//                                 - lines: []
//                                   tokens: []


// add_line("BAA1")
// add_line("BAA2")
//                         - lines: [A1, A2]
//                           tokens: []
//                         - lines: [B1, B2, B3]
//                           tokens:
//                             - lines: [BA1, BA2, BA3, BA4]
//                               tokens:
//                                 - lines: [BAA1, BAA2]
//                                   tokens: []

// add_token()
// add_line("BAB1")
// add_line("BAB2")
//                         - lines: [A1, A2]
//                           tokens: []
//                         - lines: [B1, B2, B3]
//                           tokens:
//                             - lines: [BA1, BA2, BA3, BA4]
//                               tokens:
//                                 - lines: [BAA1, BAA2]
//                                   tokens: []
//                                 - lines: [BAB1, BAB2]
//                                   tokens: []


// add_sub_token()
// add_line("BABA1")
// add_line("BABA2")
// add_line("BABA3")
//                         - lines: [A1, A2]
//                           tokens: []
//                         - lines: [B1, B2, B3]
//                           tokens:
//                             - lines: [BA1, BA2, BA3, BA4]
//                               tokens:
//                                 - lines: [BAA1, BAA2]
//                                   tokens: []
//                                 - lines: [BAB1, BAB2]
//                                   tokens:
//                                     - lines: [BABA1, BABA2, BABA3]
//                                       tokens: []


// add_back_token(1)
// add_line("BB1")
// add_line("BB2")
//                         - lines: [A1, A2]
//                           tokens: []
//                         - lines: [B1, B2, B3]
//                           tokens:
//                             - lines: [BA1, BA2, BA3, BA4]
//                               tokens:
//                                 - lines: [BAA1, BAA2]
//                                   tokens: []
//                                 - lines: [BAB1, BAB2]
//                                   tokens:
//                                     - lines: [BABA1, BABA2, BABA3]
//                                       tokens: []
//                             - lines: [BB1, BB2]
//                               tokens: []
// PLAN ------------------------------------



use ::Token;



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

    pub fn prune(&mut self) -> &Self {
        let _ = self.tokens.iter_mut().map(|t| t.prune());
        match self.tokens.len() {
            0 => (),
            len => {
                if self.tokens[len - 1].is_empty() {
                    self.tokens.pop();
                }
            }
        };
        self
    }

    fn get_token_level(&mut self, level: usize) -> &mut Self {
        let len = self.tokens.len();
        match level {
            0 => self,
            _ => {
                if self.tokens.len() == 0 {
                    self.tokens.push(Token::new());
                };
                (&mut self.tokens[len - 1]).get_token_level(level - 1)
            }
        }
    }
}

#[derive(Debug)]
pub struct AddTokens {
    new_token: Token, //  last created token, pending insert in proper possition
    nt_level: usize, //  nt will be inserted allways at end. We only need the level
    root_token: Token, //  Tokens inserted and closed
}

impl AddTokens {
    pub fn new() -> Self {
        AddTokens {
            new_token: Token::new(),
            nt_level: 0,
            root_token: Token::new(),
        }
    }

    pub fn add_line(&mut self, line: &str) -> &mut Self {
        self.new_token.lines.push(line.to_owned());
        self
    }

    pub fn add_token(&mut self) -> &mut Self {
        if self.new_token.is_empty() == false {
            use std::mem;
            let mut mv_token = Token::new();
            mem::swap(&mut mv_token, &mut self.new_token);
            self.get_nw_tk_parent().tokens.push(mv_token);
        }
        //  index keep same position
        self
    }

    pub fn add_sub_token(&mut self) -> &mut Self {
        self.add_token();   //  consolidate token
        self.nt_level += 1;
        self
    }

    pub fn add_back_token(&mut self, level: usize) -> &mut Self {
        self.add_token();   //  consolidate token
        self.nt_level = level;
        self
    }

    pub fn get_tokens_and_close(&mut self) -> Vec<Token> {
        self.add_token();
        self.root_token.prune();
        let mut result = Vec::new();
        use std::mem;
        mem::swap(&mut result, &mut self.root_token.tokens);
        result
    }


    // --------------------------------
    fn get_nw_tk_parent(&mut self) -> &mut Token {
        self.root_token.get_token_level(self.nt_level)
    }
}



//  ---------------------------------------------------------
//  ---------------------------------------------------------
//
//          T E S T S


// init
//                         []
#[test]
fn init() {
    let tokens = AddTokens::new().get_tokens_and_close();
    assert!(tokens.len() == 0);
}


// add_line("A1")
//                         - lines: [A1]
//                           tokens: []
#[test]
fn add_first_line() {
    let tokens = AddTokens::new()
        .add_line("A1")
        .get_tokens_and_close();
    assert!(tokens.len() == 1);
    assert!(tokens[0].lines.len() == 1);
    assert!(tokens[0].lines[0] == "A1");
}


// add_line("A1")
// add_line("A2")
//                         - lines: [A1, A2]
//                           tokens: []
#[test]
fn add_two_root_lines() {
    let tokens = AddTokens::new()
        .add_line("A1")
        .add_line("A2")
        .get_tokens_and_close();
    println!("{:?} _________________", tokens);
    assert!(tokens.len() == 1);
    assert!(tokens[0].lines.len() == 2);
    assert!(tokens[0].lines[0] == "A1");
    assert!(tokens[0].lines[1] == "A2");
}


// add_line("A1")
// add_line("A2")
// add_token
//                         - lines: [A1, A2]
//                           tokens: []
//  prunning empties
#[test]
fn add_two_root_lines_and_token() {
    let tokens = AddTokens::new()
        .add_line("A1")
        .add_line("A2")
        .add_token()
        .get_tokens_and_close();
    assert!(tokens.len() == 1);
    assert!(tokens[0].lines.len() == 2);
    assert!(tokens[0].lines[0] == "A1");
    assert!(tokens[0].lines[1] == "A2");
}


// add_line("A1")
// add_line("A2")
// add_token
// add_line("B1")
//                         - lines: [A1, A2]
//                           tokens: []
//                         - lines: [B1]
//                           tokens: []
#[test]
fn add_token_root() {
    let tokens = AddTokens::new()
        .add_line("A1")
        .add_line("A2")
        .add_token()
        .add_line("B1")
        .get_tokens_and_close();
    assert!(tokens.len() == 2);
    assert!(tokens[0].lines.len() == 2);
    assert!(tokens[0].lines[0] == "A1");
    assert!(tokens[0].lines[1] == "A2");
    assert!(tokens[1].lines.len() == 1);
    assert!(tokens[1].lines[0] == "B1");
}


// add_line("A1")
// add_line("A2")
// add_token
// add_line("B1")
// add_line("B2")
//                         - lines: [A1, A2]
//                           tokens: []
//                         - lines: [B1, B2, B3]
//                           tokens: []
#[test]
fn add_token_root_aditional_lines() {
    let tokens = AddTokens::new()
        .add_line("A1")
        .add_line("A2")
        .add_token()
        .add_line("B1")
        .add_line("B2")
        .add_line("B3")
        .get_tokens_and_close();
    assert!(tokens.len() == 2);
    assert!(tokens[0].lines.len() == 2);
    assert!(tokens[0].lines[0] == "A1");
    assert!(tokens[0].lines[1] == "A2");
    assert!(tokens[1].lines.len() == 3);
    assert!(tokens[1].lines[0] == "B1");
    assert!(tokens[1].lines[1] == "B2");
    assert!(tokens[1].lines[2] == "B3");
}


// add_line("A1")
// add_line("A2")
// add_token
// add_line("B1")
// add_line("B2")
// add_sub_token()
// add_line("BA1")
// add_line("BA2")
// add_line("BA3")
// add_line("BA4")
//                         - lines: [A1, A2]
//                           tokens: []
//                         - lines: [B1, B2, B3]
//                           tokens:
//                             - lines: [BA1, BA2, BA3, BA4]
//                               tokens: []
#[test]
fn add_sub_token() {
    let tokens = AddTokens::new()
        .add_line("A1")
        .add_line("A2")
        .add_token()
        .add_line("B1")
        .add_line("B2")
        .add_line("B3")
        .add_sub_token()
        .add_line("BA1")
        .add_line("BA2")
        .add_line("BA3")
        .add_line("BA4")
        .get_tokens_and_close();
    assert!(tokens.len() == 2);
    assert!(tokens[0].lines.len() == 2);
    assert!(tokens[0].lines[0] == "A1");
    assert!(tokens[0].lines[1] == "A2");
    assert!(tokens[1].lines.len() == 3);
    assert!(tokens[1].lines[0] == "B1");
    assert!(tokens[1].lines[1] == "B2");
    assert!(tokens[1].lines[2] == "B3");
    assert!(tokens[1].tokens[0].lines.len() == 4);
    assert!(tokens[1].tokens[0].lines[0] == "BA1");
    assert!(tokens[1].tokens[0].lines[1] == "BA2");
    assert!(tokens[1].tokens[0].lines[2] == "BA3");
    assert!(tokens[1].tokens[0].lines[3] == "BA4");
}




// add_line("A1")
// add_line("A2")
// add_token
// add_line("B1")
// add_line("B2")
// add_sub_token()
// add_line("BA1")
// add_line("BA2")
// add_line("BA3")
// add_line("BA4")
// add_sub_token()
// add_line("BAA1")
// add_line("BAA2")
//                         - lines: [A1, A2]
//                           tokens: []
//                         - lines: [B1, B2, B3]
//                           tokens:
//                             - lines: [BA1, BA2, BA3, BA4]
//                               tokens:
//                                 - lines: [BAA1, BAA2]
//                                   tokens: []
#[test]
fn add_sub_sub_token() {
    let tokens = AddTokens::new()
        .add_line("A1")
        .add_line("A2")
        .add_token()
        .add_line("B1")
        .add_line("B2")
        .add_line("B3")
        .add_sub_token()
        .add_line("BA1")
        .add_line("BA2")
        .add_line("BA3")
        .add_line("BA4")
        .add_sub_token()
        .add_line("BAA1")
        .add_line("BAA2")
        .get_tokens_and_close();
    assert!(tokens.len() == 2);
    assert!(tokens[0].lines.len() == 2);
    assert!(tokens[0].lines[0] == "A1");
    assert!(tokens[0].lines[1] == "A2");
    assert!(tokens[1].lines.len() == 3);
    assert!(tokens[1].lines[0] == "B1");
    assert!(tokens[1].lines[1] == "B2");
    assert!(tokens[1].lines[2] == "B3");
    assert!(tokens[1].tokens.len() == 1);
    assert!(tokens[1].tokens[0].lines.len() == 4);
    assert!(tokens[1].tokens[0].lines[0] == "BA1");
    assert!(tokens[1].tokens[0].lines[1] == "BA2");
    assert!(tokens[1].tokens[0].lines[2] == "BA3");
    assert!(tokens[1].tokens[0].lines[3] == "BA4");
    assert!(tokens[1].tokens[0].tokens.len() == 1);
    assert!(tokens[1].tokens[0].tokens.len() == 1);
    assert!(tokens[1].tokens[0].tokens[0].lines.len() == 2);
    assert!(tokens[1].tokens[0].tokens[0].lines[0] == "BAA1");
    assert!(tokens[1].tokens[0].tokens[0].lines[1] == "BAA2");
}



// add_line("A1")
// add_line("A2")
// add_token
// add_line("B1")
// add_line("B2")
// add_sub_token()
// add_line("BA1")
// add_line("BA2")
// add_line("BA3")
// add_line("BA4")
// add_sub_token()
// add_line("BAA1")
// add_line("BAA2")
// add_token()
// add_line("BAB1")
// add_line("BAB2")
// add_sub_token()
// add_line("BABA1")
// add_line("BABA2")
// add_line("BABA3")
//                         - lines: [A1, A2]
//                           tokens: []
//                         - lines: [B1, B2, B3]
//                           tokens:
//                             - lines: [BA1, BA2, BA3, BA4]
//                               tokens:
//                                 - lines: [BAA1, BAA2]
//                                   tokens: []
//                                 - lines: [BAB1, BAB2]
//                                   tokens:
//                                     - lines: [BABA1, BABA2, BABA3]
//                                       tokens: []
#[test]
fn add_sub_sub_sub_token() {
    let tokens = AddTokens::new()
        .add_line("A1")
        .add_line("A2")
        .add_token()
        .add_line("B1")
        .add_line("B2")
        .add_line("B3")
        .add_sub_token()
        .add_line("BA1")
        .add_line("BA2")
        .add_line("BA3")
        .add_line("BA4")
        .add_sub_token()
        .add_line("BAA1")
        .add_line("BAA2")
        .add_token()
        .add_line("BAB1")
        .add_line("BAB2")
        .add_sub_token()
        .add_line("BABA1")
        .add_line("BABA2")
        .add_line("BABA3")
        .get_tokens_and_close();
    assert!(tokens.len() == 2);
    assert!(tokens[0].lines.len() == 2);
    assert!(tokens[0].lines[0] == "A1");
    assert!(tokens[0].lines[1] == "A2");
    assert!(tokens[1].lines.len() == 3);
    assert!(tokens[1].lines[0] == "B1");
    assert!(tokens[1].lines[1] == "B2");
    assert!(tokens[1].lines[2] == "B3");
    assert!(tokens[1].tokens.len() == 1);
    assert!(tokens[1].tokens[0].lines.len() == 4);
    assert!(tokens[1].tokens[0].lines[0] == "BA1");
    assert!(tokens[1].tokens[0].lines[1] == "BA2");
    assert!(tokens[1].tokens[0].lines[2] == "BA3");
    assert!(tokens[1].tokens[0].lines[3] == "BA4");
    assert!(tokens[1].tokens[0].tokens.len() == 2);
    assert!(tokens[1].tokens[0].tokens[0].lines.len() == 2);
    assert!(tokens[1].tokens[0].tokens[0].lines[0] == "BAA1");
    assert!(tokens[1].tokens[0].tokens[0].lines[1] == "BAA2");
    assert!(tokens[1].tokens[0].tokens[1].lines.len() == 2);
    assert!(tokens[1].tokens[0].tokens[1].lines[0] == "BAB1");
    assert!(tokens[1].tokens[0].tokens[1].lines[1] == "BAB2");

    assert!(tokens[1].tokens[0].tokens[1].tokens.len() == 1);
    assert!(tokens[1].tokens[0].tokens[1].tokens[0].lines.len() == 3);
    assert!(tokens[1].tokens[0].tokens[1].tokens[0].lines[0] == "BABA1");
    assert!(tokens[1].tokens[0].tokens[1].tokens[0].lines[1] == "BABA2");
    assert!(tokens[1].tokens[0].tokens[1].tokens[0].lines[2] == "BABA3");
}



// add_line("A1")
// add_line("A2")
// add_token
// add_line("B1")
// add_line("B2")
// add_sub_token()
// add_line("BA1")
// add_line("BA2")
// add_line("BA3")
// add_line("BA4")
// add_sub_token()
// add_line("BAA1")
// add_line("BAA2")
// add_token()
// add_line("BAB1")
// add_line("BAB2")
// add_sub_token()
// add_line("BABA1")
// add_line("BABA2")
// add_line("BABA3")
// add_back_token(1)
// add_line("BB1")
// add_line("BB2")
//                         - lines: [A1, A2]
//                           tokens: []
//                         - lines: [B1, B2, B3]
//                           tokens:
//                             - lines: [BA1, BA2, BA3, BA4]
//                               tokens:
//                                 - lines: [BAA1, BAA2]
//                                   tokens: []
//                                 - lines: [BAB1, BAB2]
//                                   tokens:
//                                     - lines: [BABA1, BABA2, BABA3]
//                                       tokens: []
//                             - lines: [BB1, BB2]
//                               tokens: []
#[test]
fn add_back_token() {
    let gen_tokens = || -> Result<Vec<Token>, String> {
        Ok(AddTokens::new()
            .add_line("A1")
            .add_line("A2")
            .add_token()
            .add_line("B1")
            .add_line("B2")
            .add_line("B3")
            .add_sub_token()
            .add_line("BA1")
            .add_line("BA2")
            .add_line("BA3")
            .add_line("BA4")
            .add_sub_token()
            .add_line("BAA1")
            .add_line("BAA2")
            .add_token()
            .add_line("BAB1")
            .add_line("BAB2")
            .add_sub_token()
            .add_line("BABA1")
            .add_line("BABA2")
            .add_line("BABA3")
            .add_back_token(1)
            .add_line("BB1")
            .add_line("BB2")
            .get_tokens_and_close())
    };

    let tokens = gen_tokens().unwrap();

    assert!(tokens.len() == 2);
    assert!(tokens[0].tokens.len() == 0);
    assert!(tokens[0].lines.len() == 2);
    assert!(tokens[0].lines[0] == "A1");
    assert!(tokens[0].lines[1] == "A2");
    assert!(tokens[1].lines.len() == 3);
    assert!(tokens[1].lines[0] == "B1");
    assert!(tokens[1].lines[1] == "B2");
    assert!(tokens[1].lines[2] == "B3");

    assert!(tokens[1].tokens.len() == 2);

    assert!(tokens[1].tokens[0].lines.len() == 4);
    assert!(tokens[1].tokens[0].lines[0] == "BA1");
    assert!(tokens[1].tokens[0].lines[1] == "BA2");
    assert!(tokens[1].tokens[0].lines[2] == "BA3");
    assert!(tokens[1].tokens[0].lines[3] == "BA4");
    assert!(tokens[1].tokens[0].tokens.len() == 2);
    assert!(tokens[1].tokens[0].tokens[0].tokens.len() == 0);
    assert!(tokens[1].tokens[0].tokens[0].lines.len() == 2);
    assert!(tokens[1].tokens[0].tokens[0].lines[0] == "BAA1");
    assert!(tokens[1].tokens[0].tokens[0].lines[1] == "BAA2");
    assert!(tokens[1].tokens[0].tokens[1].lines.len() == 2);
    assert!(tokens[1].tokens[0].tokens[1].lines[0] == "BAB1");
    assert!(tokens[1].tokens[0].tokens[1].lines[1] == "BAB2");
    assert!(tokens[1].tokens[0].tokens[1].tokens.len() == 1);
    assert!(tokens[1].tokens[0].tokens[1].tokens[0].tokens.len() == 0);
    assert!(tokens[1].tokens[0].tokens[1].tokens[0].lines.len() == 3);
    assert!(tokens[1].tokens[0].tokens[1].tokens[0].lines[0] == "BABA1");
    assert!(tokens[1].tokens[0].tokens[1].tokens[0].lines[1] == "BABA2");
    assert!(tokens[1].tokens[0].tokens[1].tokens[0].lines[2] == "BABA3");

    assert!(tokens[1].tokens[1].lines.len() == 2);
    assert!(tokens[1].tokens[1].lines[0] == "BB1");
    assert!(tokens[1].tokens[1].lines[1] == "BB2");
}
