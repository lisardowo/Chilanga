mod rewroteL;

use rewroteL::Lexer;

use std::fs;

fn main() -> std::io::Result<()> {
    let source = fs::read_to_string("sampleCode.c")?;

    for token in Lexer::new().tokenize(&source) {
        println!(
            "{:>3}:{:<3} {:<12} {:?}",
            token.line, token.column, token.lexeme, token.token_type
        );
    }

    Ok(())
}
