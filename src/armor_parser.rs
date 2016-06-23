use armor_lexer::{ArmorToken, ArmorLexer};


pub struct ArmorParser<'a> {
    input:     ArmorLexer<'a>,      
    n:         usize,           // How many lookahead symbols?
    lookahead: Vec<ArmorToken>,
    p:         usize,
}

impl<'a> ArmorParser<'a> {
    fn new(mut input: ArmorLexer<'a>, n: usize) -> ArmorParser<'a> {
        let mut lookahead = Vec::new();
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
            n: n,
            p: p,
        }
    }
}