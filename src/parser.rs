/*
use lexer::{TokenType, ArmorToken, ArmorLexer};


pub struct ArmorParser<'a> {
    input:          ArmorLexer<'a>,      
    lookahead_size: usize,           // How many lookahead symbols?
    lookahead:      Vec<ArmorToken>,
    p:              usize,
}

impl<'a> ArmorParser<'a> {
    fn new(mut input: ArmorLexer<'a>, n: usize) -> ArmorParser<'a> {
        let mut lookahead = Vec::with_capacity(n);
        let mut p = 0;
        // Initialize buffer.
        loop {
            let token = input.next_token();
            match token {
                Some(val) => {
                    lookahead.push(val);
                    p += 1;
                }
                None => break,
            }
        }

        ArmorParser {
            input: input,
            lookahead: lookahead,
            lookahead_size: n,
            p: p,
        }
    }

    #[inline]
    fn consume(&mut self) -> Result<(), ()> {
        let next_token = self.input.next_token();
        match next_token {
            Some(token) => {
                self.lookahead[self.p] = token;
                self.p = (self.p + 1) % self.lookahead_size;
                Ok(())
            }
            None => Err(()),
        }
    }

    #[inline]
    pub fn lookahead_token(&self, i: usize) -> ArmorToken {
        self.lookahead[(self.p+i-1) % self.lookahead_size]
    }

    #[inline]
    pub fn lookahead_type(&self, i: usize) -> TokenType {
        self.lookahead_token(i).token_type()
    }

    pub fn match_token(&mut self, token_type: TokenType) -> Result<ArmorToken, String> {
        if self.lookahead_type(1) == token_type {
            let token = self.lookahead_token(1);
            self.consume();
            Ok(token)
        } else {
            let err_str = format!("Expecting: {:?}; Found {:?}", token_type, self.lookahead_token(1));
            Err(err_str)
        }
    }
    
    pub fn upper_case_letter(&mut self) -> Result<ArmorToken, ()> {
        let token = self.lookahead_token(1);
        if token.is_upper_case() {
            self.consume();
            Ok(token)
        } else {
            Err(())
        }
    }

    pub fn lower_case_letter(&mut self) -> Result<ArmorToken, ()> {
        let token = self.lookahead_token(1);
        if token.is_lower_case() {
            self.consume();
            Ok(token)
        } else {
            Err(())
        }
    }

    pub fn letter(&mut self) -> Result<ArmorToken, ()> {
        let token = self.lookahead_token(1);
        if token.is_letter() {
            self.consume();
            Ok(token)
        } else {
            Err(())
        }
    }

    pub fn digit(&mut self) -> Result<ArmorToken, ()> {
        let token = self.lookahead_token(1);
        if token.is_digit() {
            self.consume();
            Ok(token)
        } else {
            Err(())
        }
    }

    pub fn whitespace(&mut self) -> Result<ArmorToken, ()> {
        let token = self.lookahead_token(1);
        if token.is_whitespace() {
            self.consume();
            Ok(token)
        } else {
            Err(())
        }
    }

    pub fn number(&mut self) -> Result<String, ()> {
        let mut parsed_number = String::new();

        loop {
            let result = self.digit();
            match result {
                Ok(token) => {
                    parsed_number.push(token.token());
                }
                Err(_) => {
                    break;
                }
            }
        }

        if parsed_number.is_empty() {
            return Err(());
        }
        
        Ok(parsed_number)
    }

    pub fn pad(&mut self) -> Result<ArmorToken, ()> {
        let token = self.lookahead_token(1);
        if token.is_equal_sign() {
            self.consume();
            Ok(token)
        } else {
            Err(())
        }
    }

    pub fn blank_line(&mut self) -> Result<String, ()> {
        let mut parsed_blank = String::new();

        loop {
            let token = self.lookahead_token(1);
            if token.is_whitespace() {
                self.consume();
                parsed_blank.push(token.token());
            }
        }

        let last_token = self.lookahead_token(1);
        if last_token.is_newline() {
            self.consume();
            parsed_blank.push(last_token.token());

            Ok(parsed_blank)
        } else {
            Err(())
        }
    }
}


#[cfg(test)]
mod tests {

}
*/