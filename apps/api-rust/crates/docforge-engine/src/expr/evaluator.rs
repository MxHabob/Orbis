// ═══════════════════════════════════════════════════════════════
// docforge-engine — Expression Evaluator
// Evaluates parsed template expressions against a JSON context.
// ═══════════════════════════════════════════════════════════════

use crate::expr::formatters::format_value;
use crate::expr::parser::{parse_expression, Token};
use serde_json::Value;

/// Evaluate a template string against a JSON data context.
pub fn evaluate_expression(template: &str, context: &Value) -> String {
    let tokens = parse_expression(template);
    evaluate_tokens(&tokens, context)
}

fn evaluate_tokens(tokens: &[Token], context: &Value) -> String {
    let mut result = String::new();

    for token in tokens {
        match token {
            Token::Literal(val) => {
                result.push_str(val);
            }
            Token::Variable { path, formatters } => {
                let mut current_val = resolve_path(context, path).unwrap_or(Value::Null);

                // Apply formatters in order
                for fmt in formatters {
                    if fmt.name == "default" && is_empty_value(&current_val) {
                        if !fmt.quoted_args.first().copied().unwrap_or(false) {
                            if let Some(fallback_path) = fmt.args.first() {
                                if let Some(fallback_value) = resolve_path(context, fallback_path) {
                                    current_val = fallback_value;
                                    continue;
                                }
                            }
                        }
                    }
                    current_val = format_value(&fmt.name, &current_val, &fmt.args);
                }

                if !current_val.is_null() {
                    let formatted_str = match &current_val {
                        Value::String(s) => s.clone(),
                        Value::Number(n) => n.to_string(),
                        Value::Bool(b) => b.to_string(),
                        _ => current_val.to_string(),
                    };
                    result.push_str(&formatted_str);
                }
            }
            Token::Conditional {
                condition,
                then_branch,
                else_branch,
            } => {
                let cond_result = evaluate_condition(condition, context);
                if cond_result {
                    result.push_str(&evaluate_expression(then_branch, context));
                } else if let Some(ref else_b) = else_branch {
                    result.push_str(&evaluate_expression(else_b, context));
                }
            }
        }
    }

    result
}

fn is_empty_value(value: &Value) -> bool {
    match value {
        Value::Null => true,
        Value::String(value) => value.trim().is_empty(),
        _ => false,
    }
}

/// Resolve a dot-separated path against a JSON value.
pub fn resolve_path(context: &Value, path: &str) -> Option<Value> {
    if path.trim().is_empty() {
        return Some(context.clone());
    }

    let parts = path.split('.');
    let mut current = context;

    for part in parts {
        match current {
            Value::Object(map) => {
                if let Some(val) = map.get(part) {
                    current = val;
                } else {
                    return None;
                }
            }
            Value::Array(arr) => {
                if let Ok(idx) = part.parse::<usize>() {
                    if let Some(val) = arr.get(idx) {
                        current = val;
                    } else {
                        return None;
                    }
                } else {
                    return None;
                }
            }
            _ => return None,
        }
    }

    Some(current.clone())
}

/// Evaluate a boolean condition expression against a JSON data context.
pub fn evaluate_condition(condition: &str, context: &Value) -> bool {
    let trimmed = condition.trim();

    // Handle AND (&&)
    if trimmed.contains("&&") {
        let parts = split_top_level(trimmed, "&&");
        return parts.iter().all(|part| evaluate_condition(part, context));
    }

    // Handle OR (||)
    if trimmed.contains("||") {
        let parts = split_top_level(trimmed, "||");
        return parts.iter().any(|part| evaluate_condition(part, context));
    }

    // Handle NOT (!)
    if trimmed.starts_with('!') {
        return !evaluate_condition(&trimmed[1..], context);
    }

    // Handle parentheses
    if trimmed.starts_with('(') && trimmed.ends_with(')') {
        return evaluate_condition(&trimmed[1..trimmed.len() - 1], context);
    }

    // Handle comparison operators
    let comp_ops = [">=", "<=", "!=", "==", ">", "<"];
    for op in comp_ops {
        if let Some(idx) = trimmed.find(op) {
            let left_str = trimmed[..idx].trim();
            let right_str = trimmed[idx + op.len()..].trim();

            let left = resolve_value(left_str, context);
            let right = resolve_value(right_str, context);

            return compare_values(&left, &right, op);
        }
    }

    // Default: check truthiness of variable/literal
    let val = resolve_value(trimmed, context);
    is_truthy(&val)
}

