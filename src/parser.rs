use std::collections::VecDeque;
use lexer::Lexer;
use token::{Token, TokenType};


enum MessageType {
    PGPMessage,
    PGPPublicKeyBlock,
    PGPPrivateKeyBlock,
    PGPSignature,
    PGPMessagePartXofY(usize, usize),
    PGPMessagePartX(usize),
}

enum ParseError {
    CorruptHeader,
    EndOfFile,
    ParseError,
}

type ParseResult<T> = Result<T, ParseError>;


pub struct Parser<S> where S: Iterator<Item = char> {
    input:     Lexer<S>,
    buffer:    VecDeque<Token>,
}

impl<S> Parser<S> where S: Iterator<Item = char> {
    pub fn new(mut input: Lexer<S>, n: usize) -> Parser<S> {
        let mut buffer = VecDeque::with_capacity(n);
        let mut p = 0;
        // Initialize buffer.
        loop {
            let next_token = input.next();
            match next_token {
                Some(token) => {
                    buffer.push_back(token);
                    p += 1;
                }
                None => break
            }
        }

        Parser {
            input: input,
            buffer: buffer,
        }
    }

    fn parse_number(&mut self) -> ParseResult<usize> {
        let mut result = String::new();
        loop {
            match self.input.next() {
                Some(token) => {
                    match token.token_type() {
                        TokenType::Digit => {
                            result.push_str(token.as_str());
                        }
                        _ => {
                            break;
                        }
                    }
                }
                None => {
                    return Err(ParseError::EndOfFile);
                }
            }
        }

        if !result.is_empty() {
            let parse_result = result.parse::<usize>().unwrap();
            Ok(parse_result)
        } else {
            Err(ParseError::ParseError)
        }
    }

    fn parse_header_line(&mut self) -> ParseResult<MessageType> {
        match self.input.next() {
            Some(token) => {
                if !token.has_token_type(TokenType::FiveDashes) {
                    return Err(ParseError::CorruptHeader);
                }
            } None => {
                return Err(ParseError::EndOfFile);
            }
        }
        match self.input.next() {
            Some(token) => {
                if !token.has_token_type(TokenType::Begin) {
                    return Err(ParseError::CorruptHeader);
                }
            }
            None => {
                return Err(ParseError::EndOfFile);
            }
        }

        match self.input.next() {
            Some(token) => {
                match token.token_type() {
                    TokenType::PGPMessagePart => {
                        let num1 = self.parse_number();
                        match self.input.next() {
                            Some(token) => {
                                match token.token_type() {
                                    TokenType::ForwardSlash => {}
                                    _ => {
                                        return Err(ParseError::CorruptHeader);
                                    }
                                }
                            }
                            None => {
                                return Err(ParseError::CorruptHeader);
                            }
                        }
                        let num2 = self.parse_number();
                    }
                    TokenType::PGPMessage => {}
                    TokenType::PGPPublicKeyBlock => {}
                    TokenType::PGPPrivateKeyBlock => {}
                    TokenType::PGPSignature => {}
                    _ => {
                        return Err(ParseError::CorruptHeader)
                    }
                }
            }
            None => {
                return Err(ParseError::EndOfFile);
            }
        }

        match self.input.next() {
            Some(token) => {
                if !token.has_token_type(TokenType::FiveDashes) {
                    return Err(ParseError::CorruptHeader);
                }
            }
            None => {
                return Err(ParseError::EndOfFile);
            }
        }

        Ok(MessageType::PGPMessage)

    }
}


#[cfg(test)]
mod tests {

}
