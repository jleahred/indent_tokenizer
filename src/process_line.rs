const INDENT_CHAR: char = '|';
const EOL_CHAR: char = '|';


use NSpaces;
use SLine;



impl NSpaces {
    fn inc(&mut self) -> &Self {
        self.0 += 1;
        self
    }
}


#[derive(Eq, PartialEq, Debug)]
pub struct LineInfo {
    pub indent: NSpaces,
    pub content: SLine,
}



pub fn process_line(line: &SLine) -> Option<LineInfo> {
    let mut indent = NSpaces(0);
    let mut lresult = SLine::new();
    let mut located_start_indent = false;
    for c in line.0.chars() {
        if located_start_indent == false {
            if c == ' ' {
                indent.inc();
            } else if c == INDENT_CHAR {
                indent.inc();
                located_start_indent = true;
            } else {
                located_start_indent = true;
                lresult.0.push(c);
            }
        } else {
            lresult.0.push(c);
        }
    }
    if lresult.0.is_empty() && !located_start_indent {
        None
    } else {
        if lresult.0.chars().last() == Some(EOL_CHAR) {
            lresult.0.pop();
        }

        Some(LineInfo {
            indent: indent,
            content: lresult,
        })
    }
}

#[test]
fn test_process_line() {
    //  simple -------------------------------------------
    assert!(process_line(&SLine("    abcdef".to_owned())) ==
            Some(LineInfo {
        indent: NSpaces(4),
        content: SLine::from("abcdef"),
    }));

    assert!(process_line(&SLine::from(" abcdef")) ==
            Some(LineInfo {
        indent: NSpaces(1),
        content: SLine::from("abcdef"),
    }));

    //  empty line --------------------------------------
    assert!(process_line(&SLine::from("")) == None);

    assert!(process_line(&SLine::from("  ")) == None);


    //  not indented-------------------------------------
    assert!(process_line(&SLine::from("abcdef")) ==
            Some(LineInfo {
        indent: NSpaces(0),
        content: SLine::from("abcdef"),
    }));

    //  indentation indicator ---------------------------
    assert!(process_line(&SLine::from("   |abcdef")) ==
            Some(LineInfo {
        indent: NSpaces(4),
        content: SLine::from("abcdef"),
    }));

    assert!(process_line(&SLine::from("|abcdef")) ==
            Some(LineInfo {
        indent: NSpaces(1),
        content: SLine::from("abcdef"),
    }));

    assert!(process_line(&SLine::from("   |")) ==
            Some(LineInfo {
        indent: NSpaces(4),
        content: SLine::from(""),
    }));

    assert!(process_line(&SLine::from("|abcdef")) ==
            Some(LineInfo {
        indent: NSpaces(1),
        content: SLine::from("abcdef"),
    }));

    //  NSpaces at end of line ---------------------------
    assert!(process_line(&SLine::from("    abcdef  ")) ==
            Some(LineInfo {
        indent: NSpaces(4),
        content: SLine::from("abcdef  "),
    }));

    assert!(process_line(&SLine::from("    abcdef  |")) ==
            Some(LineInfo {
        indent: NSpaces(4),
        content: SLine::from("abcdef  "),
    }));

    assert!(process_line(&SLine::from("   |  |")) ==
            Some(LineInfo {
        indent: NSpaces(4),
        content: SLine::from("  "),
    }));

    //  pipe end of line ---------------------------
    assert!(process_line(&SLine::from("    abcdef  ||")) ==
            Some(LineInfo {
        indent: NSpaces(4),
        content: SLine::from("abcdef  |"),
    }));

}
