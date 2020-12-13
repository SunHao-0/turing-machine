use crate::tm::{HeadDirection, State, Symbol, TM};
use std::fmt;
use std::rc::Rc;

pub struct Runner<'a> {
    left_tape: Vec<Symbol>,
    right_tape: Vec<Symbol>,
    head: HeadPosition,
    current_state: State,
    tm: &'a TM,
    runner_state: RunnerState,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum RunnerState {
    Hungry,
    Running,
    Accept,
    Reject,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum HeadPosition {
    Left(usize),
    Right(usize),
}

impl Default for HeadPosition {
    fn default() -> Self {
        HeadPosition::Right(0)
    }
}

impl<'a> Runner<'a> {
    pub fn with_tm(tm: &'a TM) -> Self {
        Self {
            left_tape: vec![tm.empty_sym],
            right_tape: Vec::new(),
            head: HeadPosition::default(),
            current_state: Rc::clone(&tm.start_state),
            tm,
            runner_state: RunnerState::Hungry,
        }
    }

    fn reset(&mut self) {
        // just create a new runner, maybe optimize latter.
        *self = Self::with_tm(self.tm);
    }

    pub fn feed_str<T: AsRef<str>>(&mut self, input_str: T) {
        if self.runner_state != RunnerState::Hungry {
            self.reset();
        }
        self.right_tape = input_str.as_ref().chars().collect();
        self.right_tape.push(self.tm.empty_sym);
        self.runner_state = RunnerState::Running;
    }

    pub fn step(&mut self) -> RunnerState {
        use RunnerState::*;

        match self.runner_state {
            Running => self.do_transfer(),
            _ => self.runner_state,
        }
    }

    fn get_sym(&mut self) -> char {
        let (tape, pos) = self.get_tape_pos_mut();
        tape[pos]
    }

    fn write_sym(&mut self, sym: Symbol) {
        let (tape, pos) = self.get_tape_pos_mut();
        tape[pos] = sym;
    }

    fn get_tape_pos_mut(&mut self) -> (&mut [Symbol], usize) {
        let (tape, pos) = match self.head {
            HeadPosition::Left(pos) => (&mut self.left_tape, pos),
            HeadPosition::Right(pos) => (&mut self.right_tape, pos),
        };
        assert!(pos <= tape.len());
        if pos == tape.len() {
            tape.push(self.tm.empty_sym);
        }
        (tape, pos)
    }

    fn mv_head(&mut self, dir: HeadDirection) {
        match &mut self.head {
            HeadPosition::Left(pos) => match dir {
                HeadDirection::Left => *pos += 1,
                HeadDirection::Right => {
                    if *pos == 0 {
                        self.head = HeadPosition::Right(0);
                    } else {
                        *pos -= 1;
                    }
                }
                HeadDirection::Stop => (),
            },
            HeadPosition::Right(pos) => match dir {
                HeadDirection::Right => *pos += 1,
                HeadDirection::Left => {
                    if *pos == 0 {
                        self.head = HeadPosition::Left(0);
                    } else {
                        *pos -= 1;
                    }
                }
                HeadDirection::Stop => (),
            },
        }
    }

    fn do_transfer(&mut self) -> RunnerState {
        let tape_sym = self.get_sym();
        if let Some((next_state, next_sym, mv_dir)) = self.tm.transfer(
            Rc::clone(&self.current_state), /*TODO fix this*/
            tape_sym,
        ) {
            self.current_state = next_state;
            if let Some(sym) = next_sym {
                self.write_sym(sym);
            }
            self.mv_head(mv_dir);
            if self.tm.accept(&self.current_state) {
                self.runner_state = RunnerState::Accept;
            }
        } else {
            self.runner_state = RunnerState::Reject;
        }
        self.runner_state
    }

    pub fn ir(&self) -> IR {
        IR {
            head: self.head,
            left_tape: &self.left_tape,
            right_tape: &self.right_tape,
            current_state: Rc::clone(&self.current_state),
            runner_state: self.runner_state,
            empty_sym: self.tm.empty_sym,
        }
    }
}

// TODO add more methods.
pub struct IR<'a> {
    head: HeadPosition,
    left_tape: &'a [Symbol],
    right_tape: &'a [Symbol],
    current_state: State,
    empty_sym: Symbol,
    runner_state: RunnerState,
}

impl IR<'_> {
    pub fn tape_str(&self) -> String {
        self.left_tape
            .iter()
            .copied()
            .rev()
            .chain(self.right_tape.iter().copied())
            .collect::<String>()
    }
}

impl fmt::Display for IR<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // write state first.
        writeln!(f, "{:?}", self.runner_state)?;

        let syms = self
            .left_tape
            .iter()
            .copied()
            .rev()
            .chain(self.right_tape.iter().copied())
            .collect::<Vec<_>>();
        let state_pos = match self.head {
            HeadPosition::Left(pos) => {
                let pos = self.left_tape.len() - pos;
                if pos != 0 {
                    pos - 1
                } else {
                    pos
                }
            }
            HeadPosition::Right(pos) => self.left_tape.len() + pos,
        };

        for (pos, sym) in syms.iter().copied().enumerate() {
            if pos == state_pos {
                if pos == 0 {
                    write!(f, "{}", self.empty_sym)?;
                }
                write!(f, "<{}>", self.current_state)?;
            }
            write!(f, "{}", sym)?;
        }

        if state_pos == syms.len() {
            write!(f, "<{}>{}", self.current_state, self.empty_sym)?;
        }
        Ok(())
    }
}
