use std::{
    collections::HashMap,
    boxed::Box
};

use crate::sh::ast::Command;
use laddertypes::*;

pub struct Substitution(HashMap< String, CommandTypeExpr >);

#[derive(Clone)]
pub enum CommandArgPattern {
    Literal(String),
    Variable(String),
    VariablePack(Box<CommandArgPattern>),
    Optional(Box<CommandArgPattern>),
    Conjunction(Vec<CommandArgPattern>),
    Disjunction(Vec<CommandArgPattern>)
}

#[derive(Clone)]
pub struct CommandPattern {
    name: String,
    args: Vec<CommandArgPattern>,
    env: Vec<(String, CommandTypeExpr)>,
}

#[derive(Clone)]
pub struct MatchCandidate {
    at: usize,
    expected: CommandPattern,
    found: CommandTypeExpr,
}

#[derive(Clone)]
pub struct UnificationError( Vec<MatchCandidate> );

#[derive(Clone)]
pub enum CommandTypeExpr {
    Type(TypeTerm),
    Match(Box<CommandTypeExpr>, Vec<(CommandArgPattern, CommandTypeExpr)>)
}

impl CommandArgPattern {
    pub fn match_cmd(&self, cmd: &Command) -> Result<Substitution, UnificationError> {
        Err(UnificationError(vec![]))
    }
}

impl CommandTypeExpr {
    pub fn eval(self) -> Result<TypeTerm, CommandTypeExpr> {
        match self {
            CommandTypeExpr::Type(typ) => Ok(typ),
            CommandTypeExpr::Match(pattern, cases) => {
                
            }
            s=> Ok(s)
        }
    }

    pub fn apply_subst(&mut self, subst: &Substitution) {
        match self {
            CommandTypeExpr::Type(typ) => {
                self = CommandTypeExpr::Type(
                    typ.apply_substitution(|v: String| subst.get(v))
                );
            }
            CommandTypeExpr::Match( pattern, cases ) => {
                
                // todo
            }
            _ => {}
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

