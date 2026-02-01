use crate::interpretator::prototypes::types::prototype::PrototypeRef;
use std::any::Any;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;

/// Ссылка на объект. Позволяет нескольким местам владеть объектом и изменять его.
pub type ObjectRef = Rc<RefCell<Object>>;

/// Трейт для "значения", которое лежит внутри объекта.
/// Это позволяет нам добавлять любые типы данных (числа, сокеты, файлы),
/// не меняя структуру Object.
pub trait ObjectValue: fmt::Display + fmt::Debug {
    fn as_any(&self) -> &dyn Any;
    fn clone_value(&self) -> Box<dyn ObjectValue>;
}

/// Реализация клонирования для Box<dyn ObjectValue>
impl Clone for Box<dyn ObjectValue> {
    fn clone(&self) -> Self {
        self.clone_value()
    }
}

/// Универсальный объект.
/// Он состоит из ссылки на прототип, таблицы свойств и "черного ящика" с данными.
pub struct Object {
    /// Прототип определяет поведение (методы).
    pub prototype: PrototypeRef,

    /// Динамические свойства объекта (поля).
    pub properties: HashMap<String, ObjectRef>,

    /// Само значение объекта. Box<dyn Any> позволяет хранить что угодно.
    /// Мы используем Option, чтобы объект мог быть "пустым" (как простой Dictionary).
    pub value: Option<Box<dyn ObjectValue>>,
}

impl Object {
    /// Создает новый экземпляр объекта.
    pub fn new(prototype: PrototypeRef, value: Option<Box<dyn ObjectValue>>) -> ObjectRef {
        Rc::new(RefCell::new(Object {
            prototype,
            properties: HashMap::new(),
            value,
        }))
    }

    /// Поиск свойства: сначала в себе, потом в цепочке прототипов.
    pub fn get(&self, key: &str) -> Option<ObjectRef> {
        if let Some(prop) = self.properties.get(key) {
            return Some(Rc::clone(prop));
        }
        self.prototype.borrow().lookup(key)
    }

    /// Установка локального свойства.
    pub fn set(&mut self, key: &str, value: ObjectRef) {
        self.properties.insert(key.to_string(), value);
    }

    /// Метод для безопасного извлечения данных (downcasting).
    pub fn downcast<T: 'static + ObjectValue>(&self) -> Option<&T> {
        self.value.as_ref()?.as_any().downcast_ref::<T>()
    }
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(ref val) = self.value {
            // Если есть значение, делегируем отображение ему
            write!(f, "{}", val)
        } else {
            // Если значения нет, это просто пустой объект или экземпляр класса
            write!(f, "<{} instance>", self.prototype.borrow().name)
        }
    }
}

impl fmt::Debug for Object {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Object")
            .field("prototype", &self.prototype.borrow().name)
            .field("value", &self.value)
            .field("props_count", &self.properties.len())
            .finish()
    }
}

// --- Пример реализации конкретного значения (например, целое число) ---

#[derive(Debug, Clone)]
pub struct IntegerValue(pub i64);

impl ObjectValue for IntegerValue {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn clone_value(&self) -> Box<dyn ObjectValue> {
        Box::new(self.clone())
    }
}

impl fmt::Display for IntegerValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
