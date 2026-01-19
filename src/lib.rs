use libc::c_char;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::ffi::{CStr, CString};
use std::path::{Path, PathBuf};
use std::{fs, ptr};

// --- Внутренняя структура данных ---

#[derive(Debug, Clone)]
pub enum LaconValue {
    Object(HashMap<String, LaconValue>),
    Array(Vec<LaconValue>),
    String(String),
    Number(f64),
    Bool(bool),
    Null,
}

pub struct ParserContext {
    current_dir: PathBuf,
    import_stack: HashSet<PathBuf>,
    variable_registry: HashMap<String, String>,
}

// --- FFI Интерфейс с использованием libc ---

/// Парсит файл LaCoN и возвращает указатель на структуру LaconValue.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn parse_lacon(path: *const c_char) -> *mut LaconValue {
    if path.is_null() {
        return ptr::null_mut();
    }

    let c_str = unsafe { CStr::from_ptr(path) };
    let path_str = match c_str.to_str() {
        Ok(s) => s,
        Err(_) => return ptr::null_mut(),
    };

    let path_buf = PathBuf::from(path_str);
    match parse_lacon_file(&path_buf, &mut HashSet::new()) {
        Ok(val) => Box::into_raw(Box::new(val)),
        Err(_) => ptr::null_mut(),
    }
}

/// Освобождает память, выделенную под LaconValue.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn free_lacon_value(ptr: *mut LaconValue) {
    if !ptr.is_null() {
        unsafe { drop(Box::from_raw(ptr)) };
    }
}

/// Получение строки по ключу. Возвращает указатель на c_char.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn get_lacon_string(ptr: *mut LaconValue, key: *const c_char) -> *mut c_char {
    if ptr.is_null() || key.is_null() {
        return ptr::null_mut();
    }

    let key_str = unsafe { CStr::from_ptr(key).to_str().unwrap_or("") };
    let root = unsafe { &*ptr };

    if let LaconValue::Object(map) = root {
        if let Some(LaconValue::String(s)) = map.get(key_str) {
            return CString::new(s.as_str()).unwrap().into_raw();
        }
    }
    ptr::null_mut()
}

/// Освобождает строку, выделенную Rust-ом для FFI.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn free_lacon_string(ptr: *mut c_char) {
    if !ptr.is_null() {
        unsafe { drop(CString::from_raw(ptr)) };
    }
}

// --- Внутренняя логика парсера ---

fn parse_lacon_file(
    file_path: &Path,
    import_stack: &mut HashSet<PathBuf>,
) -> Result<LaconValue, String> {
    let abs_path = fs::canonicalize(file_path).map_err(|e| e.to_string())?;

    if import_stack.contains(&abs_path) {
        return Err(format!("Circular import detected: {:?}", abs_path));
    }

    let content = fs::read_to_string(&abs_path).map_err(|e| e.to_string())?;
    let current_dir = abs_path
        .parent()
        .unwrap_or_else(|| Path::new("."))
        .to_path_buf();

    import_stack.insert(abs_path.clone());

    let mut ctx = ParserContext {
        current_dir,
        import_stack: import_stack.clone(),
        variable_registry: HashMap::new(),
    };

    let result = lacon_to_value_internal(&content, &mut ctx);
    import_stack.remove(&abs_path);

    Ok(result)
}

fn lacon_to_value_internal(text: &str, ctx: &mut ParserContext) -> LaconValue {
    let mut root_map = HashMap::new();
    let lines: Vec<&str> = text.lines().collect();

    let var_regex = Regex::new(r"(?u)^\s*(?<!\\)\$([\p{L}\d._-]+)\s*=?\s*(.+)$").unwrap();
    let multiline_start_regex = Regex::new(r"(?u)^\s*([\p{L}\d._-]+)\s*=?\s*(@?\()\s*$").unwrap();

    let mut i = 0;
    while i < lines.len() {
        let line = lines[i];
        let trimmed = line.trim();

        if trimmed.is_empty() || trimmed.starts_with("//") {
            i += 1;
            continue;
        }

        if let Some(caps) = var_regex.captures(trimmed) {
            let name = caps[1].to_string();
            let val = unwrap_quotes(caps[2].trim());
            ctx.variable_registry.insert(name, val);
            i += 1;
            continue;
        }

        if trimmed.starts_with("@import") {
            handle_import_into_map(trimmed, &mut root_map, ctx);
            i += 1;
            continue;
        }

        if let Some(caps) = multiline_start_regex.captures(trimmed) {
            let key = caps[1].to_string();
            let is_raw = caps[2].starts_with('@');
            let mut block_lines = Vec::new();
            i += 1;
            while i < lines.len() && lines[i].trim() != ")" {
                block_lines.push(lines[i]);
                i += 1;
            }
            let content = if is_raw {
                process_raw_multiline(block_lines)
            } else {
                process_quoted_multiline(block_lines)
            };
            root_map.insert(
                key,
                LaconValue::String(resolve_variables(&content, &ctx.variable_registry)),
            );
            i += 1;
            continue;
        }

        if let Some(idx) = trimmed.find('=') {
            let key = trimmed[..idx].trim().to_string();
            let val = parse_single_value(trimmed[idx + 1..].trim(), ctx);
            root_map.insert(key, val);
        } else if let Some(idx) = trimmed.find(' ') {
            let key = trimmed[..idx].trim().to_string();
            let val = parse_single_value(trimmed[idx + 1..].trim(), ctx);
            root_map.insert(key, val);
        } else {
            root_map.insert(trimmed.to_string(), LaconValue::Bool(true));
        }

        i += 1;
    }

    LaconValue::Object(root_map)
}

fn parse_single_value(val: &str, ctx: &mut ParserContext) -> LaconValue {
    let resolved = resolve_variables(val, &ctx.variable_registry);
    if resolved == "true" {
        return LaconValue::Bool(true);
    }
    if resolved == "false" {
        return LaconValue::Bool(false);
    }
    if let Ok(n) = resolved.parse::<f64>() {
        return LaconValue::Number(n);
    }
    LaconValue::String(unwrap_quotes(&resolved))
}

fn handle_import_into_map(
    line: &str,
    map: &mut HashMap<String, LaconValue>,
    ctx: &mut ParserContext,
) {
    // Сохраняем строку в binding, чтобы она жила до конца функции
    let binding = line.replace("@import", "");
    let path_part = binding.trim();
    let clean_path = unwrap_quotes(path_part);
    let full_path = ctx.current_dir.join(clean_path);

    if let Ok(LaconValue::Object(imported_map)) =
        parse_lacon_file(&full_path, &mut ctx.import_stack)
    {
        for (k, v) in imported_map {
            map.insert(k, v);
        }
    }
}

// --- Хелперы обработки строк ---

fn resolve_variables(text: &str, vars: &HashMap<String, String>) -> String {
    let mut out = text.to_string();
    for (name, value) in vars {
        let target = format!("${}", name);
        out = out.replace(&target, value);
    }
    out
}

fn unwrap_quotes(s: &str) -> String {
    if s.starts_with('"') && s.ends_with('"') && s.len() >= 2 {
        s[1..s.len() - 1].to_string()
    } else {
        s.to_string()
    }
}

fn process_raw_multiline(lines: Vec<&str>) -> String {
    lines.join("\n").trim().to_string()
}

fn process_quoted_multiline(lines: Vec<&str>) -> String {
    lines
        .iter()
        .map(|l| unwrap_quotes(l.trim().trim_end_matches(',')))
        .collect::<Vec<_>>()
        .join("\n")
}
