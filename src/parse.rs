use crate::TM;
use crate::{
    tm::{Symbol, TMBuilder, TransferFnItem},
    HeadDirection,
};

use pest::{iterators::Pair, Parser};
use rustc_hash::FxHashSet;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("parse: {0}")]
    Parse(#[from] pest::error::Error<Rule>),
    #[error("semanic: {0}")]
    Semantic(String),
}

pub fn parse<T: AsRef<str>>(content: T) -> Result<TM, Error> {
    let content = content.as_ref();
    let mut pt = TMParser::parse(Rule::ROOT, content)?;

    // states set
    let mut decl_states = false;
    let mut states = if let Rule::StateSet = pt.peek().unwrap().as_rule() {
        decl_states = true;
        parse_state_set(pt.next().unwrap())
    } else {
        FxHashSet::default()
    };

    // symbol set
    let sym_set = parse_symbol_set(pt.next().unwrap());

    // tape symbol set
    let mut decl_tsym = false;
    let mut tsym_set = if let Rule::TapeSymbolSet = pt.peek().unwrap().as_rule() {
        decl_tsym = true;
        parse_symbol_set(pt.next().unwrap())
    } else {
        FxHashSet::default()
    };

    // transfer fn set
    let mut fns = Vec::new();
    let mut start = String::from("q0");
    for (i, p) in pt.next().unwrap().into_inner().enumerate() {
        let f = parse_fn(p)?;
        if !decl_tsym {
            let (sym0, sym1) = f.syms();
            tsym_set.insert(sym0);
            tsym_set.insert(sym1);
        }
        let (s0, s1) = f.states();
        if !decl_states {
            states.insert(s0.to_string());
            states.insert(s1.to_string());
        }
        if i == 0 {
            start = s0.to_string();
        }
        fns.push(f);
    }

    if let Rule::Start = pt.peek().unwrap().as_rule() {
        let start_state = pt.next().unwrap().into_inner().next().unwrap().as_str();
        start = start_state.to_string();
    }

    let mut final_set = FxHashSet::default();
    for i in pt.next().unwrap().into_inner() {
        final_set.insert(i.as_str());
    }

    let mut empty = 'B';
    if let Some(p) = pt.next() {
        if p.as_rule() == Rule::Empty {
            empty = parse_symbol(p.into_inner().next().unwrap());
        }
    }
    TMBuilder::new()
        .states(states)
        .syms(sym_set)
        .tape_syms(tsym_set)
        .start_state(start)
        .accept_states(final_set)
        .empty_sym(empty)
        .transfer_fns(fns)
        .build()
        .map_err(Error::Semantic)
}

fn parse_state_set(p: Pair<Rule>) -> FxHashSet<String> {
    let mut s = FxHashSet::default();
    for p in p.into_inner() {
        s.insert(parse_ident(p));
    }
    s
}

fn parse_symbol_set(p: Pair<Rule>) -> FxHashSet<Symbol> {
    let mut s = FxHashSet::default();
    for p in p.into_inner() {
        s.insert(parse_symbol(p));
    }
    s
}

fn parse_fn(p: Pair<Rule>) -> Result<TransferFnItem, Error> {
    let mut p = p.into_inner();
    let from_s = parse_ident(p.next().unwrap());
    let from_sym = parse_symbol(p.next().unwrap());
    let to_s = parse_ident(p.next().unwrap());
    let to_sym = parse_symbol(p.next().unwrap());
    let dir = parse_dir(p.next().unwrap())?;
    Ok(TransferFnItem::new()
        .from(from_s, from_sym)
        .to(to_s, Some(to_sym), dir))
}

fn parse_dir(p: Pair<Rule>) -> Result<HeadDirection, Error> {
    let dir = p.as_str();
    if dir == "L" {
        Ok(HeadDirection::Left)
    } else if dir == "R" {
        Ok(HeadDirection::Right)
    } else if dir == "S" {
        Ok(HeadDirection::Stop)
    } else {
        Err(Error::Semantic(format!("invalid head direction: {}", dir)))
    }
}

fn parse_ident(p: Pair<Rule>) -> String {
    p.as_str().to_string()
}

fn parse_symbol(p: Pair<Rule>) -> Symbol {
    p.as_str().chars().next().unwrap()
}

#[derive(Parser)]
#[grammar = "turing.pest"]
struct TMParser;
