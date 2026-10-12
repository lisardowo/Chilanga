use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{self, BufReader, Read};

struct lexer {
    currentState: String,
    initialState: String,
    acceptStates: HashSet<String>,
    debug: Vec<String>,
    transitions: HashMap<(String, char), String>,
}

impl lexer {
    fn new(initial: &str, accepts: Vec<&str>) -> Self {
        lexer {
            currentState: initial.to_string(),
            initialState: initial.to_string(),
            acceptStates: accepts.into_iter().map(String::from).collect(),
            debug: Vec::new(),
            transitions: HashMap::new(),
        }
    }

    fn process(&mut self) -> Vec<String> {
        let file = File::open("wordlist.txt").expect("Failed to open file"); //TODO replace to get the file from main
        let reader = BufReader::new(file);
        let mut chars = reader.bytes();

        loop {
            if let Some(nextByte) = chars.next() {
                match nextByte {
                    Ok(c) => self.currentState.push(c as char),
                    Err(e) => {
                        println! {"[DEBUG] Error: {}", e}
                    }
                }
            } else {
                println!("Reader empty");
                return vec![String::from("Debug")];
                //break;
            }
        }
    }

    fn add() {}
}

fn main() {
    let mut newlexer = lexer::new("for", vec!["for"]);
    newlexer.process();
    println!("{}", newlexer.currentState)
}
