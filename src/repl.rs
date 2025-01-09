use crate::lexer::Lexer;
use crate::token::TokenKind;
use std::io::{Stdin, Stdout, Write};

// Start the REPL
pub fn start(stdin: Stdin, mut stdout: Stdout) {
    // Loop until the user quits
    loop {
        // Write the prompt string >> to the stdout
        write!(stdout, ">> ").expect("should have written prompt string >>");
        // Flush the stdout
        stdout.flush().expect("should have flushed stdout");

        // Read the user input
        let mut input = String::new();
        if let Err(e) = stdin.read_line(&mut input) {
            // Write the error message to the stdout
            writeln!(stdout, "Error: {}", e).expect("should have written error message");
            // Return from the function
            return;
        }

        // Create a new lexer with the user input
        let mut lexer = Lexer::new(&input);

        // Loop until the lexer reaches the end of the input
        loop {
            // Get the next token
            let tok = lexer.next_token();
            // If the token is the end of the input, break the loop
            if tok.kind == TokenKind::Eof {
                break;
            }
            // Write the token to the stdout
            writeln!(stdout, "{:?}", tok).expect("should have written token");
        }
    }
}
