const INDENT_CHAR: char = '|';
const EOL_CHAR: char = '|';


use Spaces;

impl Spaces {
    fn inc(&mut self) -> &Self {
        self.0 += 1;
        self
    }
}


#[derive(Eq, PartialEq, Debug)]
pub struct LineInfo {
    pub indent: Spaces,
    pub content: String,
}



pub fn process_line(line: &str) -> Option<LineInfo> {
    let mut indent = Spaces(0);
    let mut lresult = String::new();
    let mut located_start_indent = false;
    for c in line.chars() {
        if located_start_indent == false {
            if c == ' ' {
                indent.inc();
            } else if c == INDENT_CHAR {
                indent.inc();
                located_start_indent = true;
            } else {
                located_start_indent = true;
                lresult.push(c);
            }
        } else {
            lresult.push(c);
        }
    }
    if lresult.is_empty() && !located_start_indent {
        None
    } else {
        if lresult.chars().last() == Some(EOL_CHAR) {
            lresult.pop();
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
    assert!(process_line(&"    abcdef".to_string()) ==
            Some(LineInfo {
        indent: Spaces(4),
        content: "abcdef".to_string(),
    }));

    assert!(process_line(&" abcdef".to_string()) ==
            Some(LineInfo {
        indent: Spaces(1),
        content: "abcdef".to_string(),
    }));

    //  empty line --------------------------------------
    assert!(process_line(&"".to_string()) == None);

    assert!(process_line(&"  ".to_string()) == None);


    //  not indented-------------------------------------
    assert!(process_line(&"abcdef".to_string()) ==
            Some(LineInfo {
        indent: Spaces(0),
        content: "abcdef".to_string(),
    }));

    //  indentation indicator ---------------------------
    assert!(process_line(&"   |abcdef".to_string()) ==
            Some(LineInfo {
        indent: Spaces(4),
        content: "abcdef".to_string(),
    }));

    assert!(process_line(&"|abcdef".to_string()) ==
            Some(LineInfo {
        indent: Spaces(1),
        content: "abcdef".to_string(),
    }));

    assert!(process_line(&"   |".to_string()) ==
            Some(LineInfo {
        indent: Spaces(4),
        content: "".to_string(),
    }));

    assert!(process_line(&"|abcdef".to_string()) ==
            Some(LineInfo {
        indent: Spaces(1),
        content: "abcdef".to_string(),
    }));

    //  spaces at end of line ---------------------------
    assert!(process_line(&"    abcdef  ".to_string()) ==
            Some(LineInfo {
        indent: Spaces(4),
        content: "abcdef  ".to_string(),
    }));

    assert!(process_line(&"    abcdef  |".to_string()) ==
            Some(LineInfo {
        indent: Spaces(4),
        content: "abcdef  ".to_string(),
    }));

    assert!(process_line(&"   |  |".to_string()) ==
            Some(LineInfo {
        indent: Spaces(4),
        content: "  ".to_string(),
    }));

    //  pipe end of line ---------------------------
    assert!(process_line(&"    abcdef  ||".to_string()) ==
            Some(LineInfo {
        indent: Spaces(4),
        content: "abcdef  |".to_string(),
    }));

}
