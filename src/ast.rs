use crate::lexer::{Location, Token};

#[derive(Clone)]
pub struct ASTNode<'input> {
    pub ident: ASTNodeIdent<'input>,
    pub location: Location,
    pub children: Option<Vec<ASTNode<'input>>>,
}

#[derive(Clone)]
pub enum ASTNodeIdent<'input> {
    Name(&'static str),
    Token(Token<'input>),
}

impl<'input> ASTNode<'input> {
    pub fn empty() -> ASTNode<'static> {
        ASTNode {
            ident: ASTNodeIdent::Name(""),
            location: Location::default(),
            children: None,
        }
    }

    pub fn node(name: &'static str, children: Vec<ASTNode<'input>>) -> ASTNode<'input> {
        ASTNode {
            ident: ASTNodeIdent::Name(name),
            location: children
                .get(0)
                .map(|n| n.location.clone())
                .unwrap_or_default(),
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

    pub fn ident(ident: &'input str, location: Location) -> ASTNode {
        ASTNode {
            ident: ASTNodeIdent::Token(Token::ID(ident)),
            location,
            children: None,
        }
    }

    pub fn type_name(type_name: String, location: Location) -> ASTNode<'input> {
        ASTNode {
            ident: ASTNodeIdent::Token(Token::TYPE(type_name)),
            location,
            children: None,
        }
    }

    pub fn int(value: i32, location: Location) -> ASTNode<'input> {
        ASTNode {
            ident: ASTNodeIdent::Token(Token::INT(value)),
            location,
            children: None,
        }
    }

    pub fn float(value: f32, location: Location) -> ASTNode<'input> {
        ASTNode {
            ident: ASTNodeIdent::Token(Token::FLOAT(value)),
            location,
            children: None,
        }
    }

    pub fn relop(relop: String, location: Location) -> ASTNode<'input> {
        ASTNode {
            ident: ASTNodeIdent::Token(Token::RELOP(relop)),
            location,
            children: None,
        }
    }
}
