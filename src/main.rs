#![allow(dead_code)]
use lalrpop_util::lalrpop_mod;

mod ast;
mod lexer;
lalrpop_mod!(parser);

use lexer::*;
use parser::ProgramParser;
use std::env;
use std::fs;

fn main() {
    let fname = env::args().nth(1).expect("please input source file name");
    let unparsed_src = fs::read_to_string(fname).expect("cannot read file");
    let tokens = Lexer::parse(&unparsed_src);
    let ast = ProgramParser::new().parse(tokens).unwrap();
    print_ast(&ast, 0);
}

fn print_ast(ast: &ast::ASTNode, indent: usize) {
    if ast.name.is_empty() {
        return;
    }

    let next_indent = indent + 2;
    let line_num = ast.location.line;

    match &ast.token {
        Some(token) => {
            print_indent(indent);
            match token {
                Token::ID(id) => {
                    println!("ID: {}", id);
                }
                Token::TYPE(type_name) => {
                    println!("TYPE: {}", type_name);
                }
                Token::INT(val) => {
                    println!("INT: {}", val);
                }
                Token::FLOAT(val) => {
                    println!("FLOAT: {:.6}", val);
                }
                Token::RELOP(_) => {
                    println!("RELOP");
                }
                token => {
                    println!("{:?}", token);
                }
            }
        }
        None => {
            let children = ast.children.as_ref().unwrap();
            if children.len() == 1 && &ast.name == "Exp" && &children[0].name == "Exp" {
                print_ast(&children[0], indent);
            } else {
                print_indent(indent);
                println!("{} ({})", ast.name, line_num);
                for pat in children {
                    print_ast(pat, next_indent);
                }
            }
        }
    }
}

#[inline]
fn print_indent(indent: usize) {
    for _ in 0..indent {
        print!(" ");
    }
}
