use lexer::Lexer;
use token::{Token, TokenType};

use std::collections::VecDeque;


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
    OutOfTokens,
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

    fn sync(&mut self) {
        let amount = self.buffer.capacity() - self.buffer.len();

        for _ in 0..amount {
            match self.input.next() {
                Some(token) => {
                    self.buffer.push_back(token);
                }
                None => {
                    break;
                }
            }
        }
    }
}


#[cfg(test)]
mod tests {

}
