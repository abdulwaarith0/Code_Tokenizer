use crate::token::Token;

// Node is the base trait for all nodes in the AST
trait Node {
    fn token_literal(&self) -> String;
    fn print_string(&self) -> String;
}

// StatementNode is the base trait for all statement nodes in the AST
enum StatementNode {
    Let(LetStatement),
}

impl Node for StatementNode {
    // token_literal returns the token literal of the statement
    fn token_literal(&self) -> String {
        return match self {
            Self::Let(let_statement) => let_statement.token_literal(),
        };
    }

    // print_string returns the string representation of the statement
    fn print_string(&self) -> String {
        return match self {
            Self::Let(let_statement) => let_statement.print_string(),
        };
    }
}

// ExpressionNode is the base trait for all expression nodes in the AST
enum ExpressionNode {
    IdentifierNode(Identifier),
}

impl Node for ExpressionNode {
    // token_literal returns the token literal of the expression
    fn token_literal(&self) -> String {
        return match self {
            Self::IdentifierNode(identifier) => identifier.token_literal(),
        };
    }

    // print_string returns the string representation of the expression
    fn print_string(&self) -> String {
        return match self {
            Self::IdentifierNode(identifier) => identifier.print_string(),
        };
    }
}

// Program is the root node of the AST
struct Program {
    statements: Vec<StatementNode>,
}

// Program implements the Node trait for the Program node
impl Node for Program {
    // token_literal returns the token literal of the program
    fn token_literal(&self) -> String {
        return if self.statements.len() > 0 {
            match &self.statements[0] {
                StatementNode::Let(let_statement) => let_statement.token_literal(),
            }
        } else {
            String::from("")
        };
    }

    fn print_string(&self) -> String {
        let mut output = String::from("");
        for statement in self.statements.iter() {
            output.push_str(&statement.print_string());
        }
        output
    }
}

// Identifier is a node that represents an identifier
struct Identifier {
    token: Token,
    value: String,
}

// Identifier implements the Node trait for the Identifier node
impl Node for Identifier {
    // token_literal returns the value of the identifier
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }

    // print_string returns the string representation of the identifier
    fn print_string(&self) -> String {
        self.value.clone()
    }
}

// LetStatement is a node that represents a let statement
struct LetStatement {
    token: Token,
    name: Identifier,
    value: Option<ExpressionNode>,
}

// LetStatement implements the Node trait for the LetStatement node
impl Node for LetStatement {
    // token_literal returns the token literal of the let statement
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }

    // print_string returns the string representation of the let statement
    fn print_string(&self) -> String {
        let mut output = String::from("");
        // Add the token literal to the output
        output.push_str(&self.token_literal());
        output.push_str(" ");
        // Add the name of the identifier to the output
        output.push_str(&self.name.print_string());
        output.push_str(" = ");
        // Add the value of the expression to the output if it exists
        if let Some(value) = &self.value {
            output.push_str(&value.print_string());
        }
        // Add the semicolon to the output
        output.push_str(";");

        output
    }
}
