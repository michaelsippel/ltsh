use {
    crate::ast::*,
    std::iter::{Peekable, FromIterator},
};

pub struct WordLexer<It>
where It: Iterator<Item = char> {
    chars: Peekable<It>
}

impl<It> From<It> for WordLexer<It>
where It: Iterator<Item = char> {
    fn from(iter: It) -> Self {
        WordLexer {
            chars: iter.into_iter().peekable()
        }
    }
}

#[derive(Debug)]
pub enum LexError {
    UnexpectedEnd(char)
}

impl<It> WordLexer<It>
where It: Iterator<Item = char> {
    fn collect_until(&mut self, close: char) -> Result<String, LexError> {
        let mut val = String::new();
        while let Some(c) = self.chars.peek().cloned() {
            if c == close {
                return Ok(val)
            } else {
                self.chars.next();
                val.push(c);
            }
        }

        if close.is_whitespace() {
            Ok(val)
        } else {
            Err(LexError::UnexpectedEnd(close))
        }
    }
}

impl<It> Iterator for WordLexer<It>
where It: Iterator<Item = char> {
    type Item = Result<WordSegment, LexError>;

    fn next(&mut self) -> Option<Result<WordSegment, LexError>> {
        match self.chars.peek().cloned() {
            Some('~') => {
                self.chars.next();
                match self.collect_until(' ') {
                    Ok(s) => Some(Ok(WordSegment::Tilde(s))),
                    Err(e) => Some(Err(e))
                }
            }
            Some('"') => {
                self.chars.next();
                match self.collect_until('"') {
                    Ok(s) => {
                        self.chars.next();

                        let word = Word {
                            segments: WordLexer { chars: s.chars().peekable() }
                            .scan((), |_, x| x.ok())
                                .collect::<Vec<_>>()
                        };

                        Some(Ok(WordSegment::DoubleQuote(word)))
                    },
                    Err(e) => Some(Err(e))
                }
            },
            Some('\'') => {
                self.chars.next();
                match self.collect_until('\'') {
                    Ok(s) => {
                        self.chars.next();
                        Some(Ok(WordSegment::Literal(s)))
                    },
                    Err(e) => Some(Err(e))
                }
            },
            Some('$') => {
                self.chars.next();
                match self.chars.peek() {
                    Some('{') => {
                        self.chars.next();
                        match self.collect_until('}') {
                            Ok(s) => {
                                self.chars.next();
                                Some(Ok(WordSegment::Variable(s)))
                            }
                            Err(e) => Some(Err(e))
                        }
                    }
                    _ => {
                        match self.collect_until(' ') {
                            Ok(s) => {
                                Some(Ok(WordSegment::Variable(s)))
                            }
                            Err(e) => Some(Err(e))
                        }
                    }
                }
            }
            Some(c) => {
                while let Some(c) = self.chars.peek() {
                    if c.is_whitespace() {
                        self.chars.next();
                    } else {
                        return match self.collect_until(' ') {
                            Ok(s) => {
                                Some(Ok(WordSegment::Literal(s)))
                            }
                            Err(e) => Some(Err(e))
                        };
                    }
                }
                None
            }
            None => {
                None
            }
        }
    }
}

