use crate::interpretator::prototypes::types::object::object::{Object, ObjectRef, ObjectValue};
use crate::interpretator::prototypes::types::prototype::PrototypeRef;
use std::any::Any;
use std::fmt;

/// Внутреннее значение многострочной строки
#[derive(Debug, Clone)]
pub struct MultilineStringValue {
    pub content: String,
    /// Храним отдельно строки для удобства манипуляций (например, trim_margin)
    pub lines: Vec<String>,
}

impl MultilineStringValue {
    pub fn new(content: String) -> Self {
        let lines = content.lines().map(|s| s.to_string()).collect();
        MultilineStringValue { content, lines }
    }
}

impl ObjectValue for MultilineStringValue {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn clone_value(&self) -> Box<dyn ObjectValue> {
        Box::new(self.clone())
    }
}

impl fmt::Display for MultilineStringValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Выводим как есть, сохраняя переносы
        write!(f, "{}", self.content)
    }
}

/// Конструктор для создания объекта MultilineString
pub struct MultilineString;

impl MultilineString {
    pub fn new_instance(proto: PrototypeRef, content: String) -> ObjectRef {
        Object::new(proto, Some(Box::new(MultilineStringValue::new(content))))
    }
}
