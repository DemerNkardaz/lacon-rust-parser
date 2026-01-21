use crate::interpretator::prototypes::types::object::object::{Object, ObjectRef, ObjectValue};
use crate::interpretator::prototypes::types::prototype::PrototypeRef;
use std::any::Any;
use std::fmt;

/// Список поддерживаемых типов операторов
#[derive(Debug, Clone, PartialEq)]
pub enum OperatorType {
    Add,       // +
    Sub,       // -
    Mul,       // *
    Div,       // /
    IDiv,      // //
    Mod,       // %
    Pow,       // **
    Less,      // <
    Greater,   // >
    LessEq,    // <=
    GreaterEq, // >=
    Equal,     // ==
}

impl OperatorType {
    pub fn as_str(&self) -> &'static str {
        match self {
            OperatorType::Add => "+",
            OperatorType::Sub => "-",
            OperatorType::Mul => "*",
            OperatorType::Div => "/",
            OperatorType::IDiv => "//",
            OperatorType::Mod => "%",
            OperatorType::Pow => "**",
            OperatorType::Less => "<",
            OperatorType::Greater => ">",
            OperatorType::LessEq => "<=",
            OperatorType::GreaterEq => ">=",
            OperatorType::Equal => "==",
        }
    }
}

#[derive(Debug, Clone)]
pub struct OperatorValue {
    pub op_type: OperatorType,
}

impl ObjectValue for OperatorValue {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn clone_value(&self) -> Box<dyn ObjectValue> {
        Box::new(self.clone())
    }
}

impl fmt::Display for OperatorValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.op_type.as_str())
    }
}

pub struct Operator;

impl Operator {
    /// Создает объект-оператор.
    /// Наследует напрямую от Base Prototype.
    pub fn new_instance(proto: PrototypeRef, op_type: OperatorType) -> ObjectRef {
        Object::new(proto, Some(Box::new(OperatorValue { op_type })))
    }
}
