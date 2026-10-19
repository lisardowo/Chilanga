use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufReader, Read};

pub struct dfa {
    pub currentState: String, //TODO debug shi, shouldnt be pub ??
    initialState: String,
    acceptStates: HashSet<String>,
    debug: Vec<String>,
    transitions: HashMap<(String, char), String>,
}

impl dfa {
    pub fn new(initial: &str) -> Self {
        const KEYWORDS: [&str; 3] = ["for", "if", "else"];
        dfa {
            currentState: initial.to_string(),
            initialState: initial.to_string(),
            acceptStates: KEYWORDS.iter().map(|s| s.to_string()).collect(),
            debug: Vec::new(),
            transitions: HashMap::new(),
        }
    }

    pub fn process(&mut self) -> Vec<String> {
        let file = File::open("wordlist.txt").expect("Failed to open file");
        let reader = BufReader::new(file);
        let mut chars = reader.bytes();

        let mut foundTokens = Vec::new();
        self.currentState.clear();

        loop {
            match chars.next() {
                Some(Ok(c)) => {
                    let ch = c as char;

                    if ch.is_whitespace() || "(){};<>+-=".contains(ch) {
                        if !self.currentState.is_empty() {
                            self.evaluate_current_state(&mut foundTokens);
                        }

                        if !ch.is_whitespace() {
                            println!("Recognized Symbol: {}", ch);
                            foundTokens.push(ch.to_string());
                        }

                        continue;
                    }

                    self.currentState.push(ch);
                    //TODO check for puntuaction
                }
                Some(Err(e)) => {
                    println!("[DEBUG] Error: {}", e);
                }
                None => {
                    // End of file: process whatever is left in the buffer
                    if !self.currentState.is_empty() {
                        self.evaluate_current_state(&mut foundTokens);
                    }
                    println!("Reader empty");
                    break;
                }
            }
        }

        foundTokens
    }

    // Helper function to decide what the accumulated string actually is
    fn evaluate_current_state(&mut self, foundTokens: &mut Vec<String>) {
        if self.acceptStates.contains(&self.currentState) {
            // It's a recognized keyword
            println!("Recognized Keyword: {}", self.currentState);
            foundTokens.push(self.currentState.clone());
            self.debug.push(self.currentState.clone());
        } else {
            // It's NOT in the wordlist, but it's still a word the developer wrote

            println!("Recognized Identifier/Literal: {}", self.currentState);
            self.debug
                .push(format!("IDENTIFIER: {}", self.currentState));
        }
        self.currentState.clear(); // Cleaning the buffer 
    }
    /*
    * Chilanga -> agregar "queti", para decirle al compilador que salte un fragmento de colido (debug)
    * fn cleanBuffer(self, bool hasMatch){

        if hasMatch == true{

        self.currentState = ""

        }

    } */
    fn add(self) {}
}

