use pest::iterators::Pair;
use pest::Parser;
use pest_derive::*;

#[derive(Clone, Debug)]
pub enum Token {
    SEMI,
    COMMA,
    ASSIGNOP,
    RELOP(String),
    PLUS,
    MINUS,
    STAR,
    DIV,
    AND,
    OR,
    DOT,
    NOT,
    TYPE(String),
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
    ID(String),
    INT(i32),
    FLOAT(f32),
    EOI,
}

impl Token {
    fn keyword_from_rule(r: Rule) -> Token {
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
pub enum LexicalError {
    UnknownChar(char),
    InvalidInt,
    InvalidFloat,
}

#[derive(Parser)]
#[grammar = "token.pest"]
struct CMinusLexer;

fn span_to_loc<'i>(span: pest::Span<'i>) -> (Location, Location) {
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
    type Item = Result<(Location, Token, Location), LexicalError>;

    fn next(&mut self) -> Option<Self::Item> {
        let pair = self.inner.next();
        pair.map(|p| {
            let (start, end) = span_to_loc(p.as_span());
            let tok = match p.as_rule() {
                Rule::RELOP => Token::RELOP(p.as_str().to_owned()),
                Rule::TYPE => Token::TYPE(p.as_str().to_owned()),
                Rule::ID => Token::ID(p.as_str().to_owned()),
                Rule::INT => {
                    let inner = p.into_inner().next().unwrap();
                    Token::INT(to_int(inner))
                }
                Rule::FLOAT => Token::FLOAT(p.as_str().parse::<f32>().unwrap()),
                Rule::UnknownChar => {
                    let c = p.as_str().chars().next().unwrap();
                    return Err(LexicalError::UnknownChar(c));
                }
                Rule::EndOfInput => Token::EOI,
                r => Token::keyword_from_rule(r),
            };
            Ok((start, tok, end))
        })
    }
}

fn to_int(pair: Pair<Rule>) -> i32 {
    match pair.as_rule() {
        Rule::HEX_LITERAL => i32::from_str_radix(&pair.as_str()[2..], 16).unwrap(),
        Rule::OCT_LITERAL => i32::from_str_radix(&pair.as_str()[1..], 8).unwrap(),
        Rule::DEC_LITERAL => i32::from_str_radix(&pair.as_str()[..], 10).unwrap(),
        _ => unreachable!(),
    }
}
