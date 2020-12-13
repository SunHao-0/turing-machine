use turing_machine::*;

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
use turing_machine::*;

fn main() {
    let tm = tm_foo();
    let mut runner = Runner::with_tm(&tm);
    runner.feed_str("aaaaaabbbbbb");
    loop {
        let ir = runner.ir();
        println!("{}", ir);
        if runner.step() != RunnerState::Running {
            break;
        }
    }
    println!("{}", runner.ir())
}
