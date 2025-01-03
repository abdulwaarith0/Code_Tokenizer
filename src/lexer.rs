use crate::token::{Token, TokenKind};

// Lexer is a simple lexer that tokenizes the input string into tokens.
pub struct Lexer {
    input: Vec<char>,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Self {
            input: input.chars().collect(),
        }
    }

    fn next_token(&self) -> Token {
        todo!();
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
        let lexer = Lexer::new(input);

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
