use {tokenize, Error, LineNum, Token, SLine};


// ------------------------------------------------------------------
//  TEST

#[test]
fn add_first() {
    let tokens = tokenize("....").unwrap();
    assert!(tokens.len() == 1);
    assert!(tokens[0].lines.len() == 1);
}



#[test]
fn empty_input() {
    let tokens = tokenize("").unwrap();
    assert!(tokens.len() == 0);
}



#[test]
fn empty_lines() {
    let tokens = tokenize("

")
        .unwrap();
    assert!(tokens.len() == 0);


    let tokens = tokenize("
        ")
        .unwrap();
    assert!(tokens.len() == 0);
}



#[test]
fn some_lines_one_token() {
    let tokens = tokenize("....
....")
        .unwrap();
    assert!(tokens.len() == 1);
    assert!(tokens[0].lines.len() == 2);


    let tokens = tokenize("....
....
....
....")
        .unwrap();
    assert!(tokens.len() == 1);
    assert!(tokens[0].lines.len() == 4);
}



#[test]
fn some_tokens_root_level_empty_line_separator() {
    let tokens = tokenize("1111

2222

3333
")
        .unwrap();
    assert!(tokens.len() == 3);
    assert!(tokens[0].lines[0] == SLine::from("1111"));
    assert!(tokens[1].lines[0] == SLine::from("2222"));
    assert!(tokens[2].lines[0] == SLine::from("3333"));



    let tokens = tokenize("00
01
02

10
11
12
13

20

30

    ")
        .unwrap();
    assert!(tokens.len() == 4);
    assert!(tokens[0].lines[0] == SLine::from("00"));
    assert!(tokens[0].lines[1] == SLine::from("01"));
    assert!(tokens[0].lines[2] == SLine::from("02"));
    assert!(tokens[1].lines[0] == SLine::from("10"));
    assert!(tokens[1].lines[1] == SLine::from("11"));
    assert!(tokens[1].lines[2] == SLine::from("12"));
    assert!(tokens[1].lines[3] == SLine::from("13"));
    assert!(tokens[2].lines[0] == SLine::from("20"));
    assert!(tokens[3].lines[0] == SLine::from("30"));
}



#[test]
fn nested_indent() {
    let tokens = tokenize("
0
    01
    02
        020
        021
        023
            0230
            0231
")
        .unwrap();

    let r = vec![Token {
                     lines: vec![SLine::from("0")],
                     tokens: vec![Token {
                                      lines: vec![SLine::from("01"), SLine::from("02")],
                                      tokens: vec![Token {
                                                       lines: vec![SLine::from("020"),
                                                                   SLine::from("021"),
                                                                   SLine::from("023")],
                                                       tokens: vec![Token {
                                                                        lines:
                                                                            vec![SLine::from("0230"),
                                                                                 SLine::from("0231")],
                                                                        tokens: vec![],
                                                                    }],
                                                   }],
                                  }],
                 }];

    assert!(tokens == r);
}




#[test]
fn back_indent() {
    let tokens = tokenize("
0
    01
    02
        020
        021
        023
            0230
            0231
    03
    04
    05
1
")
        .unwrap();

    let r = vec![Token {
                     lines: vec![SLine::from("0")],
                     tokens: vec![Token {
                                      lines: vec![SLine::from("01"), SLine::from("02")],
                                      tokens: vec![Token {
                                                       lines: vec![SLine::from("020"),
                                                                   SLine::from("021"),
                                                                   SLine::from("023")],
                                                       tokens: vec![Token {
                                                                        lines:
                                                                            vec![SLine::from("0230"),
                                                                                 SLine::from("0231")],
                                                                        tokens: vec![],
                                                                    }],
                                                   }],
                                  },
                                  Token {
                                      lines: vec![SLine::from("03"),
                                                  SLine::from("04"),
                                                  SLine::from("05")],
                                      tokens: vec![],
                                  }],
                 },
                 Token {
                     lines: vec![SLine::from("1")],
                     tokens: vec![],
                 }];

    assert!(tokens == r);
}




#[test]
fn complex() {
    let tokens = tokenize("
0a
0b
    01a
    01b
        010.a
        010.b
        010.c
            010..a
            010..b
1a
1b
1c
    1.a
    1.b
    1.c
            1..a
            1..b
2a

3a
    30.a
    30.b

    31.a

    32.a
        32..a
            320a
            320b

            321a
            321b
4a
4b

5a
")
        .unwrap();

    let r =
        vec![Token {
                 lines: vec![SLine::from("0a"), SLine::from("0b")],
                 tokens: vec![Token {
                                  lines: vec![SLine::from("01a"), SLine::from("01b")],
                                  tokens: vec![Token {
                                                   lines: vec![SLine::from("010.a"),
                                                               SLine::from("010.b"),
                                                               SLine::from("010.c")],
                                                   tokens: vec![Token {
                                                                    lines: vec![SLine::from("010.\
                                                                                             .a"),
                                                                                SLine::from("010.\
                                                                                             .b")],
                                                                    tokens: vec![],
                                                                }],
                                               }],
                              }],
             },
             Token {
                 lines: vec![SLine::from("1a"), SLine::from("1b"), SLine::from("1c")],
                 tokens: vec![Token {
                                  lines: vec![SLine::from("1.a"),
                                              SLine::from("1.b"),
                                              SLine::from("1.c")],
                                  tokens: vec![Token {
                                                   lines: vec![SLine::from("1..a"),
                                                               SLine::from("1..b")],
                                                   tokens: vec![],
                                               }],
                              }],
             },
             Token {
                 lines: vec![SLine::from("2a")],
                 tokens: vec![],
             },
             Token {
                 lines: vec![SLine::from("3a")],
                 tokens: vec![Token {
                                  lines: vec![SLine::from("30.a"), SLine::from("30.b")],
                                  tokens: vec![],
                              },
                              Token {
                                  lines: vec![SLine::from("31.a")],
                                  tokens: vec![],
                              },
                              Token {
                                  lines: vec![SLine::from("32.a")],
                                  tokens: vec![Token {
                                                   lines: vec![SLine::from("32..a")],
                                                   tokens: vec![Token {
                                                                    lines: vec![SLine::from("320a"),
                                                                                SLine::from("320b")],
                                                                    tokens: vec![],
                                                                },
                                                                Token {
                                                                    lines: vec![SLine::from("321a"),
                                                                                SLine::from("321b")],
                                                                    tokens: vec![],
                                                                }],
                                               }],
                              }],
             },
             Token {
                 lines: vec![SLine::from("4a"), SLine::from("4b")],
                 tokens: vec![],
             },
             Token {
                 lines: vec![SLine::from("5a")],
                 tokens: vec![],
             }];
    assert!(tokens == r);
}



#[test]
fn delimiters() {
    let tokens = tokenize("
0
    | 01a
     01b
     01c

     02a
     02b

        |020a
        |020b

        |  021a
        |021b
1a
1b
    11a
   |11b
    11c

    12a  |
   |12b  |
2a
    21a
    21b
   |
   |

")
        .unwrap();

    let r = vec![Token {
                     lines: vec![SLine::from("0")],
                     tokens: vec![Token {
                                      lines: vec![SLine::from(" 01a"),
                                                  SLine::from("01b"),
                                                  SLine::from("01c")],
                                      tokens: vec![],
                                  },
                                  Token {
                                      lines: vec![SLine::from("02a"), SLine::from("02b")],
                                      tokens: vec![Token {
                                                       lines: vec![SLine::from("020a"),
                                                                   SLine::from("020b")],
                                                       tokens: vec![],
                                                   },
                                                   Token {
                                                       lines: vec![SLine::from("  021a"),
                                                                   SLine::from("021b")],
                                                       tokens: vec![],
                                                   }],
                                  }],
                 },
                 Token {
                     lines: vec![SLine::from("1a"), SLine::from("1b")],
                     tokens: vec![Token {
                                      lines: vec![SLine::from("11a"),
                                                  SLine::from("11b"),
                                                  SLine::from("11c")],
                                      tokens: vec![],
                                  },
                                  Token {
                                      lines: vec![SLine::from("12a  "), SLine::from("12b  ")],
                                      tokens: vec![],
                                  }],
                 },
                 Token {
                     lines: vec![SLine::from("2a")],
                     tokens: vec![Token {
                                      lines: vec![SLine::from("21a"),
                                                  SLine::from("21b"),
                                                  SLine::from(""),
                                                  SLine::from("")],
                                      tokens: vec![],
                                  }],
                 }];
    assert!(tokens == r);
}



#[test]
fn delimiters_start_end() {
    let tokens = tokenize("
0
    || 01a
     01b
     01c

     02a
     02b

        |020a
        ||020b

        |  021a
        |021b
1a
1b
    11a
   ||11b
    11c

    12a  ||
   |12b  ||
2a
    21a
    21b
   |
   |

")
        .unwrap();

    let r = vec![Token {
                     lines: vec![SLine::from("0")],
                     tokens: vec![Token {
                                      lines: vec![SLine::from("| 01a"),
                                                  SLine::from("01b"),
                                                  SLine::from("01c")],
                                      tokens: vec![],
                                  },
                                  Token {
                                      lines: vec![SLine::from("02a"), SLine::from("02b")],
                                      tokens: vec![Token {
                                                       lines: vec![SLine::from("020a"),
                                                                   SLine::from("|020b")],
                                                       tokens: vec![],
                                                   },
                                                   Token {
                                                       lines: vec![SLine::from("  021a"),
                                                                   SLine::from("021b")],
                                                       tokens: vec![],
                                                   }],
                                  }],
                 },
                 Token {
                     lines: vec![SLine::from("1a"), SLine::from("1b")],
                     tokens: vec![Token {
                                      lines: vec![SLine::from("11a"),
                                                  SLine::from("|11b"),
                                                  SLine::from("11c")],
                                      tokens: vec![],
                                  },
                                  Token {
                                      lines: vec![SLine::from("12a  |"), SLine::from("12b  |")],
                                      tokens: vec![],
                                  }],
                 },
                 Token {
                     lines: vec![SLine::from("2a")],
                     tokens: vec![Token {
                                      lines: vec![SLine::from("21a"),
                                                  SLine::from("21b"),
                                                  SLine::from(""),
                                                  SLine::from("")],
                                      tokens: vec![],
                                  }],
                 }];
    assert!(tokens == r);
}


#[test]
fn invalid_indentation() {
    let error = tokenize("
aaa
    bbb
   ccc 
")
        .unwrap_err();
    assert!(error ==
            Error {
        line: LineNum(4),
        desc: "invalid indentation".to_owned(),
    });


    let error = tokenize("
aaa
    bbb
        cccc
            dddd
     eeee
    ffff
gggg
")
        .unwrap_err();

    assert!(error ==
            Error {
        line: LineNum(6),
        desc: "invalid indentation".to_owned(),
    });


    let error = tokenize("
aaa
    bbb
        cccc
    |eeee
")
        .unwrap_err();

    assert!(error ==
            Error {
        line: LineNum(5),
        desc: "invalid indentation".to_owned(),
    });

}
