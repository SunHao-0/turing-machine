#[macro_use]
extern crate pest_derive;

pub mod parse;
pub mod runner;
pub mod tm;

pub use parse::*;
pub use runner::*;
pub use tm::*;

pub fn tm_foo() -> TM {
    TMBuilder::new()
        .start_state("q0")
        .accept_state("q1")
        .sym('a')
        .transfer_fns(vec![
            TransferFnItem::new()
                .from("q0", 'a')
                .to("q0", None, HeadDirection::Right),
            TransferFnItem::new()
                .from("q0", 'B')
                .to("q1", None, HeadDirection::Left),
        ])
        .build()
        .unwrap()
}

pub fn tm_bar() -> TM {
    TMBuilder::new()
        .start_state("q0")
        .states(vec!["q1", "q2", "q3", "q4"])
        .accept_state("q4")
        .syms(vec!['a', 'b'])
        .empty_sym('B')
        .tape_syms(vec!['x', 'y'])
        .transfer_fns(vec![
            TransferFnItem::new()
                .from("q0", 'a')
                .to("q1", Some('x'), HeadDirection::Right),
            TransferFnItem::new()
                .from("q0", 'y')
                .to("q3", None, HeadDirection::Right),
            TransferFnItem::new()
                .from("q1", 'a')
                .to("q1", None, HeadDirection::Right),
            TransferFnItem::new()
                .from("q1", 'y')
                .to("q1", None, HeadDirection::Right),
            TransferFnItem::new()
                .from("q1", 'b')
                .to("q2", Some('y'), HeadDirection::Left),
            TransferFnItem::new()
                .from("q2", 'a')
                .to("q2", None, HeadDirection::Left),
            TransferFnItem::new()
                .from("q2", 'y')
                .to("q2", None, HeadDirection::Left),
            TransferFnItem::new()
                .from("q2", 'x')
                .to("q0", None, HeadDirection::Right),
            TransferFnItem::new()
                .from("q3", 'y')
                .to("q3", None, HeadDirection::Right),
            TransferFnItem::new()
                .from("q3", 'B')
                .to("q4", None, HeadDirection::Left),
        ])
        .build()
        .unwrap()
}
