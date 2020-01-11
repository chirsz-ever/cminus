use pest::iterators::Pair;
use pest::Parser;
use pest_derive::*;
use std::fmt;

#[derive(Clone, Debug)]
pub enum Token<'input> {
    SEMI,
    COMMA,
    ASSIGNOP,
    RELOP(&'input str),
    PLUS,
    MINUS,
    STAR,
    DIV,
    AND,
    OR,
    DOT,
    NOT,
    TYPE(&'input str),
    LP,
    RP,
    LB,
    RB,
    LC,
    RC,
    STRUCT,
    RETURN,
    IF,
    ELSE,
    WHILE,
    ID(&'input str),
    INT(i32),
    FLOAT(f32),
    EOI,
    UnknownChar(char),
    IllegalOct(&'input str),
    IllegalHex(&'input str),
}

impl Token<'_> {
    fn keyword_from_rule(r: Rule) -> Token<'static> {
        match r {
            Rule::SEMI => Token::SEMI,
            Rule::COMMA => Token::COMMA,
            Rule::ASSIGNOP => Token::ASSIGNOP,
            Rule::PLUS => Token::PLUS,
            Rule::MINUS => Token::MINUS,
            Rule::STAR => Token::STAR,
            Rule::DIV => Token::DIV,
            Rule::AND => Token::AND,
            Rule::OR => Token::OR,
            Rule::DOT => Token::DOT,
            Rule::NOT => Token::NOT,
            Rule::LP => Token::LP,
            Rule::RP => Token::RP,
            Rule::LB => Token::LB,
            Rule::RB => Token::RB,
            Rule::LC => Token::LC,
            Rule::RC => Token::RC,
            Rule::STRUCT => Token::STRUCT,
            Rule::RETURN => Token::RETURN,
            Rule::IF => Token::IF,
            Rule::ELSE => Token::ELSE,
            Rule::WHILE => Token::WHILE,
            r => panic!("{:?} is not a keyword", r),
        }
    }
}

impl fmt::Display for Token<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self {
            Token::SEMI => write!(f, ";")?,
            Token::COMMA => write!(f, ",")?,
            Token::ASSIGNOP => write!(f, "=")?,
            Token::PLUS => write!(f, "+")?,
            Token::MINUS => write!(f, "-")?,
            Token::STAR => write!(f, "*")?,
            Token::DIV => write!(f, "/")?,
            Token::AND => write!(f, "&&")?,
            Token::OR => write!(f, "||")?,
            Token::DOT => write!(f, ".")?,
            Token::NOT => write!(f, "!")?,
            Token::LP => write!(f, "(")?,
            Token::RP => write!(f, ")")?,
            Token::LB => write!(f, "[")?,
            Token::RB => write!(f, "]")?,
            Token::LC => write!(f, "{{")?,
            Token::RC => write!(f, "}}")?,
            Token::STRUCT => write!(f, "struct")?,
            Token::RETURN => write!(f, "return")?,
            Token::IF => write!(f, "if")?,
            Token::ELSE => write!(f, "else")?,
            Token::WHILE => write!(f, "while")?,
            Token::RELOP(relop) => write!(f, "{}", relop)?,
            tok => write!(f, "{:?}", tok)?,
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Location {
    pub line: usize,
    pub column: usize,
}

impl Location {
    pub fn new(line: usize, column: usize) -> Location {
        Location { line, column }
    }
}

#[derive(Clone, Debug)]
pub enum LexicalError {}

#[derive(Parser)]
#[grammar = "token.pest"]
struct CMinusLexer;

fn span_to_loc(span: pest::Span<'_>) -> (Location, Location) {
    let (start, end) = span.split();
    let (line, column) = start.line_col();
    let start = Location::new(line, column);
    let (line, column) = end.line_col();
    let end = Location::new(line, column);
    (start, end)
}

pub struct Lexer<'input> {
    inner: pest::iterators::Pairs<'input, Rule>,
}

impl Lexer<'_> {
    pub fn parse(source: &str) -> Lexer {
        Lexer {
            inner: CMinusLexer::parse(Rule::TokenList, &source).expect("parse error"),
        }
    }
}

impl<'input> Iterator for Lexer<'input> {
    type Item = Result<(Location, Token<'input>, Location), LexicalError>;

    fn next(&mut self) -> Option<Self::Item> {
        let pair = self.inner.next();
        pair.map(|p| {
            let (start, end) = span_to_loc(p.as_span());
            let tok = match p.as_rule() {
                Rule::RELOP => Token::RELOP(p.as_str()),
                Rule::TYPE => Token::TYPE(p.as_str()),
                Rule::ID => Token::ID(p.as_str()),
                Rule::INT => {
                    let inner = p.into_inner().next().unwrap();
                    get_int(inner)
                }
                Rule::FLOAT => Token::FLOAT(p.as_str().parse::<f32>().unwrap()),
                Rule::UnknownChar => {
                    let c = p.as_str().chars().next().unwrap();
                    Token::UnknownChar(c)
                }
                Rule::EndOfInput => Token::EOI,
                r => Token::keyword_from_rule(r),
            };
            Ok((start, tok, end))
        })
    }
}

fn get_int(pair: Pair<Rule>) -> Token {
    let raw = pair.as_str();
    match pair.as_rule() {
        Rule::HEX_LITERAL => i32::from_str_radix(&raw[2..], 16)
            .map(Token::INT)
            .unwrap_or_else(|_| Token::IllegalHex(raw)),
        Rule::OCT_LITERAL => i32::from_str_radix(&raw[1..], 8)
            .map(Token::INT)
            .unwrap_or_else(|_| Token::IllegalOct(raw)),
        Rule::DEC_LITERAL => Token::INT(i32::from_str_radix(&raw[..], 10).unwrap()),
        _ => unreachable!(),
    }
}
