use std::io::{stdin, stdout, Write};
use turing_machine::*;

pub fn tm_multiply() -> TM {
    TMBuilder::new()
        .start_state("q0")
        .accept_state("q12")
        .states(vec![
            "q1", "q2", "q3", "q4", "q5", "q6", "q7", "q8", "q9", "q10", "q10", "q11",
        ])
        .syms(vec!['0', '1'])
        .tape_sym('X')
        .empty_sym('B')
        .transfer_fns(vec![
            TransferFnItem::new()
                .from("q0", '0')
                .to("q6", Some('B'), HeadDirection::Right),
            TransferFnItem::new()
                .from("q6", '0')
                .to("q6", None, HeadDirection::Right),
            TransferFnItem::new()
                .from("q6", '1')
                .to("q1", None, HeadDirection::Right),
            TransferFnItem::new()
                .from("q1", '0')
                .to("q2", Some('X'), HeadDirection::Right),
            TransferFnItem::new()
                .from("q2", '1')
                .to("q2", None, HeadDirection::Right),
            TransferFnItem::new()
                .from("q2", '0')
                .to("q2", None, HeadDirection::Right),
            TransferFnItem::new()
                .from("q2", 'B')
                .to("q3", Some('0'), HeadDirection::Left),
            TransferFnItem::new()
                .from("q3", '1')
                .to("q3", None, HeadDirection::Left),
            TransferFnItem::new()
                .from("q3", '0')
                .to("q3", None, HeadDirection::Left),
            TransferFnItem::new()
                .from("q3", 'X')
                .to("q1", None, HeadDirection::Right),
            TransferFnItem::new()
                .from("q1", '1')
                .to("q4", None, HeadDirection::Left),
            TransferFnItem::new()
                .from("q4", 'X')
                .to("q4", Some('0'), HeadDirection::Left),
            TransferFnItem::new()
                .from("q4", '1')
                .to("q5", None, HeadDirection::Right),
            TransferFnItem::new()
                .from("q5", '0')
                .to("q7", None, HeadDirection::Left),
            TransferFnItem::new()
                .from("q7", '1')
                .to("q8", None, HeadDirection::Left),
            TransferFnItem::new()
                .from("q8", '0')
                .to("q9", None, HeadDirection::Left),
            TransferFnItem::new()
                .from("q8", 'B')
                .to("q10", None, HeadDirection::Right),
            TransferFnItem::new()
                .from("q9", '0')
                .to("q9", None, HeadDirection::Left),
            TransferFnItem::new()
                .from("q9", 'B')
                .to("q0", None, HeadDirection::Right),
            TransferFnItem::new()
                .from("q10", '1')
                .to("q11", Some('B'), HeadDirection::Right),
            TransferFnItem::new()
                .from("q11", '0')
                .to("q11", Some('B'), HeadDirection::Right),
            TransferFnItem::new()
                .from("q11", '1')
                .to("q12", Some('B'), HeadDirection::Right),
        ])
        .build()
        .unwrap()
}

fn main() {
    let tm = tm_multiply();
    let mut n1 = String::new();
    let mut n2 = String::new();
    print!("INPUT n1.\n>>");
    stdout().flush().unwrap();
    stdin().read_line(&mut n1).unwrap();
    print!("INPUT n2.\n>>");
    stdout().flush().unwrap();
    stdin().read_line(&mut n2).unwrap();

    let n1: u8 = n1.trim().parse().unwrap();
    let n2: u8 = n2.trim().parse().unwrap();
    let input_str = format!("{}1{}1", "0".repeat(n1 as usize), "0".repeat(n2 as usize));
    let mut runner = Runner::with_tm(&tm);
    println!("====== INPUT STR ======");
    println!("{}", input_str);

    runner.feed_str(&input_str);
    while runner.step() == RunnerState::Running {}
    let tape_str = runner.ir().tape_str();
    let result = tape_str.trim_matches('B').len();
    println!("====== RESULT ======");
    println!("{}", num_to_text(result as u16));
}

