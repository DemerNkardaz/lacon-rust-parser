use crate::interpretator::prototypes::types::object::object::{Object, ObjectRef, ObjectValue};
use crate::interpretator::prototypes::types::prototype::PrototypeRef;
use std::any::Any;
use std::fmt;

#[derive(Debug, Clone)]
pub struct NilValue;

impl ObjectValue for NilValue {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn clone_value(&self) -> Box<dyn ObjectValue> {
        Box::new(self.clone())
    }
}

impl fmt::Display for NilValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "nil")
    }
}

pub struct Nil;
impl Nil {
    pub fn new_instance(proto: PrototypeRef) -> ObjectRef {
        Object::new(proto, Some(Box::new(NilValue)))
    }
}
