use std::collections::VecDeque;
use std::iter::Peekable;
use lexer::Lexer;
use token::{Token, TokenType};


#[derive(Clone, PartialEq, Eq, Debug)]
enum MessageType {
    PGPMessage,
    PGPPublicKeyBlock,
    PGPPrivateKeyBlock,
    PGPSignature,
    PGPMessagePartXofY(usize, usize),
    PGPMessagePartX(usize),
}

#[derive(Clone, PartialEq, Eq, Debug)]
enum ParseError {
    CorruptHeader,
    EndOfFile,
    ParseError,
}

type ParseResult<T> = Result<T, ParseError>;

impl ParseError {
    fn eof<T>() -> ParseResult<T> {
        Err(ParseError::EndOfFile)
    }

    fn parse_error<T>() -> ParseResult<T> {
        Err(ParseError::ParseError)
    }

    fn corrupt_header<T>() -> ParseResult<T> {
        Err(ParseError::CorruptHeader)
    }
}


pub struct Parser<S> where S: Iterator<Item=char> {
    input:  Peekable<Lexer<S>>,
    lookahead: VecDeque<Token>,
    offset: usize
}

impl<S> Parser<S> where S: Iterator<Item=char> {
    pub fn new(mut input: Lexer<S>) -> Parser<S> {
        Parser {
            input:     input.peekable(),
            lookahead: VecDeque::with_capacity(20),
            offset:    0
        }
    }

    fn peek_token(&mut self) -> Option<Token> {
        if self.lookahead.is_empty() {
            self.offset = 0;
            let next_token = self.input.next();
            if next_token.is_some() {
                self.lookahead.push_back(next_token.clone().unwrap());
                Some(next_token.unwrap())
            } else {
                None
            }
        } else {
            self.sync();
            Some(self.lookahead[self.offset].clone())
        }
    }

