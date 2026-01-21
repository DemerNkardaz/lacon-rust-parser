use crate::interpretator::prototypes::types::object::object::{Object, ObjectRef, ObjectValue};
use crate::interpretator::prototypes::types::prototype::PrototypeRef;
use std::any::Any;
use std::fmt;

#[derive(Debug, Clone)]
pub struct AutoValue;

impl ObjectValue for AutoValue {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn clone_value(&self) -> Box<dyn ObjectValue> {
        Box::new(self.clone())
    }
}

impl fmt::Display for AutoValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "auto")
    }
}

pub struct Auto;
impl Auto {
    pub fn new_instance(proto: PrototypeRef) -> ObjectRef {
        Object::new(proto, Some(Box::new(AutoValue)))
    }
}
