mod lexer;

use lexer::dfa;

fn main() {
    println!(
        "Using hardocded test file, for production change process ln 26 for a way to get user input from binary call"
    );
    let mut newlexer = dfa::new("");
    newlexer.process();
    println!("{}", newlexer.currentState)
}
