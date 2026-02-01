use crate::interpretator::prototypes::types::object::object::{Object, ObjectRef, ObjectValue};
use crate::interpretator::prototypes::types::prototype::PrototypeRef;
use std::any::Any;
use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;

/// Внутренние данные массива, которые будут храниться в Object.value
#[derive(Debug, Clone)]
pub struct ArrayValue {
    pub elements: Vec<ObjectRef>,
}

impl ArrayValue {
    pub fn new(elements: Vec<ObjectRef>) -> Self {
        ArrayValue { elements }
    }
}

impl ObjectValue for ArrayValue {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn clone_value(&self) -> Box<dyn ObjectValue> {
        Box::new(self.clone())
    }
}

impl fmt::Display for ArrayValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let strings: Vec<String> = self
            .elements
            .iter()
            .map(|el| format!("{}", el.borrow()))
            .collect();
        write!(f, "[{}]", strings.join(", "))
    }
}

/// Конструктор для создания объекта-массива
pub struct Array;

impl Array {
    /// Создает новый Object, который ведет себя как массив.
    /// Он получает ArrayPrototype, который в свою очередь наследует ObjectPrototype.
    pub fn new_instance(proto: PrototypeRef, elements: Vec<ObjectRef>) -> ObjectRef {
        Object::new(proto, Some(Box::new(ArrayValue::new(elements))))
    }
}
