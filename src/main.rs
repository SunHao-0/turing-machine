use std::process::exit;
use std::{env, fs::read_to_string};
use turing_machine::*;

fn main() {
    let mut args = env::args().skip(1).collect::<Vec<_>>();
    if args.len() < 2 {
        eprintln!("turing-machine  [-v] file input");
        exit(1)
    }

    let mut verbose = false;
    if args.len() == 3 && args[0] == "-v" {
        verbose = true;
        args.remove(0);
    }

    let tm_file = &args[0];
    let input = &args[1];

    let tm_def = read_to_string(&tm_file).unwrap_or_else(|e| {
        eprintln!("Error: failed to read {}: {}", tm_file, e);
        exit(1)
    });
    let tm = parse(&tm_def).unwrap_or_else(|e| {
        eprintln!("Error: invalid defination of turing machine.\n{}", e);
        exit(1)
    });

    let mut runner = Runner::with_tm(&tm);
    runner.feed_str(input);
    while runner.step() == RunnerState::Running {
        if verbose {
            println!("{}", runner.ir());
        }
    }
    println!("{}", runner.ir());
}