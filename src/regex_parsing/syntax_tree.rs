use crate::regex_parsing::tokenize::{Lexer, Token};
use crate::automata::nfa_builder::NfaBuilder;

#[derive(Debug)]
pub enum Ast {
    Atom(char),
    Concat(Box<Ast>, Box<Ast>),
    Or(Box<Ast>, Box<Ast>),
    Star(Box<Ast>)
}



pub fn parse_tree(lexer: &mut Lexer) -> Ast {
    parse_expr(lexer)
}

fn parse_expr(lexer: &mut Lexer) -> Ast {
    let mut node = parse_term(lexer);

    while matches!(lexer.peek(), Some(Token::Or)) {
        lexer.next();
        let right = parse_term(lexer);
        node = Ast::Or(Box::new(node), Box::new(right));
    };
    node
}

fn parse_term(lexer: &mut Lexer) -> Ast {
    let mut node = parse_factor(lexer);

    while matches!(lexer.peek(), Some(Token::Add)) {
        lexer.next();
        let right = parse_factor(lexer);
        node = Ast::Concat(Box::new(node), Box::new(right));
    };
    node
}

fn parse_factor(lexer: &mut Lexer) -> Ast {
    let mut node = parse_base(lexer);

    while matches!(lexer.peek(), Some(Token::Star)) {
        lexer.next();
        node = Ast::Star(Box::new(node));
    };

    node
}

fn parse_base(lexer: &mut Lexer) -> Ast {
    match lexer.next() {
        Some(s) => {
            match s {
                Token::Atom(c) => Ast::Atom(c),
                Token::LParen => {
                    let node = parse_expr(lexer);
                    match lexer.next() {
                        Some(Token::RParen) => node,
                        _ => panic!("No RPAREN")
                    }
                }
                _ => panic!("No base")
            }
        },
        None => {
            panic!("No base")
        }
    }
}

pub fn parse_to_nfa(tree: &Ast) -> NfaBuilder {
    match tree {
        Ast::Atom(c) => NfaBuilder::new(&[*c]),
        Ast::Concat(ltree, rtree) => parse_to_nfa(ltree).add(parse_to_nfa(rtree)),
        Ast::Or(ltree, rtree) => parse_to_nfa(ltree).or(parse_to_nfa(rtree)),
        Ast::Star(tree)  => parse_to_nfa(tree).star()
    }
}