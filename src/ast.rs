use std::boxed::Box;

//<<<<>>>><<>><><<>><<<*>>><<>><><<>><<<<>>>>\\

#[derive(Debug, PartialEq)]
pub enum Command {
    Simple {
        assignments: Vec<Assignment>,
        command_word: Word,
        redirections: Vec<Redirection>
    },
    Pipeline(Vec<Command>),
    Sequence(Vec<Command>),
    ShortCircuitConjunction(Vec<Command>),
    ShortCircuitDisjunction(Vec<Command>),
    Negation(Box<Command>),
    While {
        condition: Box<Command>,
        loop_body: Box<Command>
    },
    For {
        varname: String,
        sequence: Word,
        loop_body: Box<Command>
    },
    If {
        condition: Box<Command>,
        then_branch: Box<Command>,
        else_branch: Box<Command>
    },
    Case {
        expr: Word,
        cases: Vec<(Word, Command)>
    },
    Function {
        name: String,
        body: Box<Command>
    }
}

//<<<<>>>><<>><><<>><<<*>>><<>><><<>><<<<>>>>\\

#[derive(Debug, PartialEq)]
pub struct Assignment {
    pub name: String,
    pub value: Word
}

#[derive(Debug, PartialEq)]
pub struct Word {
    pub segments: Vec<WordSegment>
}

#[derive(Debug, PartialEq)]
pub enum WordSegment {
    Tilde(String),
    Literal(String),
    Parameter(String, ParameterFormat),
    Subshell(Command),
    DoubleQuote(Word),
}

#[derive(Debug, PartialEq)]
pub enum ParameterFormat {
    Normal,
    Length,
    Default(Word),
    Assign(Word),
    Error(Word),
    Alt(Word),
    Sub(ParamSubSide, ParamSubMode, Word),
}

#[derive(Debug, PartialEq)]
pub enum ParamSubMode {
    Shortest, Longest
}

#[derive(Debug, PartialEq)]
pub enum ParamSubSide {
    Prefix, Suffix
}

//<<<<>>>><<>><><<>><<<*>>><<>><><<>><<<<>>>>\\

#[derive(Debug, PartialEq)]
pub struct Redirection {
    redirection_type: RedirectionType,
    fd: u64,
    target: Word
}

#[derive(Debug, PartialEq)]
pub enum RedirectionType {
    File(FileRedirectionType),
    Dup(DupRedirectionType),
    Heredoc // '<<'
}

#[derive(Debug, PartialEq)]
pub enum FileRedirectionType {
    In,         // '<'
    InOut,      // '<>'
    Out,        // '>'
    OutReplace, // '>|'
    OutAppend,  // '>>'
}

#[derive(Debug, PartialEq)]
pub enum DupRedirectionType {
    In,  // '<&'
    Out  // '>&'
}

//<<<<>>>><<>><><<>><<<*>>><<>><><<>><<<<>>>>\\
