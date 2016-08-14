use std::collections::VecDeque;
use std::iter::Peekable;
use lexer::Lexer;
use token::{Token, TokenType};
use base64::Base64;
use crc24;
use std::io;
use std::io::Write;

#[derive(Clone, PartialEq, Eq, Debug)]
enum MessageType {
    PGPMessage,
    PGPPublicKeyBlock,
    PGPPrivateKeyBlock,
    PGPSignature,
    PGPMessagePartXofY(usize, usize),
    PGPMessagePartX(usize)
}

#[derive(Clone, PartialEq, Eq, Debug)]
enum HeaderType {
    Version,
    Comment,
    MessageID,
    Hash,
    Charset,
    OtherHeader(String)
}

fn token_type_to_header_type(token_type: TokenType) -> HeaderType {
    match token_type {
        TokenType::Version   => HeaderType::Version,
        TokenType::Comment   => HeaderType::Comment,
        TokenType::MessageID => HeaderType::MessageID,
        TokenType::Hash      => HeaderType::Hash,
        TokenType::Charset   => HeaderType::Charset,
        _                    => HeaderType::OtherHeader(String::new())
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
struct Header {
    header_type: MessageType,
    header_block: Vec<(HeaderType, String)>
}

#[derive(Clone, PartialEq, Eq, Debug)]
struct Body {
    body: Base64,
    checksum: crc24::Crc24
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum ParseError {
    CorruptHeader,
    InvalidHeaderLine,
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
    pub fn new(input: Lexer<S>) -> Parser<S> {
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

    fn read_token_or_else(&mut self, tt: TokenType, err: ParseError) -> ParseResult<Token> {
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
                                self.consume(); // Do I want to consume here?
                                return Ok(MessageType::PGPMessagePartXofY(x,y))
                            }
                            Err(_)    => {}
                        }
                        match self.parse_number() {
                            Ok(x)  => {
                                self.consume(); // Do I want to consume here?
                                return Ok(MessageType::PGPMessagePartX(x))
                            }
                            Err(_) => {
                                Err(ParseError::CorruptHeader)
                            }
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
                        self.read_token();
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

    fn parse_header_tail_line(&mut self, token_type: TokenType) -> ParseResult<MessageType> {
        match self.read_token_or_else(TokenType::FiveDashes, ParseError::CorruptHeader) {
            Ok(_)  => {}
            Err(e) => return Err(e)
        }

        match self.read_token_or_else(token_type, ParseError::CorruptHeader) {
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

        match self.read_token_or_else(TokenType::FiveDashes, ParseError::CorruptHeader) {
            Ok(_)  => {}
            Err(e) => return Err(e)
        }

        self.consume();
        Ok(message_type.unwrap())
    }

    fn parse_header_line(&mut self) -> ParseResult<MessageType> {
        self.parse_header_tail_line(TokenType::Begin)
    }

    fn parse_tail_line(&mut self) -> ParseResult<MessageType> {
        self.parse_header_tail_line(TokenType::End)
    }

    fn parse_header_text(&mut self) -> ParseResult<String> {
        let mut result = String::new();
        loop {
            match self.peek_token() {
                Some(token) => {
                    match token.token_type() {
                        TokenType::Version
                            | TokenType::Comment
                            | TokenType::MessageID
                            | TokenType::Hash
                            | TokenType::Charset => break,
                        TokenType::BlankLine => break,
                        _ => {
                            result.push_str(token.as_str());
                            self.read_token();
                        }
                    }
                }
                None => {
                    return Err(ParseError::EndOfFile)
                }
            }
        }

        Ok(result)
    }

    fn skip_whitespace(&mut self) -> ParseResult<()> {
        loop {
            match self.peek_token() {
                Some(token) => {
                    match token.token_type() {
                        TokenType::WhiteSpace => {
                            self.read_token();
                        }
                        _ => break
                    }
                }
                None => return Err(ParseError::EndOfFile)
            }
        }

        Ok(())
    }

    fn parse_headerkv(&mut self) -> ParseResult<(HeaderType, String)> {
        let header_type = match self.peek_token() {
            Some(token) => {
                match token.token_type() {
                    tt @ TokenType::Version
                        | tt @ TokenType::Comment
                        | tt @ TokenType::MessageID
                        | tt @ TokenType::Hash
                        | tt @ TokenType::Charset => {
                           Ok(token_type_to_header_type(tt))
                    }
                    _ => return Err(ParseError::InvalidHeaderLine)
                }
            }
            None => {
                return Err(ParseError::EndOfFile)
            }
        };
        match header_type {
            Ok(_) => {
                self.read_token();
                self.skip_whitespace();
            }
            Err(e) => return Err(e)
        }
        match self.peek_token() {
            Some(token) => {
                match token.token_type() {
                    TokenType::ColonSpace => {
                        self.read_token();
                        self.skip_whitespace();
                    }
                    _ => return Err(ParseError::InvalidHeaderLine)
                }
            }
            None => {
                return Err(ParseError::EndOfFile)
            }
        }
        let header_text = match self.peek_token() {
            Some(_) => {
                self.parse_header_text()
            }
            None => {
                return Err(ParseError::EndOfFile)
            }
        };
        match header_text {
            Ok(_) => {}
            Err(e) => return Err(e)
        }

        self.consume();
        Ok((header_type.unwrap(), header_text.unwrap()))
    }

    fn parse_header_block(&mut self) -> ParseResult<Vec<(HeaderType, String)>> {
        let mut result = Vec::new();
        loop {
            match self.peek_token() {
                Some(token) => {
                    match token.token_type() {
                        TokenType::Version
                           | TokenType::Comment
                           | TokenType::MessageID
                           | TokenType::Hash
                           | TokenType::Charset => {

                            let kv = self.parse_headerkv();
                            match kv {
                                Ok((key, val)) => {
                                    result.push((key, val));
                                }
                                Err(e) => return Err(e)
                            }
                        }
                        TokenType::BlankLine => {
                            self.read_token();
                            break;
                        }
                        _ => {
                            return Err(ParseError::CorruptHeader)
                        }
                    }
                }
                None => {
                    return Err(ParseError::EndOfFile)
                }
            }
        }

        self.consume();
        Ok(result)
    }

    fn parse_header(&mut self) -> ParseResult<Header> {
        let header_type: ParseResult<MessageType> = match self.parse_header_line() {
            Ok(val) => Ok(val),
            Err(e) => return Err(e)
        };

        self.skip_whitespace();

        let header_block: ParseResult<Vec<(HeaderType, String)>> = match self.parse_header_block() {
            Ok(block) => Ok(block),
            Err(e) => return Err(e)
        };

        let header = Header {
            header_type: header_type.unwrap(),
            header_block: header_block.unwrap()
        };

        self.consume();
        Ok(header)
    }

    fn parse_tail(&mut self) -> ParseResult<MessageType> {
        self.parse_tail_line()
    }

}


#[cfg(test)]
mod tests {
    use lexer::Lexer;
    use super::Parser;
    use super::MessageType;
    use std::io;
    use std::io::Write;


    #[test]
    fn test_parse_header_line() {
        let string = String::from("-----BEGIN PGP MESSAGE-----\n");
        let lexer  = Lexer::new(string.chars());
        let mut parser = Parser::new(lexer);
        let result = parser.parse_header_line();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), MessageType::PGPMessage);
    }

    #[test]
    fn test_parse_tail_line() {
        let string = String::from("-----END PGP MESSAGE-----\n");
        let lexer  = Lexer::new(string.chars());
        let mut parser = Parser::new(lexer);
        let result = parser.parse_tail_line();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), MessageType::PGPMessage);
    }

}
