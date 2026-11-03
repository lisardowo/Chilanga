mod lexer;

use lexer::dfa;

struct ast {}

fn main() {
    println!(
        "Using hardocded test file, for production change process ln 26 for a way to get user input from binary call"
    );
    let mut newlexer = dfa::new("");
    newlexer.process();
    for token in newlexer.debug.iter() {
        println!("lexer arrays {}", token);
    }
}
