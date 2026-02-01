use crate::interpretator::prototypes::types::object::object::{Object, ObjectRef, ObjectValue};
use crate::interpretator::prototypes::types::prototype::PrototypeRef;
use std::any::Any;
use std::fmt;

#[derive(Debug, Clone)]
pub struct UndefinedValue;

impl ObjectValue for UndefinedValue {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn clone_value(&self) -> Box<dyn ObjectValue> {
        Box::new(self.clone())
    }
}

impl fmt::Display for UndefinedValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "undefined")
    }
}

pub struct Undefined;
impl Undefined {
    pub fn new_instance(proto: PrototypeRef) -> ObjectRef {
        Object::new(proto, Some(Box::new(UndefinedValue)))
    }
}
