use rustc_hash::{FxHashMap, FxHashSet};
use std::iter::IntoIterator;
use std::rc::Rc;

/// State of a turing machine, e.g. p0, p1.
pub type State = Rc<str>;

/// Symbol of a turing machine.
pub type Symbol = char;

/// Definition of a truing machine. Inmutable.
#[allow(dead_code)]
pub struct TM {
    states: FxHashSet<State>,
    pub(crate) start_state: State,
    accept_states: FxHashSet<State>,
    syms: FxHashSet<Symbol>,
    tape_syms: FxHashSet<Symbol>,
    pub(crate) empty_sym: Symbol,
    transfer_fn: TransferFn,
}

impl TM {
    pub fn transfer(
        &self,
        s: State,
        sym: Symbol,
    ) -> Option<(State, Option<Symbol>, HeadDirection)> {
        self.transfer_fn.transfer(s, sym)
    }

    pub fn accept<T: AsRef<str>>(&self, s: T) -> bool {
        self.accept_states.contains(s.as_ref())
    }
}

/// Moving Direction of a turing machine's tape head.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum HeadDirection {
    Left,
    Right,
    Stop,
}

/// Tranfer function of a turing machine.
pub struct TransferFn(FxHashMap<(State, Symbol), (State, Option<Symbol>, HeadDirection)>);

impl TransferFn {
    pub fn transfer(
        &self,
        s: State,
        sym: Symbol,
    ) -> Option<(State, Option<Symbol>, HeadDirection)> {
        if let Some((s, sym, head)) = self.0.get(&(s, sym)) {
            Some((Rc::clone(s), *sym, *head))
        } else {
            None
        }
    }

    pub fn tranfer_uncheck(&self, s: State, sym: Symbol) -> (State, Option<Symbol>, HeadDirection) {
        self.transfer(s, sym).unwrap()
    }

    fn add_item(&mut self, item: TransferFnItem, states: &FxHashSet<State>) {
        let (s0, sym0) = item.from.unwrap();
        let (s1, sym1, dir) = item.to.unwrap();
        let s0 = Rc::clone(states.get(&s0[..]).unwrap()); // TODO fix this
        let s1 = Rc::clone(states.get(&s1[..]).unwrap()); // TODO fix this
        self.0.insert((s0, sym0), (s1, sym1, dir));
    }

    fn add_items<T: IntoIterator<Item = TransferFnItem>>(
        &mut self,
        items: T,
        states: &FxHashSet<State>,
    ) {
        for item in items {
            self.add_item(item, states);
        }
    }
}

/// Temporay stroage for transfor function item.
#[derive(Default)]
pub struct TransferFnItem {
    from: Option<(String, Symbol)>,
    to: Option<(String, Option<Symbol>, HeadDirection)>,
}

impl TransferFnItem {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from<T: Into<String>>(mut self, s: T, sym: Symbol) -> Self {
        self.from = Some((s.into(), sym));
        self
    }

    pub fn to<T: Into<String>>(mut self, s: T, sym: Option<Symbol>, dir: HeadDirection) -> Self {
        self.to = Some((s.into(), sym, dir));
        self
    }
}

#[derive(Default)]
pub struct TMBuilder {
    states: FxHashSet<String>,
    start_state: Option<String>,
    accept_states: FxHashSet<String>,
    syms: FxHashSet<Symbol>,
    tape_syms: FxHashSet<Symbol>,
    empty_sym: Option<Symbol>,
    transfer_fn_items: Vec<TransferFnItem>,
}

impl TMBuilder {
    pub fn new() -> Self {
        TMBuilder::default()
    }

    pub fn state<T: Into<String>>(mut self, s: T) -> Self {
        self.states.insert(s.into());
        self
    }

    pub fn states<T: IntoIterator<Item = F>, F: Into<String>>(mut self, s: T) -> Self {
        self.states.extend(s.into_iter().map(|s| s.into()));
        self
    }

    pub fn start_state<T: Into<String>>(mut self, s: T) -> Self {
        let s = s.into();
        self = self.state(s.clone());
        self.start_state = Some(s);
        self
    }

    pub fn accept_state<T: Into<String>>(mut self, s: T) -> Self {
        let s = s.into();
        self = self.state(s.clone());
        self.accept_states.insert(s);
        self
    }

    pub fn accept_states<T: IntoIterator<Item = F>, F: Into<String>>(mut self, s: T) -> Self {
        for state in s {
            self = self.accept_state(state);
        }
        self
    }

    pub fn sym(mut self, s: Symbol) -> Self {
        self.syms.insert(s);
        self.tape_syms.insert(s);
        self
    }

    pub fn syms<T: IntoIterator<Item = Symbol>>(mut self, s: T) -> Self {
        for sy in s {
            self = self.sym(sy);
        }
        self
    }

    pub fn tape_sym(mut self, s: Symbol) -> Self {
        self.tape_syms.insert(s);
        self
    }

    pub fn tape_syms<T: IntoIterator<Item = Symbol>>(mut self, s: T) -> Self {
        self.tape_syms.extend(s);
        self
    }

    pub fn empty_sym(mut self, s: Symbol) -> Self {
        self.empty_sym = Some(s);
        self.tape_syms.insert(s);
        self
    }

    pub fn transfer_fn(mut self, f: TransferFnItem) -> Self {
        self.transfer_fn_items.push(f);
        self
    }

    pub fn transfer_fns<T: IntoIterator<Item = TransferFnItem>>(mut self, fs: T) -> Self {
        self.transfer_fn_items.extend(fs);
        self
    }

    /// Try to build the tm, just return str if anything bad happened.
    pub fn build(self) -> Result<TM, String> {
        let states = self
            .states
            .into_iter()
            .map(Rc::from)
            .collect::<FxHashSet<Rc<str>>>();
        let start_state = self.start_state.ok_or("No start state specified")?;
        let start_state = Rc::clone(
            states
                .get(&start_state[..])
                .ok_or("Start state not in states set")?,
        );
        let mut accept_states = FxHashSet::default();
        for accept_state in self.accept_states.into_iter() {
            let s = states
                .get(&accept_state[..])
                .ok_or("Accept state not on states set")?;
            accept_states.insert(Rc::clone(s));
        }

        let mut fns = TransferFn(FxHashMap::default());
        fns.add_items(self.transfer_fn_items, &states);
        Ok(TM {
            start_state,
            accept_states,
            states,
            syms: self.syms,
            tape_syms: self.tape_syms,
            empty_sym: self.empty_sym.unwrap_or('B'),
            transfer_fn: fns,
        })
    }
}
