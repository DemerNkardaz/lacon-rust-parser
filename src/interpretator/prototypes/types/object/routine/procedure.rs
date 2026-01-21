use crate::interpretator::prototypes::types::object::object::ObjectRef;
use crate::interpretator::prototypes::types::object::routine::routine::{Routine, RoutineContent};
use crate::interpretator::prototypes::types::prototype::PrototypeRef;

pub struct Procedure;

impl Procedure {
    /// Конструктор процедуры. Принимает Rust-функцию, возвращающую void ().
    pub fn new_instance(
        proto: PrototypeRef,
        name: String,
        parameters: Vec<String>,
        proc: fn(Vec<ObjectRef>),
    ) -> ObjectRef {
        Routine::new_instance(
            proto,
            name,
            parameters,
            RoutineContent::NativeProcedure(proc),
        )
    }
}
