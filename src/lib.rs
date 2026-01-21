// src/lib.rs

pub mod interpretator;

// Импортируем необходимые трейты и модули из стандартной библиотеки
use std::any::Any;
use std::fmt;

// Импортируем ваши типы, используя полные пути согласно структуре проекта
use crate::interpretator::prototypes::types::object::object::{Object, ObjectValue};
use crate::interpretator::prototypes::types::prototype::Prototype;

fn main() {
    println!("Система объектов загружена успешно.");
}

#[cfg(test)]
mod tests {
    // Импортируем всё из внешнего модуля (из lib.rs)
    use super::*;

    // 1. Создаем реализацию значения для теста
    #[derive(Debug, Clone)]
    struct StringValue(String);

    impl ObjectValue for StringValue {
        fn as_any(&self) -> &dyn Any {
            self
        }
        fn clone_value(&self) -> Box<dyn ObjectValue> {
            Box::new(self.clone())
        }
    }

    impl fmt::Display for StringValue {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", self.0)
        }
    }

    #[test]
    fn test_generic_object_behavior() {
        // Создаем базовый прототип "String"
        let string_proto = Prototype::new("String", None);

        // Создаем свойство-пустышку в прототипе
        let method_name = "length".to_string();
        let method_obj = Object::new(string_proto.clone(), None);

        string_proto
            .borrow_mut()
            .properties
            .insert(method_name.clone(), method_obj.clone());

        // 2. Создаем экземпляр объекта с данными
        let my_string = Object::new(
            string_proto.clone(),
            Some(Box::new(StringValue("Hello Rust!".to_string()))),
        );

        // 3. ПРОВЕРКА 1: Display
        assert_eq!(format!("{}", my_string.borrow()), "Hello Rust!");

        // 4. ПРОВЕРКА 2: Наследование
        let prop = my_string.borrow().get("length");
        assert!(prop.is_some(), "Свойство должно быть найдено в прототипе");

        // 5. ПРОВЕРКА 3: Downcasting
        let obj_borrow = my_string.borrow();
        if let Some(val) = obj_borrow.downcast::<StringValue>() {
            assert_eq!(val.0, "Hello Rust!");
        } else {
            panic!("Не удалось выполнить downcast к StringValue");
        }
    }
}
