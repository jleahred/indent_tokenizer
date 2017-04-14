// init
//                         - lines: []
//                             tokens: []



// add_line("A1")
//                         - lines: [A1]
//                             tokens: []

// add_line("A2")
//                         - lines: [A1, A2]
//                             tokens: []


// add_token()
//                         - lines: [A1, A2]
//                             tokens: []
//                         - lines: []
//                             tokens: []


// add_line("B1")
//                         - lines: [A1, A2]
//                             tokens: []
//                         - lines: [B1]
//                             tokens: []


// add_line("B2")
//                         - lines: [A1, A2]
//                             tokens: []
//                         - lines: [B1, B2]
//                             tokens: []

// add_line("B3")
//                         - lines: [A1, A2]
//                             tokens: []
//                         - lines: [B1, B2, B3]
//                             tokens: []

// add_sub_token()
//                         - lines: [A1, A2]
//                             tokens: []
//                         - lines: [B1, B2, B3]
//                             tokens:
//                             - lines: []
//                                 tokens: []



// add_line("BA1")
// add_line("BA2")
// add_line("BA3")
// add_line("BA4")

//                         - lines: [A1, A2]
//                             tokens: []
//                         - lines: [B1, B2, B3]
//                             tokens:
//                             - lines: [BA1, BA2, BA3, BA4]
//                                 tokens: []


// add_sub_token()
//                         - lines: [A1, A2]
//                             tokens: []
//                         - lines: [B1, B2, B3]
//                             tokens:
//                             - lines: [BA1, BA2, BA3, BA4]
//                                 tokens:
//                                 - lines: []
//                                     tokens: []


// add_line("BAA1")
// add_line("BAA2")
//                         - lines: [A1, A2]
//                             tokens: []
//                         - lines: [B1, B2, B3]
//                             tokens:
//                             - lines: [BA1, BA2, BA3, BA4]
//                                 tokens:
//                                 - lines: [BAA1, BAA2]
//                                     tokens: []

// add_token()
// add_line("BAB1")
// add_line("BAB2")
//                         - lines: [A1, A2]
//                             tokens: []
//                         - lines: [B1, B2, B3]
//                             tokens:
//                             - lines: [BA1, BA2, BA3, BA4]
//                                 tokens:
//                                 - lines: [BAA1, BAA2]
//                                     tokens: []
//                                 - lines: [BAB1, BAB2]
//                                     tokens: []


// add_sub_token()
// add_line("BABA1")
// add_line("BABA2")
// add_line("BABA3")
//                         - lines: [A1, A2]
//                             tokens: []
//                         - lines: [B1, B2, B3]
//                             tokens:
//                             - lines: [BA1, BA2, BA3, BA4]
//                                 tokens:
//                                 - lines: [BAA1, BAA2]
//                                     tokens: []
//                                 - lines: [BAB1, BAB2]
//                                     tokens:
//                                     - lines: [BABA1, BABA2, BABA3]
//                                         tokens: []


// add_back_token(1)
// add_line("BB1")
// add_line("BB2")
//                         - lines: [A1, A2]
//                             tokens: []
//                         - lines: [B1, B2, B3]
//                             tokens:
//                             - lines: [BA1, BA2, BA3, BA4]
//                                 tokens:
//                                 - lines: [BAA1, BAA2]
//                                     tokens: []
//                                 - lines: [BAB1, BAB2]
//                                     tokens:
//                                     - lines: [BABA1, BABA2, BABA3]
//                                         tokens: []
//                             - lines: [BB1, BB2]
//                                 tokens: []









#[derive(Debug)]
pub struct Token {
    pub lines: Vec<String>,
    pub tokens: Vec<Token>,
}




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
