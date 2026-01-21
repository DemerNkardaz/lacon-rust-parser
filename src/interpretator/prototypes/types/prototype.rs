use crate::interpretator::prototypes::types::object::object::ObjectRef;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

/// Умный указатель для прототипа, чтобы несколько объектов могли ссылаться на один чертеж.
pub type PrototypeRef = Rc<RefCell<Prototype>>;

pub struct Prototype {
    pub name: String,
    /// Ссылка на родительский прототип для реализации цепочки наследования.
    pub parent: Option<PrototypeRef>,
    /// Таблица методов или констант, принадлежащих прототипу.
    pub properties: HashMap<String, ObjectRef>,
}

impl Prototype {
    pub fn new(name: &str, parent: Option<PrototypeRef>) -> PrototypeRef {
        Rc::new(RefCell::new(Prototype {
            name: name.to_string(),
            parent,
            properties: HashMap::new(),
        }))
    }

    /// Поиск свойства вверх по цепочке прототипов.
    pub fn lookup(&self, key: &str) -> Option<ObjectRef> {
        if let Some(val) = self.properties.get(key) {
            return Some(Rc::clone(val));
        }

        if let Some(ref parent) = self.parent {
            return parent.borrow().lookup(key);
        }

        None
    }
}
