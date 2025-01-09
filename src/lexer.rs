use crate::token::{lookup_ident, Token, TokenKind};

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
    pub fn next_token(&mut self) -> Token {
        // Skip the whitespace
        self.skip_whitespace();
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
            _ => {
                return if Lexer::is_letter(self.ch) {
                    let literal = self.read_identifier();
                    let kind = lookup_ident(literal.clone());
                    Token { kind, literal }
                } else if Lexer::is_digit(self.ch) {
                    let literal = self.read_number();
                    Token { kind: TokenKind::Int, literal }
                } else {
                    Lexer::new_token(TokenKind::Illegal, self.ch)
                }
            }
        };
        // Read the next character
        self.read_char();
        // Return the token
        token
    }


    // Skip the whitespace characters
    fn skip_whitespace(&mut self) {
        while self.ch.is_ascii_whitespace() {
            self.read_char();
        }
    }

    // Check if the character is a letter
    fn is_letter(ch: char) -> bool {
        ch.is_alphabetic() || ch == '_'
    }

    // Read the identifier from the input
    fn read_identifier(&mut self) -> String {
        // Create a new identifier string
        let mut identifier = String::new();
        // Read the identifier until the character is not a letter
        while Lexer::is_letter(self.ch) {
            identifier.push(self.ch);
            self.read_char();
        }
        identifier
    }

    // Check if the character is a digit
    fn is_digit(ch: char) -> bool {
        ch.is_numeric()
    }

    // Read the number from the input 
    fn read_number(&mut self) -> String {
        // Create a new number string
        let mut number = String::from("");
        // Read the number until the character is not a digit
        while Lexer::is_digit(self.ch) {
            number.push(self.ch);
            self.read_char();
        }
        number
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
        // Test input string with multi-line string syntax
        let input = r#"
        let five = 5;
        let ten = 10;
        
        let add = fn(x, y) {
            x + y;
        };
        
        let result = add(five, ten);
        "#;

        // Expected tokens and their corresponding literals
        let expected_tests = vec![
            (TokenKind::Let, "let".to_string()),
            (TokenKind::Ident, "five".to_string()),
            (TokenKind::Assign, "=".to_string()),
            (TokenKind::Int, "5".to_string()),
            (TokenKind::Semicolon, ";".to_string()),
            (TokenKind::Let, "let".to_string()),
            (TokenKind::Ident, "ten".to_string()),
            (TokenKind::Assign, "=".to_string()),
            (TokenKind::Int, "10".to_string()),
            (TokenKind::Semicolon, ";".to_string()),
            (TokenKind::Let, "let".to_string()),
            (TokenKind::Ident, "add".to_string()),
            (TokenKind::Assign, "=".to_string()),
            (TokenKind::Function, "fn".to_string()),
            (TokenKind::LeftParen, "(".to_string()),
            (TokenKind::Ident, "x".to_string()),
            (TokenKind::Comma, ",".to_string()),
            (TokenKind::Ident, "y".to_string()),
            (TokenKind::RightParen, ")".to_string()),
            (TokenKind::LeftBrace, "{".to_string()),
            (TokenKind::Ident, "x".to_string()),
            (TokenKind::Plus, "+".to_string()),
            (TokenKind::Ident, "y".to_string()),
            (TokenKind::Semicolon, ";".to_string()),
            (TokenKind::RightBrace, "}".to_string()),
            (TokenKind::Semicolon, ";".to_string()),
            (TokenKind::Let, "let".to_string()),
            (TokenKind::Ident, "result".to_string()),
            (TokenKind::Assign, "=".to_string()),
            (TokenKind::Ident, "add".to_string()),
            (TokenKind::LeftParen, "(".to_string()),
            (TokenKind::Ident, "five".to_string()),
            (TokenKind::Comma, ",".to_string()),
            (TokenKind::Ident, "ten".to_string()),
            (TokenKind::RightParen, ")".to_string()),
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
