// ═══════════════════════════════════════════════════════════════
// docforge-engine — Expression Parser
// Tokenizes template strings containing {{expressions}}
// ═══════════════════════════════════════════════════════════════

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Literal(String),
    Variable {
        path: String,
        formatters: Vec<FormatterCall>,
    },
    Conditional {
        condition: String,
        then_branch: String,
        else_branch: Option<String>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub struct FormatterCall {
    pub name: String,
    pub args: Vec<String>,
    pub quoted_args: Vec<bool>,
}

/// Parse a template string into a sequence of tokens.
pub fn parse_expression(template: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut remaining = template;

    while !remaining.is_empty() {
        // Check for conditional block opening: {{#if condition}}
        if remaining.starts_with("{{#if ") {
            if let Some(close_idx) = remaining.find("}}") {
                let condition = remaining[6..close_idx].trim().to_string();
                let after_open = &remaining[close_idx + 2..];

                let (then_branch, else_branch, rest) = parse_conditional_body(after_open);
                tokens.push(Token::Conditional {
                    condition,
                    then_branch,
                    else_branch,
                });
                remaining = rest;
                continue;
            }
        }

        // Check for variable expression: {{variable}}
        if remaining.starts_with("{{") {
            if let Some(close_idx) = remaining.find("}}") {
                let expr = remaining[2..close_idx].trim();
                tokens.push(parse_variable_expression(expr));
                remaining = &remaining[close_idx + 2..];
                continue;
            }
        }

        // Find the next {{ or end of string
        if let Some(next_open) = remaining.find("{{") {
            if next_open > 0 {
                tokens.push(Token::Literal(remaining[..next_open].to_string()));
            }
            remaining = &remaining[next_open..];
        } else {
            tokens.push(Token::Literal(remaining.to_string()));
            break;
        }
    }

    tokens
}

/// Parse a variable expression (content between {{ and }}).
/// Handles pipe-based formatters: `path | formatter:'arg1':'arg2'`
fn parse_variable_expression(expr: &str) -> Token {
    let parts: Vec<&str> = expr.split('|').map(|p| p.trim()).collect();
    let path = parts[0].to_string();
    let mut formatters = Vec::new();

    for &fmt_str in parts.iter().skip(1) {
        if let Some(colon_idx) = fmt_str.find(':') {
            let name = fmt_str[..colon_idx].trim().to_string();
            let args_str = &fmt_str[colon_idx + 1..];
            let args = args_str
                .split(':')
                .map(|a| {
                    let trimmed = a.trim();
                    // Strip enclosing quotes
                    if (trimmed.starts_with('\'') && trimmed.ends_with('\''))
                        || (trimmed.starts_with('"') && trimmed.ends_with('"'))
                    {
                        trimmed[1..trimmed.len() - 1].to_string()
                    } else {
                        trimmed.to_string()
                    }
                })
                .collect();
            let quoted_args = args_str
                .split(':')
                .map(|arg| {
                    let trimmed = arg.trim();
                    (trimmed.starts_with('\'') && trimmed.ends_with('\''))
                        || (trimmed.starts_with('"') && trimmed.ends_with('"'))
                })
                .collect();
            formatters.push(FormatterCall { name, args, quoted_args });
        } else {
            formatters.push(FormatterCall {
                name: fmt_str.to_string(),
                args: Vec::new(),
                quoted_args: Vec::new(),
            });
        }
    }

    Token::Variable { path, formatters }
}

/// Parse body of a conditional block. Returns (then_branch, else_branch, rest).
fn parse_conditional_body(input: &str) -> (String, Option<String>, &str) {
    let mut depth = 1;
    let mut i = 0;
    let mut then_branch = String::new();
    let mut else_branch: Option<String> = None;
    let mut in_else = false;

    let bytes = input.as_bytes();
    while i < bytes.len() && depth > 0 {
        let current_slice = &input[i..];

        // Check for nested {{#if
        if current_slice.starts_with("{{#if ") {
            depth += 1;
            if let Some(close_idx) = current_slice.find("}}") {
                let chunk = &current_slice[..close_idx + 2];
                if in_else {
                    else_branch.as_mut().unwrap().push_str(chunk);
                } else {
                    then_branch.push_str(chunk);
                }
                i += chunk.len();
                continue;
            }
        }

        // Check for {{/if}}
        if current_slice.starts_with("{{/if}}") {
            depth -= 1;
            if depth == 0 {
                i += 7; // Length of "{{/if}}"
                break;
            }
            let chunk = "{{/if}}";
            if in_else {
                else_branch.as_mut().unwrap().push_str(chunk);
            } else {
                then_branch.push_str(chunk);
            }
            i += chunk.len();
            continue;
        }

        // Check for {{#else}} at current depth
        if depth == 1 && current_slice.starts_with("{{#else}}") {
            in_else = true;
            else_branch = Some(String::new());
            i += 9; // Length of "{{#else}}"
            continue;
        }

        // Check for {{#elseif ...}} at current depth
        if depth == 1 && current_slice.starts_with("{{#elseif ") {
            if let Some(close_idx) = current_slice.find("}}") {
                let cond = &current_slice[10..close_idx].trim();
                in_else = true;
                else_branch = Some(format!("{{{{#if {}}}}}", cond));
                i += close_idx + 2;
                continue;
            }
        }

        // Read character-by-character (respecting UTF-8)
        if let Some(ch) = current_slice.chars().next() {
            if in_else {
                else_branch.as_mut().unwrap().push(ch);
            } else {
                then_branch.push(ch);
            }
            i += ch.len_utf8();
        } else {
            break;
        }
    }

    (then_branch, else_branch, &input[i..])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_literal() {
        assert_eq!(
            parse_expression("Hello, world!"),
            vec![Token::Literal("Hello, world!".to_string())]
        );
    }

    #[test]
    fn test_parse_variable() {
        assert_eq!(
            parse_expression("Hello {{name}}!"),
            vec![
                Token::Literal("Hello ".to_string()),
                Token::Variable {
                    path: "name".to_string(),
                    formatters: Vec::new(),
                },
                Token::Literal("!".to_string())
            ]
        );
    }

    #[test]
    fn test_parse_formatter() {
        assert_eq!(
            parse_expression("{{amount | number:'0.00' | default:'—'}}"),
            vec![Token::Variable {
                path: "amount".to_string(),
                formatters: vec![
                    FormatterCall {
                        name: "number".to_string(),
                        args: vec!["0.00".to_string()]
                        , quoted_args: vec![true]
                    },
                    FormatterCall {
                        name: "default".to_string(),
                        args: vec!["—".to_string()]
                        , quoted_args: vec![true]
                    }
                ]
            }]
        );
    }

    #[test]
    fn test_parse_conditional() {
        let tokens = parse_expression("{{#if show}}Yes{{#else}}No{{/if}}");
        assert_eq!(
            tokens,
            vec![Token::Conditional {
                condition: "show".to_string(),
                then_branch: "Yes".to_string(),
                else_branch: Some("No".to_string())
            }]
        );
    }

    #[test]
    fn test_parse_nested_conditional() {
        let tokens = parse_expression("{{#if show_outer}}{{#if show_inner}}Inner{{/if}}{{/if}}");
        assert_eq!(
            tokens,
            vec![Token::Conditional {
                condition: "show_outer".to_string(),
                then_branch: "{{#if show_inner}}Inner{{/if}}".to_string(),
                else_branch: None,
            }]
        );
    }
}
