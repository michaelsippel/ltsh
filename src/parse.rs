use {
    crate::ast::*,
    std::iter::{Peekable},
};


#[derive(Debug, PartialEq)]
pub enum LexError {
    UnexpectedEnd(Vec<Option<char>>),
    UnexpectedToken(char),
    InvalidFileRedirectionType
}


///! iterates chars until it finds some char in `delim`
pub struct DelimIter<'a, It>
where It: Iterator<Item = char> {
    chars: &'a mut Peekable<It>,
    delim: Vec<(Option<char>, bool)>
}

impl<'a, It> DelimIter<'a, It>
where It: Iterator<Item = char> {
    fn new(chars: &'a mut Peekable<It>, delim: Vec<(Option<char>, bool)>) -> Self {
        DelimIter { chars, delim }
    }

    fn new_whitespace(chars: &'a mut Peekable<It>) -> Self {
        DelimIter::new(chars, vec![
            (None, true),
            (Some(' '), true),
            (Some('\t'), true),
            (Some('\n'), true)
        ])
    }

    fn new_shell_word(chars: &'a mut Peekable<It>) -> Self {
        DelimIter::new(chars, vec![
            (None, true),
            (Some(' '), true),
            (Some('\t'), true),
            (Some('\n'), true),
            (Some('|'), false),
            (Some('&'), false),
            (Some(';'), false),
            (Some('\"'), false),
            (Some('\''), false)
        ])
    }
}

impl<'a, It> Iterator for DelimIter<'a, It>
where It: 'a + Iterator<Item = char> {
    type Item = Result<char, LexError>;

    fn next(&mut self) -> Option<Result<char, LexError>> {
        for (delim, consume) in self.delim.iter() {
            if self.chars.peek().cloned() == *delim {
                if *consume {
                    self.chars.next();
                }
                return None;
            }
        }

        match self.chars.next() {
            Some(c) => Some(Ok(c)),
            None => Some(Err(LexError::UnexpectedEnd(vec![])))
        }
    }
}


pub struct WordLexer<'a, It>
where It: 'a + Iterator<Item = char> {
    chars: &'a mut Peekable<It>
}

impl<'a, It> WordLexer<'a, It>
where It: Iterator<Item = char> {
    fn collect_until(&mut self, close: Option<char>) -> Result<String, LexError> {
        DelimIter::new(&mut self.chars, vec![(close, true)])
            .try_collect::<String>()
    }
}

pub fn skip_whitespace<It>(chars: &mut Peekable<It>)
where It: Iterator<Item = char>
{
    while let Some(c) = chars.peek() {
        if c.is_whitespace() {
            chars.next();
        } else {
            break;
        }
    }
}

pub fn parse_quoted<It>(chars: &mut Peekable<It>) -> Result<WordSegment, LexError>
where It: Iterator<Item = char>
{
    assert_eq!( chars.next(), Some('\''));
    let quoted = DelimIter::new(chars, vec![(Some('\''), true)]).try_collect::<String>();
    match quoted {
        Ok(s) => {
            Ok(WordSegment::Literal(s))
        },
        Err(e) => Err(e)
    }
}

pub fn parse_doublequoted<It>(chars: &mut Peekable<It>) -> Result<WordSegment, LexError>
where It: Iterator<Item = char>
{
    assert_eq!( chars.next(), Some('\"'));
    let quoted = DelimIter::new(chars, vec![(Some('\"'), true)]).try_collect::<String>();
    match quoted {
        Ok(s) => {
            let word = Word {
                segments: // fixme: handle spaces correctly -> create QuoteLexer
                WordLexer { chars: &mut s.chars().peekable() }
                .scan((), |_, x| x.ok())
                    .collect::<Vec<_>>()
            };

            Ok(WordSegment::DoubleQuote(word))
        },
        Err(e) => Err(e)
    }    
}

pub fn parse_word<It>(chars: &mut Peekable<It>) -> Result<Word, LexError>
where It: Iterator<Item = char>
{
    Ok(Word {
        segments: WordLexer{ chars }.try_collect::<Vec<_>>()?
    })
}

