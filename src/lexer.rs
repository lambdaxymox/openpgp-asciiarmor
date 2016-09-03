#![allow(dead_code)]
use std::iter::Iterator;
use std::collections::VecDeque;
use std::iter::Peekable;
use token::{Token, TokenType, Location};
use token;


pub struct Lexer<S>
    where S: Iterator<Item = char>
{
    input:         Peekable<S>,
    lookahead:     VecDeque<char>,
    location:      Location,
    offset:        usize
}

impl<S> Lexer<S>
    where S: Iterator<Item = char>
{
    pub fn new(input: S) -> Lexer<S> {
        let start = Location { absolute: 0 };

        Lexer {
            input:         input.peekable(),
            lookahead:     VecDeque::with_capacity(30),
            location:      start,
            offset:        0
        }
    }

    fn scan_one_of<F>(&mut self, choices: &[F], default: F) -> Token
        where F: Fn(&mut Lexer<S>) -> Option<Token>
    {
        for choice in choices {
            match choice(self) {
                Some(token) => {
                    return token;
                }
                None => continue
            }
        }

        default(self).unwrap()
    }

    fn scan_or_else<F, G>(&mut self, scanner: F, default: G) -> Token
        where F: Fn(&mut Lexer<S>) -> Option<Token>,
              G: Fn(&mut Lexer<S>) -> Option<Token>
    {
        match scanner(self) {
            Some(token) => token,
            None => default(self).unwrap()
        }
    }

    pub fn next_token(&mut self) -> Token {
        match self.peek_char() {
            Some('-') => self.scan_or_else(Lexer::scan_five_dashes, Lexer::scan_other_utf8),
            Some('=') => self.scan_pad_symbol().unwrap(),
            Some('/') => self.scan_forwardslash().unwrap(),
            Some(':') => self.scan_or_else(Lexer::scan_colon_space, Lexer::scan_colon),
            Some('+') => self.scan_plus_sign().unwrap(),
            Some(',') => self.scan_comma().unwrap(),
            Some(' ') => self.scan_whitespace_symbol().unwrap(),
            Some('B') => self.scan_or_else(Lexer::scan_begin, Lexer::scan_letter),
            Some('E') => self.scan_or_else(Lexer::scan_end, Lexer::scan_letter),
            Some('V') => self.scan_or_else(Lexer::scan_version, Lexer::scan_letter),
            Some('C') => {
                self.scan_one_of([Lexer::scan_comment,
                                  Lexer::scan_charset].as_ref(),
                                  Lexer::scan_letter)
            }
            Some('H') => self.scan_or_else(Lexer::scan_hash, Lexer::scan_letter),
            Some('P') => {
                self.scan_one_of([Lexer::scan_pgp_message_part,
                                  Lexer::scan_pgp_public_key_block,
                                  Lexer::scan_pgp_private_key_block,
                                  Lexer::scan_pgp_message,
                                  Lexer::scan_pgp_signature,
                                  ].as_ref(),
                                  Lexer::scan_letter)
            }
            Some('M')  => self.scan_or_else(Lexer::scan_messageid, Lexer::scan_letter),
            Some('\n') => self.scan_or_else(Lexer::scan_blankline, Lexer::scan_newline),
            Some('0'...'9') => self.scan_digit().unwrap(),
            Some('a'...'z') => self.scan_letter().unwrap(),
            Some('A'...'Z') => self.scan_letter().unwrap(),
            Some(_) => self.scan_other_utf8().unwrap(),
            None    => self.scan_eof().unwrap(),
        }
    }

    fn peek_char(&mut self) -> Option<char> {
        if self.lookahead.is_empty() {
            self.offset = 0;
            let next_ch = self.input.next();
            if next_ch.is_some() {
                //self.location.increment(1);
                self.lookahead.push_back(next_ch.unwrap());
                Some(next_ch.unwrap())
            } else {
                None
            }
        } else {
            self.sync();
            Some(self.lookahead[self.offset])
        }
    }

    fn sync(&mut self) {
        if self.offset > self.lookahead.len()-1 {
            let n = self.offset - (self.lookahead.len()-1);
            self.fill(n);
        }
    }

    fn fill(&mut self, amount: usize) {
        for _ in 0..amount {
            match self.input.next() {
                Some(ch) => {
                    self.lookahead.push_back(ch);
                }
                None => {
                    break;
                }
            }
        }
    }

    fn read_char(&mut self) -> Option<char> {
        match self.peek_char() {
            Some(ch) => {
                self.offset += 1;
                Some(ch)
            }
            None => None
        }
    }

    fn reset_offset(&mut self) {
        self.offset = 0;
    }

    fn consume(&mut self) {
        for _ in 0..self.offset {
            self.lookahead.pop_front();
        }
        self.location.increment(self.offset);
        self.reset_offset();
    }

    fn consume_char(&mut self) {
        if self.lookahead.is_empty() {
            self.reset_offset();
        } else {
            self.lookahead.pop_front();
            self.location.increment(1);
            if self.offset > 0 {
                self.offset -= 1;
            }
        }
    }

    fn backtrack(&mut self, amount: usize) {
        if amount > self.offset {
            self.reset_offset();
        } else {
            self.offset -= amount;
        }
    }

    fn match_terminal_symbol(&mut self, token_string: &str) -> Option<Token> {
        let mut result = String::new();
        let location = self.location;

        for ch in token_string.chars() {
            match self.peek_char() {
                Some(other_ch) => {
                    if other_ch == ch {
                        self.read_char();
                        result.push(other_ch);
                    } else {
                        self.backtrack(result.len());
                        return None;
                    }
                }
                None => return None,
            }
        }

        self.consume();
        let token_type = token::string_to_token_type(token_string).unwrap();

        Some(Token::new(token_type, token_string, location))
    }

    #[inline]
    fn scan_symbol(&mut self, token_type: TokenType) -> Option<Token> {
        self.match_terminal_symbol(token_type.armor_string().unwrap())
    }

    fn scan_five_dashes(&mut self) -> Option<Token> {
        self.scan_symbol(TokenType::FiveDashes)
    }

    fn scan_colon_space(&mut self) -> Option<Token> {
        self.scan_symbol(TokenType::ColonSpace)
    }

    fn scan_forwardslash(&mut self) -> Option<Token> {
        self.scan_symbol(TokenType::ForwardSlash)
    }

    fn scan_colon(&mut self) -> Option<Token> {
        self.scan_symbol(TokenType::Colon)
    }

    fn scan_plus_sign(&mut self) -> Option<Token> {
        self.scan_symbol(TokenType::PlusSign)
    }

    fn scan_pad_symbol(&mut self) -> Option<Token> {
        self.scan_symbol(TokenType::Pad)
    }

    fn scan_whitespace_symbol(&mut self) -> Option<Token> {
        self.scan_symbol(TokenType::WhiteSpace)
    }

    fn scan_comma(&mut self) -> Option<Token> {
        self.scan_symbol(TokenType::Comma)
    }

    fn scan_newline(&mut self) -> Option<Token> {
        self.scan_symbol(TokenType::NewLine)
    }

    fn scan_begin(&mut self) -> Option<Token> {
        self.scan_symbol(TokenType::Begin)
    }

    fn scan_end(&mut self) -> Option<Token> {
        self.scan_symbol(TokenType::End)
    }

    fn scan_version(&mut self) -> Option<Token> {
        self.scan_symbol(TokenType::Version)
    }

    fn scan_comment(&mut self) -> Option<Token> {
        self.scan_symbol(TokenType::Comment)
    }

    fn scan_messageid(&mut self) -> Option<Token> {
        self.scan_symbol(TokenType::MessageID)
    }

    fn scan_hash(&mut self) -> Option<Token> {
        self.scan_symbol(TokenType::Hash)
    }

    fn scan_charset(&mut self) -> Option<Token> {
        self.scan_symbol(TokenType::Charset)
    }

    fn scan_pgp_message(&mut self) -> Option<Token> {
        self.scan_symbol(TokenType::PGPMessage)
    }

    fn scan_pgp_public_key_block(&mut self) -> Option<Token> {
        self.scan_symbol(TokenType::PGPPublicKeyBlock)
    }

    fn scan_pgp_private_key_block(&mut self) -> Option<Token> {
        self.scan_symbol(TokenType::PGPPrivateKeyBlock)
    }

    fn scan_pgp_message_part(&mut self) -> Option<Token> {
        self.scan_symbol(TokenType::PGPMessagePart)
    }

    fn scan_pgp_signature(&mut self) -> Option<Token> {
        self.scan_symbol(TokenType::PGPSignature)
    }

    fn scan_other_utf8(&mut self) -> Option<Token> {
        let location = self.location;
        let result = self.read_char()
                         .map(|ch| Token::from_char(TokenType::OtherUtf8, ch, location));
        self.consume_char();

        result
    }

    fn process_char(&mut self, ch: &str, token_type: TokenType) -> Option<Token> {
        let result = Some(Token::new(token_type, ch, self.location));
        self.consume_char();

        result
    }

    fn scan_letter(&mut self) -> Option<Token> {
        match self.peek_char() {
            Some('a') => self.process_char("a", TokenType::Letter),
            Some('b') => self.process_char("b", TokenType::Letter),
            Some('c') => self.process_char("c", TokenType::Letter),
            Some('d') => self.process_char("d", TokenType::Letter),
            Some('e') => self.process_char("e", TokenType::Letter),
            Some('f') => self.process_char("f", TokenType::Letter),
            Some('g') => self.process_char("g", TokenType::Letter),
            Some('h') => self.process_char("h", TokenType::Letter),
            Some('i') => self.process_char("i", TokenType::Letter),
            Some('j') => self.process_char("j", TokenType::Letter),
            Some('k') => self.process_char("k", TokenType::Letter),
            Some('l') => self.process_char("l", TokenType::Letter),
            Some('m') => self.process_char("m", TokenType::Letter),
            Some('n') => self.process_char("n", TokenType::Letter),
            Some('o') => self.process_char("o", TokenType::Letter),
            Some('p') => self.process_char("p", TokenType::Letter),
            Some('q') => self.process_char("q", TokenType::Letter),
            Some('r') => self.process_char("r", TokenType::Letter),
            Some('s') => self.process_char("s", TokenType::Letter),
            Some('t') => self.process_char("t", TokenType::Letter),
            Some('u') => self.process_char("u", TokenType::Letter),
            Some('v') => self.process_char("v", TokenType::Letter),
            Some('w') => self.process_char("w", TokenType::Letter),
            Some('x') => self.process_char("x", TokenType::Letter),
            Some('y') => self.process_char("y", TokenType::Letter),
            Some('z') => self.process_char("z", TokenType::Letter),
            Some('A') => self.process_char("A", TokenType::Letter),
            Some('B') => self.process_char("B", TokenType::Letter),
            Some('C') => self.process_char("C", TokenType::Letter),
            Some('D') => self.process_char("D", TokenType::Letter),
            Some('E') => self.process_char("E", TokenType::Letter),
            Some('F') => self.process_char("F", TokenType::Letter),
            Some('G') => self.process_char("G", TokenType::Letter),
            Some('H') => self.process_char("H", TokenType::Letter),
            Some('I') => self.process_char("I", TokenType::Letter),
            Some('J') => self.process_char("J", TokenType::Letter),
            Some('K') => self.process_char("K", TokenType::Letter),
            Some('L') => self.process_char("L", TokenType::Letter),
            Some('M') => self.process_char("M", TokenType::Letter),
            Some('N') => self.process_char("N", TokenType::Letter),
            Some('O') => self.process_char("O", TokenType::Letter),
            Some('P') => self.process_char("P", TokenType::Letter),
            Some('Q') => self.process_char("Q", TokenType::Letter),
            Some('R') => self.process_char("R", TokenType::Letter),
            Some('S') => self.process_char("S", TokenType::Letter),
            Some('T') => self.process_char("T", TokenType::Letter),
            Some('U') => self.process_char("U", TokenType::Letter),
            Some('V') => self.process_char("V", TokenType::Letter),
            Some('W') => self.process_char("W", TokenType::Letter),
            Some('X') => self.process_char("X", TokenType::Letter),
            Some('Y') => self.process_char("Y", TokenType::Letter),
            Some('Z') => self.process_char("Z", TokenType::Letter),
            _ => None,
        }
    }

    fn scan_digit(&mut self) -> Option<Token> {
        match self.peek_char() {
            Some('0') => self.process_char("0", TokenType::Digit),
            Some('1') => self.process_char("1", TokenType::Digit),
            Some('2') => self.process_char("2", TokenType::Digit),
            Some('3') => self.process_char("3", TokenType::Digit),
            Some('4') => self.process_char("4", TokenType::Digit),
            Some('5') => self.process_char("5", TokenType::Digit),
            Some('6') => self.process_char("6", TokenType::Digit),
            Some('7') => self.process_char("7", TokenType::Digit),
            Some('8') => self.process_char("8", TokenType::Digit),
            Some('9') => self.process_char("9", TokenType::Digit),
            _ => None,
        }
    }

    fn scan_blankline(&mut self) -> Option<Token> {
        let mut result = String::new();
        let location = self.location;

        match self.peek_char() {
            Some('\n') => {
                result.push('\n');
                self.read_char();
            }
            _ => {
                return None;
            }
        }

        loop {
            match self.peek_char() {
                Some(' ') => {
                    result.push(' ');
                    self.read_char();
                }
                Some('\n') => {
                    result.push('\n');
                    self.read_char();
                    break;
                }
                _ => {
                    self.backtrack(result.len());
                    return None;
                }
            }
        }

        if result.is_empty() {
            None
        } else {
            self.consume();
            Some(Token::new(TokenType::BlankLine, result.as_str(), location))
        }
    }

    fn scan_eof(&mut self) -> Option<Token> {
        match self.peek_char() {
            Some(_) => None,
            None => {
                let token_string = TokenType::Eof.armor_string().unwrap();

                Some(Token::new(TokenType::Eof, token_string, Location::eof()))
            }
        }
    }
}

impl<S> Iterator for Lexer<S> where S: Iterator<Item = char> {
    type Item = Token;

    fn next(&mut self) -> Option<Token> {
        let next_token = self.next_token();
        if next_token.has_token_type(TokenType::Eof) {
            None
        } else {
            Some(next_token)
        }
    }
}


#[cfg(test)]
mod tests {
    use super::Lexer;


    fn ascii_armored_data() -> String {
        String::from("-----BEGIN PGP MESSAGE-----\n\
                      Version: OpenPrivacy 0.99\n      \n\
                      yDgBO22WxBHv7O8X7O/jygAEzol56iUKiXmV+XmpCtmpqQUKiQrFqclFqUDBovzS\n\
                      vBSFjNSiVHsuAA==\n\
                      =njUN\n\
                      -----END PGP MESSAGE-----")
    }

    #[test]
    fn test_armor_lexer() {
        let armored_data = ascii_armored_data();
        let mut lexer = Lexer::new(armored_data.chars());

        for token in &mut lexer {
            assert!(token.is_valid_token());
        }
    }
}
