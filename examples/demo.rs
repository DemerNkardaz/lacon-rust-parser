// examples/demo.rs
use lacon_rust_parser::{parse_string_literal, parse_typed_value, parse_value};

fn main() {
    println!("=== ТЕСТ: ЧТЕНИЕ ФАЙЛА ===\n");

    // Читаем файл
    let content = std::fs::read_to_string("test.lacon").expect("Не удалось прочитать test.lacon");

    println!("Содержимое файла:\n{}\n", content);
    println!("=== РЕЗУЛЬТАТЫ ПАРСИНГА ===\n");

    // Парсим построчно
    for line in content.lines() {
        let line = line.trim();

        if line.is_empty() {
            continue;
        }

        // Разделяем по '='
        if let Some((key, value)) = line.split_once('=') {
            let key = key.trim();
            let value = value.trim();

            // Проверяем кавычки
            let parsed = if value.starts_with('"') && value.ends_with('"') {
                // Убираем кавычки
                let unquoted = &value[1..value.len() - 1];
                parse_string_literal(unquoted)
            } else {
                // Типизированное значение
                parse_typed_value(value).unwrap_or_else(|| parse_value(value).unwrap())
            };

            println!(
                "{:25} = {:50} -> {:?}",
                format!("{}={}", key, value),
                format!("{:?}", parsed),
                parsed
            );
        }
    }

    println!("\n=== РАЗНИЦА МЕЖДУ ТИПАМИ ===\n");

    // Наглядная демонстрация разницы
    let comparisons = vec![
        ("100em", false),
        ("100em", true),
        ("true", false),
        ("true", true),
        ("auto", false),
        ("auto", true),
        ("50%", false),
        ("50%", true),
    ];

    for (input, is_quoted) in comparisons {
        let display = if is_quoted {
            format!("\"{}\"", input)
        } else {
            input.to_string()
        };

        let result = if is_quoted {
            parse_string_literal(input)
        } else {
            parse_typed_value(input).unwrap_or_else(|| parse_value(input).unwrap())
        };

        println!("{:15} -> {:?}", display, result);
    }
}