pub fn parse_assignment<It>(chars: &mut Peekable<It>) -> Result<Assignment, LexError>
where It: Iterator<Item = char>
{
    let name = DelimIter::new(chars, vec![(Some('='), true)]).try_collect::<String>()?;
    let value_str = DelimIter::new_whitespace(chars).try_collect::<String>()?;
    let value = parse_word(&mut value_str.chars().peekable())?;
    Ok(Assignment{ name, value })
}

impl std::str::FromStr for FileRedirectionType {
    type Err = LexError;

    fn from_str(s: &str) -> Result<FileRedirectionType, LexError> {
        match s {
            "<" => Ok(FileRedirectionType::In),
            "<>" => Ok(FileRedirectionType::InOut),
            ">" => Ok(FileRedirectionType::Out),
            ">|" => Ok(FileRedirectionType::OutReplace),
            ">>" => Ok(FileRedirectionType::OutAppend),
            _ => Err(LexError::InvalidFileRedirectionType)
        }
    }
}

pub fn parse_redirection<It>(chars: &mut Peekable<It>) -> Result<Redirection, LexError>
where It: Iterator<Item = char>
{
    Err(LexError::InvalidFileRedirectionType)
    //    let name = DelimIterator::new(chars, vec!['<', '>']).collect::<String>();
}

pub fn parse_simple_cmd<It>(chars: &mut Peekable<It>) -> Result<Option<Command>, LexError>
where It: Iterator<Item = char>
{
    let mut assignments = Vec::new();
    let mut redirections = Vec::new();

    if chars.peek() == None {
        return Ok(None);
    }

    let mut first = DelimIter::new_shell_word(chars).try_collect::<String>()?;

    while first.contains('=') {
        assignments.push( parse_assignment(chars)? );
        first = DelimIter::new_shell_word(chars).try_collect::<String>()?;
    }

    let mut cmd_segments = WordLexer{ chars }.try_collect::<Vec<_>>()?;
    cmd_segments.insert(0, WordSegment::Literal(first));

    Ok(Some(Command::Simple {
        assignments,
        command_word: Word { segments: cmd_segments },
        redirections,
    }))
}

pub fn parse_cmd<It>(chars: &mut Peekable<It>) -> Result<Option<Command>, LexError>
where It: Iterator<Item = char>
{
    skip_whitespace(chars);
    match chars.peek() {
        Some('!') => {
            chars.next();
            if let Some(cmd) = parse_cmd(chars)? {
                Ok(Some(Command::Negation(Box::new(cmd))))
            } else {
                Err(LexError::UnexpectedEnd(vec![]))
            }
        }
        _ => {
            if let Some(head) = parse_simple_cmd(chars)? {
                skip_whitespace(chars);

                match chars.peek() {
                    Some(';') => {
                        chars.next();

                        let tail = parse_cmd( chars ) ?;
                        match tail {
                            Some(Command::Sequence(mut s)) => {
                                s.insert(0, head);
                                Ok(Some(Command::Sequence(s)))
                            }
                            Some(tail) => {
                                Ok(Some(Command::Sequence(vec![ head, tail ])))
                            }
                            None => {
                                Ok(Some(head))
                            }
                        }
                    }
                    Some('|') => {
                        chars.next();
                        match chars.peek() {
                            Some('|') => {
                                chars.next();

                                let tail = parse_cmd( chars ) ?;
                                match tail {
                                    Some(Command::ShortCircuitDisjunction(mut s)) => {
                                        s.insert(0, head);
                                        Ok(Some(Command::ShortCircuitDisjunction(s)))
                                    }
                                    Some(tail) => {
                                        Ok(Some(Command::ShortCircuitDisjunction(vec![ head, tail ])))
                                    }
                                    None => {
                                        Err(LexError::UnexpectedEnd(vec![Some('|')]))
                                    }
                                }
                            }
                            _ => {
                                let tail = parse_cmd( chars ) ?;
                                match tail {
                                    Some(Command::Pipeline(mut s)) => {
                                        s.insert(0, head);
                                        Ok(Some(Command::Pipeline(s)))
                                    }
                                    Some(tail) => {
                                        Ok(Some(Command::Pipeline(vec![ head, tail ])))
                                    }
                                    None => {
                                        Err(LexError::UnexpectedEnd(vec![]))
                                    }
                                }
                            }
                        }
                    }
                    Some('&') => {
                        chars.next();
                        match chars.peek() {
                            Some('&') => {
                                chars.next();

                                let tail = parse_cmd( chars ) ?;
                                match tail {
                                    Some(Command::ShortCircuitConjunction(mut s)) => {
                                        s.insert(0, head);
                                        Ok(Some(Command::ShortCircuitConjunction(s)))
                                    }
                                    Some(tail) => {
                                        Ok(Some(Command::ShortCircuitConjunction(vec![ head, tail ])))
                                    }
                                    None => {
                                        Err(LexError::UnexpectedEnd(vec![Some('&'), Some('&')]))
                                    }
                                }
                            }
                            Some(c) => {
                                Err(LexError::UnexpectedToken(*c))
                            }
                            None => {
                                // todo:
                                // background job
                                Ok(Some(head))
                            }
                        }
                    }
                    Some(c) => {
                        Err(LexError::UnexpectedToken(*c))
                    }
                    None => {
                        Ok(Some(head))
                    }
                }
            } else {
                Ok(None)
            }
        }
    }
}

