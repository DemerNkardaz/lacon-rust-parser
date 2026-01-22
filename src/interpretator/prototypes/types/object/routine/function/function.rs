use crate::interpretator::prototypes::types::object::object::ObjectRef;
use crate::interpretator::prototypes::types::object::routine::routine::{Routine, RoutineContent};
use crate::interpretator::prototypes::types::prototype::PrototypeRef;

pub struct Function;

impl Function {
    /// Конструктор функции. Принимает Rust-функцию, возвращающую ObjectRef.
    pub fn new_instance(
        proto: PrototypeRef,
        name: String,
        parameters: Vec<String>,
        func: fn(Vec<ObjectRef>) -> ObjectRef,
    ) -> ObjectRef {
        Routine::new_instance(
            proto,
            name,
            parameters,
            RoutineContent::NativeFunction(func),
        )
    }
}