    fn read_token(&mut self) -> Option<Token> {
        match self.peek_token() {
            Some(token) => {
                self.offset += 1;
                Some(token)
            }
            None => None
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
                Some(token) => {
                    self.lookahead.push_back(token);
                }
                None => break
            }
        }
    }

    fn reset_offset(&mut self) {
        self.offset = 0;
    }

    fn consume(&mut self) {
        for _ in 0..self.offset {
            self.lookahead.pop_front();
        }
        self.reset_offset();
    }

    fn consume_char(&mut self) {
        if self.lookahead.is_empty() {
            self.reset_offset();
        } else {
            self.lookahead.pop_front();
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

    fn parse_number(&mut self) -> ParseResult<usize> {
        let mut result = String::new();
        loop {
            match self.peek_token() {
                Some(token) => {
                    match token.token_type() {
                        TokenType::Digit => {
                            result.push_str(token.as_str());
                            self.read_token();
                        }
                        _ => break
                    }
                }
                None => break
            }
        }

        if !result.is_empty() {
            let parse_result = result.parse::<usize>().unwrap();
            Ok(parse_result)
        } else if self.peek_token().is_none() {
            Err(ParseError::EndOfFile)
        } else {
            Err(ParseError::ParseError)
        }
    }

    fn read_or_else(&mut self, tt: TokenType, err: ParseError) -> ParseResult<Token> {
        match self.peek_token() {
            Some(token) => {
                if !token.has_token_type(tt) {
                    return Err(err);
                }
            } None => {
                return Err(ParseError::EndOfFile);
            }
        }

        Ok(self.read_token().unwrap())
    }

    fn parse_x_div_y(&mut self) -> ParseResult<(usize, usize)> {
        let num_x = self.parse_number();
        match num_x {
            Ok(_) => {}
            Err(e) => return Err(e)
        }

        match self.peek_token() {
            Some(token) => {
                match token.token_type() {
                    TokenType::ForwardSlash => {}
                    TokenType::FiveDashes => {}
                    _ => {
                        return Err(ParseError::CorruptHeader);
                    }
                }
            }
            None => return Err(ParseError::CorruptHeader)
        }

        let num_y = self.parse_number();
        match num_y {
            Ok(_) => {}
            Err(e) => return Err(e)
        }

        Ok((num_x.unwrap(), num_y.unwrap()))
    }

    fn parse_pgp_message_part(&mut self) -> ParseResult<MessageType> {
        match self.peek_token() {
            Some(token) => {
                match token.token_type() {
                    TokenType::PGPMessagePart => {
                        match self.parse_x_div_y() {
                            Ok((x,y)) => {
                                self.consume();
                                return Ok(MessageType::PGPMessagePartXofY(x,y))
                            }
                            Err(_)    => {}
                        }
                        match self.parse_number() {
                            Ok(x)  => {
                                self.consume();
                                return Ok(MessageType::PGPMessagePartX(x))
                            }
                            Err(e) => Err(ParseError::CorruptHeader)
                        }
                    }
                    _ => return Err(ParseError::CorruptHeader)
                }
            }
            None => return Err(ParseError::EndOfFile)
        }
    }

    fn parse_pgp_message(&mut self) -> ParseResult<MessageType> {
        match self.peek_token() {
            Some(token) => {
                match token.token_type() {
                    TokenType::PGPMessage => {
                        self.consume();
                        Ok(MessageType::PGPMessage)
                    }
                    _ => Err(ParseError::CorruptHeader)
                }
            }
            None => return Err(ParseError::EndOfFile)
        }
    }

    fn parse_pgp_publickey_block(&mut self) -> ParseResult<MessageType> {
        match self.peek_token() {
            Some(token) => {
                match token.token_type() {
                    TokenType::PGPPublicKeyBlock => {
                        self.consume();
                        Ok(MessageType::PGPPublicKeyBlock)
                    }
                    _ => Err(ParseError::CorruptHeader)
                }
            }
            None => return Err(ParseError::EndOfFile)
        }
    }

    fn parse_pgp_privatekey_block(&mut self) -> ParseResult<MessageType> {
        match self.peek_token() {
            Some(token) => {
                match token.token_type() {
                    TokenType::PGPPrivateKeyBlock => {
                        self.consume();
                        Ok(MessageType::PGPPrivateKeyBlock)
                    }
                    _ => Err(ParseError::CorruptHeader)
                }
            }
            None => return Err(ParseError::EndOfFile)
        }
    }

    fn parse_pgp_signature(&mut self) -> ParseResult<MessageType> {
        match self.peek_token() {
            Some(token) => {
                match token.token_type() {
                    TokenType::PGPSignature => {
                        self.consume();
                        Ok(MessageType::PGPSignature)
                    }
                    _ => Err(ParseError::CorruptHeader)
                }
            }
            None => return Err(ParseError::EndOfFile)
        }
    }

    fn parse_header_line(&mut self) -> ParseResult<MessageType> {
        match self.read_or_else(TokenType::FiveDashes, ParseError::CorruptHeader) {
            Ok(_)  => {}
            Err(e) => return Err(e)
        }
        match self.read_or_else(TokenType::Begin, ParseError::CorruptHeader) {
            Ok(_) => {}
            Err(e) => return Err(e)
        }

        let message_type = match self.peek_token() {
            Some(token) => {
                match token.token_type() {
                    TokenType::PGPMessagePart     => self.parse_pgp_message_part(),
                    TokenType::PGPMessage         => self.parse_pgp_message(),
                    TokenType::PGPPublicKeyBlock  => self.parse_pgp_publickey_block(),
                    TokenType::PGPPrivateKeyBlock => self.parse_pgp_privatekey_block(),
                    TokenType::PGPSignature       => self.parse_pgp_signature(),
                    _ => return Err(ParseError::CorruptHeader)
                }
            }
            None => return Err(ParseError::EndOfFile)
        };

        match message_type {
            Ok(_) => {}
            Err(e) => return Err(e)
        }

        match self.read_or_else(TokenType::FiveDashes, ParseError::CorruptHeader) {
            Ok(_)  => {}
            Err(e) => return Err(e)
        }

        self.consume();
        Ok(message_type.unwrap())
    }
}


#[cfg(test)]
mod tests {

}
