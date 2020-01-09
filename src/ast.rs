use crate::lexer::{Location, Token};

#[derive(Clone)]
pub struct ASTNode {
    pub ident: ASTNodeIdent,
    pub location: Location,
    pub children: Option<Vec<ASTNode>>,
}

#[derive(Clone)]
pub enum ASTNodeIdent {
    Name(&'static str),
    Token(Token),
}

impl ASTNode {
    pub fn empty() -> ASTNode {
        ASTNode {
            ident: ASTNodeIdent::Name(""),
            location: Location::default(),
            children: None,
        }
    }

    pub fn node(name: &'static str, children: Vec<ASTNode>) -> ASTNode {
        ASTNode {
            ident: ASTNodeIdent::Name(name),
            location: children[0].location.clone(),
            children: Some(children),
        }
    }

    pub fn keyword(token: Token, location: Location) -> ASTNode {
        ASTNode {
            ident: ASTNodeIdent::Token(token),
            location,
            children: None,
        }
    }

    pub fn ident(ident: String, location: Location) -> ASTNode {
        ASTNode {
            ident: ASTNodeIdent::Token(Token::ID(ident)),
            location,
            children: None,
        }
    }

    pub fn type_name(type_name: String, location: Location) -> ASTNode {
        ASTNode {
            ident: ASTNodeIdent::Token(Token::TYPE(type_name)),
            location,
            children: None,
        }
    }

    pub fn int(value: i32, location: Location) -> ASTNode {
        ASTNode {
            ident: ASTNodeIdent::Token(Token::INT(value)),
            location,
            children: None,
        }
    }

    pub fn float(value: f32, location: Location) -> ASTNode {
        ASTNode {
            ident: ASTNodeIdent::Token(Token::FLOAT(value)),
            location,
            children: None,
        }
    }

    pub fn relop(relop: String, location: Location) -> ASTNode {
        ASTNode {
            ident: ASTNodeIdent::Token(Token::RELOP(relop)),
            location,
            children: None,
        }
    }
}
