use crate::interpretator::prototypes::types::object::object::{Object, ObjectRef, ObjectValue};
use crate::interpretator::prototypes::types::prototype::PrototypeRef;
use std::any::Any;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;

/// Внутренние данные словаря.
/// В отличие от Object.properties (которые хранят системные поля/методы),
/// DictionaryValue хранит данные самого пользователя языка.
#[derive(Debug, Clone)]
pub struct DictionaryValue {
    pub entries: HashMap<String, ObjectRef>,
}

impl DictionaryValue {
    pub fn new(entries: HashMap<String, ObjectRef>) -> Self {
        DictionaryValue { entries }
    }
}

impl ObjectValue for DictionaryValue {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn clone_value(&self) -> Box<dyn ObjectValue> {
        Box::new(self.clone())
    }
}

impl fmt::Display for DictionaryValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let pairs: Vec<String> = self
            .entries
            .iter()
            .map(|(k, v)| format!("\"{}\": {}", k, v.borrow()))
            .collect();
        write!(f, "{{{}}}", pairs.join(", "))
    }
}

/// Конструктор для создания объекта-словаря
pub struct Dictionary;

impl Dictionary {
    /// Создает новый Object, который ведет себя как словарь.
    /// Цепочка прототипов: Dictionary Instance -> Dictionary Prototype -> Object Prototype
    pub fn new_instance(proto: PrototypeRef, entries: HashMap<String, ObjectRef>) -> ObjectRef {
        Object::new(proto, Some(Box::new(DictionaryValue::new(entries))))
    }
}
