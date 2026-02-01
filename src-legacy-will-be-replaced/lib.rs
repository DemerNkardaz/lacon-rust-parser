// src/lib.rs

pub mod interpretator;

// Импортируем необходимые трейты и модули из стандартной библиотеки
use std::any::Any;
use std::fmt;

// Импортируем ваши типы, используя полные пути согласно структуре проекта
use crate::interpretator::prototypes::types::object::object::{Object, ObjectValue};
use crate::interpretator::prototypes::types::object::routine::procedure::procedure::Procedure;
use crate::interpretator::prototypes::types::prototype::Prototype;

fn main() {
    println!("Система объектов загружена успешно.");
}

#[test]
fn test_procedure_violation() {
    let object_proto = Prototype::new("Object", None);
    let routine_proto = Prototype::new("Routine", Some(object_proto.clone()));
    let procedure_proto = Prototype::new("Procedure", Some(routine_proto.clone()));

    // ПОПЫТКА НАРУШЕНИЯ:
    // Мы создаем Процедуру, но передаем ей логику, которая возвращает объект.
    let malicious_proc =
        Procedure::new_instance(procedure_proto, "broken_proc".to_string(), vec![], |args| {
            let dummy_proto = Prototype::new("String", None);
            // Пытаемся вернуть объект, хотя процедура требует void ()
            Object::new(dummy_proto, None)
        });
}