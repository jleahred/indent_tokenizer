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
    assert!(tokens[0].lines[0] == "1111");
    assert!(tokens[1].lines[0] == "2222");
    assert!(tokens[2].lines[0] == "3333");



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
    assert!(tokens[0].lines[0] == "00");
    assert!(tokens[0].lines[1] == "01");
    assert!(tokens[0].lines[2] == "02");
    assert!(tokens[1].lines[0] == "10");
    assert!(tokens[1].lines[1] == "11");
    assert!(tokens[1].lines[2] == "12");
    assert!(tokens[1].lines[3] == "13");
    assert!(tokens[2].lines[0] == "20");
    assert!(tokens[3].lines[0] == "30");
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
                     lines: vec!["0".to_owned()],
                     tokens: vec![Token {
                                      lines: vec!["01".to_owned(), "02".to_owned()],
                                      tokens: vec![Token {
                                                       lines: vec!["020".to_owned(),
                                                                   "021".to_owned(),
                                                                   "023".to_owned()],
                                                       tokens: vec![Token {
                                                                        lines:
                                                                            vec!["0230".to_owned(),
                                                                                 "0231".to_owned()],
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

    let r =
        vec![Token {
                 lines: vec!["0".to_owned()],
                 tokens: vec![Token {
                                  lines: vec!["01".to_owned(), "02".to_owned()],
                                  tokens: vec![Token {
                                                   lines: vec!["020".to_owned(),
                                                               "021".to_owned(),
                                                               "023".to_owned()],
                                                   tokens: vec![Token {
                                                                    lines: vec!["0230".to_owned(),
                                                                                "0231".to_owned()],
                                                                    tokens: vec![],
                                                                }],
                                               }],
                              },
                              Token {
                                  lines: vec!["03".to_owned(), "04".to_owned(), "05".to_owned()],
                                  tokens: vec![],
                              }],
             },
             Token {
                 lines: vec!["1".to_owned()],
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

    println!("{:?}", tokens);
    let r =
        vec![Token {
                 lines: vec!["0a".to_owned(), "0b".to_owned()],
                 tokens: vec![Token {
                                  lines: vec!["01a".to_owned(), "01b".to_owned()],
                                  tokens: vec![Token {
                                                   lines: vec!["010.a".to_owned(),
                                                               "010.b".to_owned(),
                                                               "010.c".to_owned()],
                                                   tokens: vec![Token {
                                                                    lines: vec!["010..a"
                                                                                    .to_owned(),
                                                                                "010..b"
                                                                                    .to_owned()],
                                                                    tokens: vec![],
                                                                }],
                                               }],
                              }],
             },
             Token {
                 lines: vec!["1a".to_owned(), "1b".to_owned(), "1c".to_owned()],
                 tokens: vec![Token {
                                  lines: vec!["1.a".to_owned(), "1.b".to_owned(), "1.c".to_owned()],
                                  tokens: vec![Token {
                                                   lines: vec!["1..a".to_owned(),
                                                               "1..b".to_owned()],
                                                   tokens: vec![],
                                               }],
                              }],
             },
             Token {
                 lines: vec!["2a".to_owned()],
                 tokens: vec![],
             },
             Token {
                 lines: vec!["3a".to_owned()],
                 tokens: vec![Token {
                                  lines: vec!["30.a".to_owned(), "30.b".to_owned()],
                                  tokens: vec![],
                              },
                              Token {
                                  lines: vec!["31.a".to_owned()],
                                  tokens: vec![],
                              },
                              Token {
                                  lines: vec!["32.a".to_owned()],
                                  tokens: vec![Token {
                                                   lines: vec!["32..a".to_owned()],
                                                   tokens: vec![Token {
                                                                    lines: vec!["320a".to_owned(),
                                                                                "320b".to_owned()],
                                                                    tokens: vec![],
                                                                },
                                                                Token {
                                                                    lines: vec!["321a".to_owned(),
                                                                                "321b".to_owned()],
                                                                    tokens: vec![],
                                                                }],
                                               }],
                              }],
             },
             Token {
                 lines: vec!["4a".to_owned(), "4b".to_owned()],
                 tokens: vec![],
             },
             Token {
                 lines: vec!["5a".to_owned()],
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

    println!("{:?}", tokens);
    let r =
        vec![Token {
                 lines: vec!["0".to_owned()],
                 tokens: vec![Token {
                                  lines: vec![" 01a".to_owned(),
                                              "01b".to_owned(),
                                              "01c".to_owned()],
                                  tokens: vec![],
                              },
                              Token {
                                  lines: vec!["02a".to_owned(), "02b".to_owned()],
                                  tokens: vec![Token {
                                                   lines: vec!["020a".to_owned(),
                                                               "020b".to_owned()],
                                                   tokens: vec![],
                                               },
                                               Token {
                                                   lines: vec!["  021a".to_owned(),
                                                               "021b".to_owned()],
                                                   tokens: vec![],
                                               }],
                              }],
             },
             Token {
                 lines: vec!["1a".to_owned(), "1b".to_owned()],
                 tokens: vec![Token {
                                  lines: vec!["11a".to_owned(), "11b".to_owned(), "11c".to_owned()],
                                  tokens: vec![],
                              },
                              Token {
                                  lines: vec!["12a  ".to_owned(), "12b  ".to_owned()],
                                  tokens: vec![],
                              }],
             },
             Token {
                 lines: vec!["2a".to_owned()],
                 tokens: vec![Token {
                                  lines: vec!["21a".to_owned(),
                                              "21b".to_owned(),
                                              "".to_owned(),
                                              "".to_owned()],
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

    println!("{:?}", tokens);
    let r = vec![Token {
                     lines: vec!["0".to_owned()],
                     tokens: vec![Token {
                                      lines: vec!["| 01a".to_owned(),
                                                  "01b".to_owned(),
                                                  "01c".to_owned()],
                                      tokens: vec![],
                                  },
                                  Token {
                                      lines: vec!["02a".to_owned(), "02b".to_owned()],
                                      tokens: vec![Token {
                                                       lines: vec!["020a".to_owned(),
                                                                   "|020b".to_owned()],
                                                       tokens: vec![],
                                                   },
                                                   Token {
                                                       lines: vec!["  021a".to_owned(),
                                                                   "021b".to_owned()],
                                                       tokens: vec![],
                                                   }],
                                  }],
                 },
                 Token {
                     lines: vec!["1a".to_owned(), "1b".to_owned()],
                     tokens: vec![Token {
                                      lines: vec!["11a".to_owned(),
                                                  "|11b".to_owned(),
                                                  "11c".to_owned()],
                                      tokens: vec![],
                                  },
                                  Token {
                                      lines: vec!["12a  |".to_owned(), "12b  |".to_owned()],
                                      tokens: vec![],
                                  }],
                 },
                 Token {
                     lines: vec!["2a".to_owned()],
                     tokens: vec![Token {
                                      lines: vec!["21a".to_owned(),
                                                  "21b".to_owned(),
                                                  "".to_owned(),
                                                  "".to_owned()],
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
        line: 4,
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
        line: 6,
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
        line: 5,
        desc: "invalid indentation".to_owned(),
    });

}
