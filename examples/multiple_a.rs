use std::process::exit;
use turing_machine::*;

pub fn tm_foo() -> TM {
    let tm = "
    // StateSet={q0, q1}; ignore state set
    SymbolSet = {a};
    // TSymbolSet = {a, B}; ignore tape symbol set.
    FnSet = {
        (q0, a) -> (q0, a, R),
        (q0, B) -> (q1, B, L)
    };
    // ignore start state
    // Start = q0;
    FinalSet = {q1};
    // Empty = B;
    ";
    parse(tm).unwrap_or_else(|e| {
        eprintln!("{}", tm.escape_debug());
        eprintln!("{}", e);
        exit(1)
    })
}

fn main() {
    let tm = tm_foo();
    let mut runner = Runner::with_tm(&tm);
    runner.feed_str("aaaaaa");
    loop {
        let ir = runner.ir();
        println!("{}", ir);
        if runner.step() != RunnerState::Running {
            break;
        }
    }
    println!("{}", runner.ir())
}
