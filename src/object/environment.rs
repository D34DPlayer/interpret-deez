use super::builtins::Builtin;
use super::Object;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Default, Debug, PartialEq, Clone)]
pub struct Environment {
    pub store: HashMap<String, Object>,
    outer: Option<HeapEnvironment>,
}

pub type HeapEnvironment = Rc<RefCell<Environment>>;

impl Environment {
    pub fn new(outer: Option<HeapEnvironment>) -> Self {
        let outer = match outer {
            Some(_) => outer,
            None => Some(Builtin::environment()),
        };

        Self {
            store: HashMap::new(),
            outer: outer,
        }
    }

    pub fn new_heap(outer: Option<HeapEnvironment>) -> HeapEnvironment {
        Rc::new(RefCell::new(Self::new(outer)))
    }

    pub fn get(&self, k: &str) -> Option<Object> {
        if let Some(o) = self.store.get(k) {
            Some(o.clone())
        } else {
            if let Some(outer) = &self.outer {
                outer.borrow().get(k)
            } else {
                None
            }
        }
    }

    pub fn set(&mut self, k: &str, v: Object) -> Option<Object> {
        self.store.insert(k.to_string(), v)
    }

    pub fn remove(&mut self, k: &str) -> Option<Object> {
        match self.store.remove(k) {
            None => match &self.outer {
                Some(outer) => outer.borrow_mut().remove(k),
                None => None,
            },
            x => x,
        }
    }
}
