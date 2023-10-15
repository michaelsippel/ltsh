//<<<<>>>><<>><><<>><<<*>>><<>><><<>><<<<>>>>\\

#[derive(Debug)]
pub enum Command {
    Simple {
        assignments: Vec<(String, Word)>,
        command_word: Word,
        redirections: Vec<Redirection>
    },
    Pipeline(Vec<Command>),
    Sequence(Vec<Command>),
    ShortCircuitConjection(Vec<Command>),
    ShortCircuitDisjunction(Vec<Command>),
    Negation(Command),
    While {
        condition: Command,
        loop_body: Command
    },
    For {
        varname: String,
        sequence: Word,
        loop_body: Command
    }
    If {
        condition: Command,
        then_branch: Command,
        else_branch: Command
    },
    Case {
        expr: Word,
        cases: Vec<(Word, Command)>
    },
    Function {
        name: String,
        body: Command
    }
}

/*
 * We are all luminous beings.
 * Why then, do we not appear before each
 * other radiant in our illumination ?
 */

/*
 * Bewteen the idea
 * And the reality
 * Between the motion
 * And the act
 * Falls the Shadow
 * (T.S. Eliot)
 */

//<<<<>>>><<>><><<>><<<*>>><<>><><<>><<<<>>>>\\

#[derive(Debug)]
pub struct Word {
    pub segments: Vec<WordSegment>
}

#[derive(Debug)]
pub enum WordSegment {
    FieldSeparator,
    Tilde(String),
    Literal(String),
    Parameter(String, ParameterFormat),
    Subshell(Command),
    DoubleQuote(Word),
}

#[derive(Debug)]
pub enum ParameterFormat {
    Normal,
    Length,
    Default(Word),
    Assign(Word),
    Error(Word),
    Alt(Word),
    Sub(ParamSubSide, ParamSubMode, Word),
}

#[derive(Debug)]
pub enum ParamSubMode {
    Shortest, Longest
}

#[derive(Debug)]
pub enum ParamSubSide {
    Prefix, Suffix
}

//<<<<>>>><<>><><<>><<<*>>><<>><><<>><<<<>>>>\\

#[derive(Debug)]
pub struct Redirection {
    redirection_type: RedirectionType,
    fd: u64,
    target: Word
}

#[derive(Debug)]
pub enum RedirectionType {
    File(FileRedirectionType),
    Dup(DupRedirectionType),
    Heredoc // '<<'
}

#[derive(Debug)]
pub enum FileRedirectionType {
    In,         // '<'
    InOut,      // '<>'
    Out,        // '>'
    OutReplace, // '>|'
    OutAppend,  // '>|'
}

#[derive(Debug)]
pub enum DupRedirectionType {
    In,  // '<&'
    Out  // '>&'
}

//<<<<>>>><<>><><<>><<<*>>><<>><><<>><<<<>>>>\\
