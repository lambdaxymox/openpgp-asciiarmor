use std::marker::PhantomData;
use combine::combinator::{And, Or, Token, Satisfy, Expected};
use combine::primitives::Stream;
use combine::{ParserExt, Parser, State, ParseResult, ParseError};
use combine::char;
use combine::char::{Upper, Lower, Spaces, NewLine, CrLf};


pub struct BlankLine<I> where I: Stream<Item=char> {
    inner: And<Spaces<I>, Or<NewLine<I>, CrLf<I>>>,
    _marker: PhantomData<I>,
}

pub fn blankline<I>() -> BlankLine<I> where I: Stream<Item=char> {
    BlankLine {
        inner: char::spaces().and(char::newline().or(char::crlf())),
        _marker: PhantomData,
    }
}

impl<I> Parser for BlankLine<I> where I: Stream<Item=char> {
    type Input = I;
    type Output = <And<Spaces<I>, Or<NewLine<I>, CrLf<I>>> as Parser>::Output;

    fn parse_lazy(&mut self, input: Self::Input) -> ParseResult<Self::Output, Self::Input> {
        self.inner.parse_lazy(input)
    }

    fn add_error(&mut self, _error: &mut ParseError<Self::Input>) {
        self.inner.add_error(_error);
    }
}
