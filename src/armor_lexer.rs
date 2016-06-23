use std::str::Chars;
use std::iter::Iterator;


#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum TokenType {
    Character,
    NewLine,
    WhiteSpace,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct ArmorToken {
    token_type: TokenType,
    token: char,
}

impl ArmorToken {
    pub fn new(token_type: TokenType, token: char) -> ArmorToken {
        ArmorToken {
            token_type: token_type,
            token: token,
        }
    }

    pub fn valid_token(&self) -> bool {
        match self.token {
            ' '  | '\t' => self.token_type == TokenType::WhiteSpace,
            '\n' | '\r' => self.token_type == TokenType::NewLine,
            _           => self.token_type == TokenType::Character, 
        }
    }

    pub fn token_type(&self) -> TokenType {
        self.token_type
    }
}

pub struct ArmorLexer<'a> {
    stream: String,
    chars:  Chars<'a>,
}

impl<'a> ArmorLexer<'a> {
    pub fn new(stream: &String) -> ArmorLexer {
        let new_stream = stream.clone();

        ArmorLexer {
            stream: new_stream,
            chars:  stream.chars(),
        }
    }

    pub fn next_token(&mut self) -> Option<ArmorToken> {
        let next_char = self.chars.next();

        match next_char {
            None                    => None,
            Some(' ')  | Some('\t') => Some(ArmorToken::new(TokenType::WhiteSpace, ' ')),
            Some('\n') | Some('\r') => Some(ArmorToken::new(TokenType::NewLine, '\n')),
            Some(character)         => Some(ArmorToken::new(TokenType::Character, character)), 
        } 
    }
}

impl<'a> Iterator for ArmorLexer<'a> {
    type Item = ArmorToken;

    fn next(&mut self) -> Option<ArmorToken> {
        self.next_token()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.chars.size_hint()
    }
}