/// Split condition string by top-level operator, respecting quotes and parentheses.
fn split_top_level(expr: &str, operator: &str) -> Vec<String> {
    let mut parts = Vec::new();
    let mut depth = 0;
    let mut in_string = false;
    let mut string_char = ' ';
    let mut current = String::new();

    let bytes = expr.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        let ch = expr[i..].chars().next().unwrap();
        let ch_len = ch.len_utf8();

        if in_string {
            current.push(ch);
            if ch == string_char {
                in_string = false;
            }
            i += ch_len;
            continue;
        }

        if ch == '\'' || ch == '"' {
            in_string = true;
            string_char = ch;
            current.push(ch);
            i += ch_len;
            continue;
        }

        if ch == '(' {
            depth += 1;
            current.push(ch);
            i += ch_len;
            continue;
        }
        if ch == ')' {
            depth -= 1;
            current.push(ch);
            i += ch_len;
            continue;
        }

        if depth == 0 && expr[i..].starts_with(operator) {
            parts.push(current.trim().to_string());
            current.clear();
            i += operator.len();
            continue;
        }

        current.push(ch);
        i += ch_len;
    }

    if !current.trim().is_empty() {
        parts.push(current.trim().to_string());
    }

    parts
}

/// Resolve condition token: string literal, number, boolean, null, or path.
fn resolve_value(token: &str, context: &Value) -> Value {
    let trimmed = token.trim();

    // String literal
    if (trimmed.starts_with('\'') && trimmed.ends_with('\''))
        || (trimmed.starts_with('"') && trimmed.ends_with('"'))
    {
        return Value::String(trimmed[1..trimmed.len() - 1].to_string());
    }

    // Number
    if let Ok(num) = trimmed.parse::<f64>() {
        if let Some(n) = serde_json::Number::from_f64(num) {
            return Value::Number(n);
        }
    }

    // Booleans and nulls
    match trimmed {
        "true" => return Value::Bool(true),
        "false" => return Value::Bool(false),
        "null" | "undefined" => return Value::Null,
        _ => {}
    }

    // Path
    resolve_path(context, trimmed).unwrap_or(Value::Null)
}

fn compare_values(left: &Value, right: &Value, op: &str) -> bool {
    match op {
        "==" => left == right,
        "!=" => left != right,
        ">" => get_f64(left) > get_f64(right),
        ">=" => get_f64(left) >= get_f64(right),
        "<" => get_f64(left) < get_f64(right),
        "<=" => get_f64(left) <= get_f64(right),
        _ => false,
    }
}

fn get_f64(val: &Value) -> f64 {
    match val {
        Value::Number(n) => n.as_f64().unwrap_or(0.0),
        Value::String(s) => s.parse::<f64>().unwrap_or(0.0),
        Value::Bool(b) => {
            if *b {
                1.0
            } else {
                0.0
            }
        }
        _ => 0.0,
    }
}

fn is_truthy(value: &Value) -> bool {
    match value {
        Value::Null => false,
        Value::Bool(b) => *b,
        Value::Number(n) => n.as_f64().unwrap_or(0.0) != 0.0,
        Value::String(s) => !s.trim().is_empty(),
        Value::Array(arr) => !arr.is_empty(),
        Value::Object(obj) => !obj.is_empty(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_evaluate_simple_interpolation() {
        let ctx = json!({ "name": "أحمد" });
        assert_eq!(evaluate_expression("مرحبا {{name}}", &ctx), "مرحبا أحمد");
    }

    #[test]
    fn test_default_can_resolve_fallback_path() {
        let ctx = json!({ "student": { "nameAr": "", "fullName": "Ahmed Student" } });
        assert_eq!(
            evaluate_expression("{{student.nameAr | default:student.fullName}}", &ctx),
            "Ahmed Student"
        );
    }

    #[test]
    fn test_default_keeps_quoted_text_literal() {
        let ctx = json!({ "name": "", "bar": "context value" });
        assert_eq!(evaluate_expression("{{name | default:'bar'}}", &ctx), "bar");
    }

    #[test]
    fn test_evaluate_condition_truthy() {
        let ctx = json!({ "show": true, "name": "خالد" });
        assert_eq!(
            evaluate_expression("{{#if show}}أهلا {{name}}{{#else}}مع السلامة{{/if}}", &ctx),
            "أهلا خالد"
        );
    }

    #[test]
    fn test_evaluate_condition_falsy() {
        let ctx = json!({ "show": false, "name": "خالد" });
        assert_eq!(
            evaluate_expression("{{#if show}}أهلا {{name}}{{#else}}مع السلامة{{/if}}", &ctx),
            "مع السلامة"
        );
    }

    #[test]
    fn test_nested_conditions_eval() {
        let ctx = json!({ "outer": true, "inner": false });
        assert_eq!(
            evaluate_expression(
                "{{#if outer}}Outer {{#if inner}}Inner{{#else}}InnerElse{{/if}}{{/if}}",
                &ctx
            ),
            "Outer InnerElse"
        );
    }

    #[test]
    fn test_logical_operators_condition() {
        let ctx = json!({ "role": "admin", "active": true });
        assert!(evaluate_condition("role == 'admin' && active", &ctx));
        assert!(!evaluate_condition("role != 'admin' || !active", &ctx));
    }
}
