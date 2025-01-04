#[allow(dead_code)]

use crate::token::{Token, TokenKind};

// Lexer is a simple lexer that tokenizes the input string into tokens.
pub struct Lexer {
    input: Vec<char>,
    position: usize,
    read_position: usize,
    ch: char,
}

impl Lexer {
    // Create a new Lexer instance
    pub fn new(input: &str) -> Lexer {
        let mut lexer = Lexer {
            input: input.chars().collect(),
            position: 0,
            read_position: 0,
            ch: '\0',
        };
        // Read the first character
        lexer.read_char();
        // Return the lexer
        lexer
    }

    // Read the next character in the input
    fn read_char(&mut self) {
        // If we've reached the end of the input, set the character to the null character
        if self.read_position >= self.input.len() {
            self.ch = '\0';
        } else {
            self.ch = self.input[self.read_position];
        }
        // Update the position and read_position
        self.position = self.read_position;
        self.read_position += 1;
    }

    // Read the next token in the input
    fn next_token(&mut self) -> Token {
        // Create a new token with the current character
        let token = match self.ch {
            '=' => Lexer::new_token(TokenKind::Assign, self.ch),
            '+' => Lexer::new_token(TokenKind::Plus, self.ch),
            '(' => Lexer::new_token(TokenKind::LeftParen, self.ch),
            ')' => Lexer::new_token(TokenKind::RightParen, self.ch),
            '{' => Lexer::new_token(TokenKind::LeftBrace, self.ch),
            '}' => Lexer::new_token(TokenKind::RightBrace, self.ch),
            ',' => Lexer::new_token(TokenKind::Comma, self.ch),
            ';' => Lexer::new_token(TokenKind::Semicolon, self.ch),
            '\0' => Token {
                kind: TokenKind::Eof,
                literal: "".to_string(),
            },
            _ => Lexer::new_token(TokenKind::Illegal, self.ch),
        };
        // Read the next character
        self.read_char();
        // Return the token
        token
    }

    // Create a new token
    fn new_token(kind: TokenKind, ch: char) -> Token {
        // Create a new token with the given kind and character literal
        Token {
            kind,
            literal: ch.to_string(),
        }
    }
}

// Test cases for the Lexer
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    // Test the next_token method of the Lexer
    fn test_next_token() {
        // Test input string
        let input = "=+(){},;";

        // Expected tokens and their corresponding literals
        let expected_tests = vec![
            (TokenKind::Assign, "=".to_string()),
            (TokenKind::Plus, "+".to_string()),
            (TokenKind::LeftParen, "(".to_string()),
            (TokenKind::RightParen, ")".to_string()),
            (TokenKind::LeftBrace, "{".to_string()),
            (TokenKind::RightBrace, "}".to_string()),
            (TokenKind::Comma, ",".to_string()),
            (TokenKind::Semicolon, ";".to_string()),
            (TokenKind::Eof, "".to_string()),
        ];

        // Create a new Lexer instance with the test input
        let mut lexer = Lexer::new(input);

        // Iterate over the expected tokens and their corresponding literals
        for (index, exp_token) in expected_tests.iter().enumerate() {
            // Get the next token from the lexer
            let received_token = lexer.next_token();

            // Assert that the token kind and literal match the expected values
            assert_eq!(
                received_token.kind, exp_token.0,
                "tests[{index}] - tokentype wrong, expected={}, got={}",
                exp_token.0, received_token.kind
            );
            assert_eq!(
                received_token.literal, exp_token.1,
                "tests[{index}] - literal wrong, expected={}, got={}",
                exp_token.1, received_token.literal
            );
        }
    }
}