const NUM_TEXTS: [&str; 10] = [
    r#"
 .----------------. 
 | .--------------. |
 | |     ____     | |
 | |   .'    '.   | |
 | |  |  .--.  |  | |
 | |  | |    | |  | |
 | |  |  `--'  |  | |
 | |   '.____.'   | |
 | |              | |
 | '--------------' |
  '----------------' 
  "#,
    r#"
 .----------------. 
 | .--------------. |
 | |     __       | |
 | |    /  |      | |
 | |    `| |      | |
 | |     | |      | |
 | |    _| |_     | |
 | |   |_____|    | |
 | |              | |
 | '--------------' |
  '----------------' 
  "#,
    r#"
 .----------------. 
 | .--------------. |
 | |    _____     | |
 | |   / ___ `.   | |
 | |  |_/___) |   | |
 | |   .'____.'   | |
 | |  / /____     | |
 | |  |_______|   | |
 | |              | |
 | '--------------' |
  '----------------' 
  "#,
    r#"
 .----------------. 
 | .--------------. |
 | |    ______    | |
 | |   / ____ `.  | |
 | |   `'  __) |  | |
 | |   _  |__ '.  | |
 | |  | \____) |  | |
 | |   \______.'  | |
 | |              | |
 | '--------------' |
  '----------------' 
  "#,
    r#"
 .----------------. 
 | .--------------. |
 | |   _    _     | |
 | |  | |  | |    | |
 | |  | |__| |_   | |
 | |  |____   _|  | |
 | |      _| |_   | |
 | |     |_____|  | |
 | |              | |
 | '--------------' |
  '----------------' 
  "#,
    r#"
 .----------------. 
 | .--------------. |
 | |   _______    | |
 | |  |  _____|   | |
 | |  | |____     | |
 | |  '_.____''.  | |
 | |  | \____) |  | |
 | |   \______.'  | |
 | |              | |
 | '--------------' |
  '----------------' 
  "#,
    r#"
 .----------------. 
 | .--------------. |
 | |    ______    | |
 | |  .' ____ \   | |
 | |  | |____\_|  | |
 | |  | '____`'.  | |
 | |  | (____) |  | |
 | |  '.______.'  | |
 | |              | |
 | '--------------' |
  '----------------' 
  "#,
    r#"
 .----------------. 
 | .--------------. |
 | |   _______    | |
 | |  |  ___  |   | |
 | |  |_/  / /    | |
 | |      / /     | |
 | |     / /      | |
 | |    /_/       | |
 | |              | |
 | '--------------' |
  '----------------' 
  "#,
    r#"
 .----------------. 
 | .--------------. |
 | |     ____     | |
 | |   .' __ '.   | |
 | |   | (__) |   | |
 | |   .`____'.   | |
 | |  | (____) |  | |
 | |  `.______.'  | |
 | |              | |
 | '--------------' |
  '----------------' 
  "#,
    r#"
 .----------------. 
 | .--------------. |
 | |    ______    | |
 | |  .' ____ '.  | |
 | |  | (____) |  | |
 | |  '_.____. |  | |
 | |  | \____| |  | |
 | |   \______,'  | |
 | |              | |
 | '--------------' |
  '----------------' 
  "#,
];

pub fn num_to_text(mut n: u16) -> String {
    let text_height: usize = NUM_TEXTS[0].lines().count();

    let mut ns = Vec::new();
    while n != 0 {
        let t = n % 10;
        n -= t;
        if n != 0 {
            n /= 10;
        }
        ns.push(t);
    }
    ns.reverse();
    let ns_text = ns
        .iter()
        .map(|n| NUM_TEXTS[*n as usize].lines().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut h = 0;
    let mut ret = String::new();
    while h != text_height {
        for t in ns_text.iter() {
            ret += t[h].trim_matches('\n');
        }
        if h + 1 != text_height {
            ret += "\n"
        }
        h += 1;
    }
    ret
}
