use armor_lexer::{TokenType, ArmorToken, ArmorLexer};


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

    pub fn match_token(&mut self, token_type: TokenType) -> Result<(), String> {
        if self.lookahead_type(1) == token_type {
            self.consume();
            Ok(())
        } else {
            let err_str = format!("Expecting: {:?}; Found {:?}", token_type, self.lookahead_token(1));
            Err(err_str)
        }
    }
}