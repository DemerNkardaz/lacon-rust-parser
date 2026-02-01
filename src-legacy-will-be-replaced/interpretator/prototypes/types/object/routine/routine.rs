use crate::interpretator::prototypes::types::object::object::{Object, ObjectRef, ObjectValue};
use crate::interpretator::prototypes::types::prototype::PrototypeRef;
use std::any::Any;
use std::fmt;

/// Строгое разделение типов контента
#[derive(Clone)]
pub enum RoutineContent {
    /// Обязана вернуть ObjectRef
    NativeFunction(fn(Vec<ObjectRef>) -> ObjectRef),
    /// Возвращает (), то есть ничего
    NativeProcedure(fn(Vec<ObjectRef>)),
    /// Пользовательское определение (например, тело функции в виде строки или AST)
    Defined(String),
}

#[derive(Debug, Clone)]
pub struct RoutineValue {
    pub name: String,
    pub parameters: Vec<String>,
    pub content: RoutineContent,
}

impl ObjectValue for RoutineValue {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn clone_value(&self) -> Box<dyn ObjectValue> {
        Box::new(self.clone())
    }
}

impl fmt::Display for RoutineValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let r_type = match self.content {
            RoutineContent::NativeFunction(_) | RoutineContent::NativeProcedure(_) => "native ",
            _ => "",
        };
        write!(
            f,
            "<{}routine {}({})>",
            r_type,
            self.name,
            self.parameters.join(", ")
        )
    }
}

// Нужен для отладки, так как в enum есть указатели на функции
impl fmt::Debug for RoutineContent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RoutineContent::NativeFunction(_) => write!(f, "NativeFunction"),
            RoutineContent::NativeProcedure(_) => write!(f, "NativeProcedure"),
            RoutineContent::Defined(s) => write!(f, "Defined({})", s),
        }
    }
}

pub struct Routine;

impl Routine {
    pub fn new_instance(
        proto: PrototypeRef,
        name: String,
        parameters: Vec<String>,
        content: RoutineContent,
    ) -> ObjectRef {
        Object::new(
            proto,
            Some(Box::new(RoutineValue {
                name,
                parameters,
                content,
            })),
        )
    }
}
