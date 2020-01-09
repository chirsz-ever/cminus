use crate::lexer::{Location, Token};
use std::string::ToString;

#[derive(Clone)]
pub struct ASTNode {
    pub name: String,
    pub location: Location,
    pub children: Option<Vec<ASTNode>>,
    pub token: Option<Token>,
}

impl ASTNode {
    pub fn empty() -> ASTNode {
        ASTNode {
            name: String::new(),
            location: Location::default(),
            children: None,
            token: None,
        }
    }

    pub fn node(name: &str, children: Vec<ASTNode>) -> ASTNode {
        ASTNode {
            name: name.to_string(),
            location: children[0].location.clone(),
            children: Some(children),
            token: None,
        }
    }

    pub fn keyword(token: Token, location: Location) -> ASTNode {
        let name = format!("{:?}", token);
        ASTNode {
            name,
            location,
            children: None,
            token: Some(token),
        }
    }

    pub fn ident(ident: String, location: Location) -> ASTNode {
        ASTNode {
            name: "ID".to_string(),
            location,
            children: None,
            token: Some(Token::ID(ident)),
        }
    }

    pub fn type_name(type_name: String, location: Location) -> ASTNode {
        ASTNode {
            name: "TYPE".to_string(),
            location,
            children: None,
            token: Some(Token::TYPE(type_name)),
        }
    }

    pub fn int(value: i32, location: Location) -> ASTNode {
        ASTNode {
            name: "INT".to_string(),
            location,
            children: None,
            token: Some(Token::INT(value)),
        }
    }

    pub fn float(value: f32, location: Location) -> ASTNode {
        ASTNode {
            name: "FLOAT".to_string(),
            location,
            children: None,
            token: Some(Token::FLOAT(value)),
        }
    }

    pub fn relop(relop: String, location: Location) -> ASTNode {
        ASTNode {
            name: "RELOP".to_string(),
            location,
            children: None,
            token: Some(Token::RELOP(relop)),
        }
    }
}
