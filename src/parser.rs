use lexer::{Lexer, Token, TokenType};


pub struct Parser<S> where S: Iterator<Item = char> {
    input:          Lexer<S>,
    lookahead_size: usize,           // How many lookahead symbols?
    lookahead:      Vec<Token>,
    p:              usize,
}

impl<S> Parser<S>  where S: Iterator<Item = char> {
    pub fn new(mut input: Lexer<S>, n: usize) -> Parser<S> {
        let mut lookahead = Vec::with_capacity(n);
        let mut p = 0;
        // Initialize buffer.
        loop {
            let next_token = input.next();
            match next_token {
                Some(token) => {
                    lookahead.push(token);
                    p += 1;
                }
                None => break
            }
        }

        Parser {
            input: input,
            lookahead: lookahead,
            lookahead_size: n,
            p: p,
        }
    }
}


#[cfg(test)]
mod tests {

}
