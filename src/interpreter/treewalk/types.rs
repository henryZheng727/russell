use std::{collections::HashMap, rc::Rc};

pub(super) struct Env {
    parent: Option<Rc<Env>>,
    symbols: HashMap<String, Value>,
}

impl Env {
    fn lookup(&self, key: &str) -> Option<&Value> {
        if self.symbols.contains_key(key) {
            self.symbols.get(key)
        } else {
            match &self.parent {
                Some(p) => p.lookup(key),
                None => None,
            }
        }
    }

    fn define(&mut self, key: String, value: Value) {
        self.symbols.insert(key, value);
    }
}

pub(super) enum Value {
    Int,
    Float,
    Bool,
    Closure,
}
