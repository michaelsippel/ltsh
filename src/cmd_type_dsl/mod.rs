use std::{
    collections::HashMap,
    boxed::Box
};

use crate::ast::Command;
use laddertypes::*;


pub struct Substitution(HashMap< String, CommandTypeExpr >);
impl Substitution {
   pub fn apply(&self, expr: &mut CommandTypeExpr) {
        
   }
}

pub enum CommandArgPattern {
    Literal(String),
    Variable(String),
    VariablePack(Box<CommandArgPattern>),
    Optional(Box<CommandArgPattern>),
    Conjunction(Vec<CommandArgPattern>),
    Disjunction(Vec<CommandArgPattern>)
}

pub struct CommandPattern {
    name: String,
    args: Vec<CommandArgPattern>,
    env: Vec<(String, CommandTypeExpr)>,
}

impl CommandArgPattern {
    pub fn match_cmd(&self, cmd: &Command) -> Result<Substitution, UnificationError> {
        Err(UnificationError(vec![]))
    }
}

pub struct MatchCandidate {
    at: usize,
    expected: CommandPattern,
    found: CommandTypeExpr,
}

pub struct UnificationError( Vec<MatchCandidate> );



pub enum CommandTypeExpr {
    Parameter(String),
    ParameterPack(String),
    Char(char),
    Match(Box<CommandTypeExpr>, Vec<(CommandArgPattern, CommandTypeExpr)>)
}

impl CommandTypeExpr {
    pub fn eval(self) -> CommandTypeExpr {
        match self {
            s=>s
        }
    }
}

pub struct FileDescriptor(u32);
pub enum PipeDirection { In, Out }

pub enum Selector {
    Pipe(FileDescriptor, PipeDirection),
    Parameter(String),
    ParameterPack(String),
    File(String)
}

pub enum CommandTypeStatement {
    TypAssign(Selector, TypeTerm),
    ValAssign(String, CommandTypeExpr),
    Block(Vec<CommandTypeStatement>),
    Match(Box<CommandTypeExpr>, Vec<(CommandArgPattern, CommandTypeStatement)>)
}

pub struct CommandType(Vec<(Selector, TypeTerm)>);

impl CommandTypeStatement {
    pub fn eval(self) -> CommandType {
       match self {
            CommandTypeStatement::Block(stmnts) => {
                CommandType( stmnts.into_iter().map(|stmnt| stmnt.eval().0.into_iter()).flatten().collect() )
            }
            CommandTypeStatement::TypAssign(selector, typ) => {
                CommandType( vec![ (selector, typ) ])
            }
            CommandTypeStatement::ValAssign(variable, expr) => {
                CommandType(vec![])
            }
            CommandTypeStatement::Match(pattern, cases) => {
                    /*
                    for (case,stmnt) in cases.into_iter() {
                        if let Ok(unificator) = pattern
                        if let Ok() = case.match_expr()
                        CommandType( vec![] )
                    }
                        */
                CommandType(vec![])
            }
        }
    }
}