impl<'a, It> Iterator for WordLexer<'a, It>
where It: 'a + Iterator<Item = char> {
    type Item = Result<WordSegment, LexError>;

    fn next(&mut self) -> Option<Result<WordSegment, LexError>> {
        skip_whitespace(self.chars);
        match self.chars.peek().cloned() {
            Some('|') => { None }
            Some('&') => { None }
            Some(';') => { None }
            Some('~') => {
                self.chars.next();
                let user = DelimIter::new_whitespace(self.chars).collect();
                match user {
                    Ok(user) => Some(Ok(WordSegment::Tilde(user))),
                    Err(e) => Some(Err(e))
                }
            }
            Some('"') => { Some(parse_doublequoted(self.chars)) },
            Some('\'') => { Some(parse_quoted(self.chars)) },
            Some('$') => {
                self.chars.next();
                match self.chars.peek() {
                    Some('{') => {
                        self.chars.next();
                        match DelimIter::new(&mut self.chars, vec![(Some('}'), true)]).try_collect::<String>() {
                            Ok(s) => {
                                Some(Ok(WordSegment::Parameter(s, ParameterFormat::Normal)))
                            }
                            Err(e) => Some(Err(e))
                        }
                    }
                    Some('(') => {
                        self.chars.next();
                        let subcmd_str = DelimIter::new(&mut self.chars, vec![(Some(')'), true)]).try_collect::<String>();
                        match subcmd_str {
                            Ok(subcmd_str) => {
                                match parse_cmd(&mut subcmd_str.chars().peekable()) {
                                    Ok(Some(subcmd)) => {
                                        Some(Ok(WordSegment::Subshell(subcmd)))        
                                    }
                                    Ok(None) => None,
                                    Err(err) => Some(Err(err))
                                }
                            }
                            Err(err) => Some(Err(err))
                        }
                    }
                    _ => {
                        match DelimIter::new_whitespace(self.chars).collect() {
                            Ok(s) => {
                                Some(Ok(WordSegment::Parameter(s, ParameterFormat::Normal)))
                            }
                            Err(e) => Some(Err(e))
                        }
                    }
                }
            }
            Some(c) => {
                let s : Result<String, LexError> = DelimIter::new_shell_word(self.chars).collect();
                match s {
                    Ok(s) => Some(Ok(WordSegment::Literal(s))),
                    Err(e) => Some(Err(e))
                }
            }
            None => {
                None
            }
        }
    }
}


mod test {
    use crate::parse::*;

    #[test]
    fn test_delim_iter() {
        let mut cs = "test 1234".chars().peekable();
        let mut lexer = DelimIter::new_shell_word(&mut cs);
        assert_eq!(lexer.try_collect::<String>(), Ok(String::from("test")));
    }

    #[test]
    fn test_word_lexer() {
        let mut cs = "test   1234|test".chars().peekable();

        {
            let mut lexer = WordLexer{ chars: &mut cs };
            assert_eq!(lexer.next(), Some(Ok(WordSegment::Literal(String::from("test")))));
            assert_eq!(lexer.next(), Some(Ok(WordSegment::Literal(String::from("1234")))));
            assert_eq!(lexer.next(), None);
        }

        assert_eq!(cs.next(), Some('|'));
    }
}

