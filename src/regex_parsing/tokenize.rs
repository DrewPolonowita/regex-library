#[derive(Clone, Debug)]
pub enum Token {
    Atom(char),
    Add,
    Or,
    Star,
    LParen,
    RParen
}

pub struct Lexer {
    tokens: Vec<Token>,
    last_token: Option<Token>,
    peeked: Option<Token>
}

impl<'a> Lexer {
    pub fn new(input: &str) -> Lexer {
        let mut tokens = input
            .chars()
            .map(|c| match c {
                '|' => Token::Or,
                '*' => Token::Star,
                '(' => Token::LParen,
                ')' => Token::RParen,
                c => Token::Atom(c)
            })
            .collect::<Vec<_>>();
        tokens.reverse();
        Lexer {
            tokens: tokens,
            last_token: None,
            peeked: None
        }
    }

    pub fn peek(&mut self) -> Option<Token> {
        if self.peeked.is_none() {
            self.peeked = self.compute_next();
        }
        self.peeked.clone()
    }

    fn compute_next(&mut self) -> Option<Token> {
        match self.last_token {
            Some(Token::Atom(_)) | Some(Token::Star) | Some(Token::RParen) => {
                match self.tokens.last() {
                    Some(Token::Atom(_)) | Some(Token::LParen) => {
                        return Some(Token::Add);
                    }
                    _ => {}
                }
            }
            _ => {}
        }

        self.tokens.pop()
    }
}

impl Iterator for Lexer {

    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(tok) = self.peeked.take() {
            self.last_token = Some(tok.clone());
            return Some(tok);
        }

        let tok = self.compute_next();
        if let Some(ref t) = tok {
            self.last_token = Some(t.clone());
        }
        tok
    }
}