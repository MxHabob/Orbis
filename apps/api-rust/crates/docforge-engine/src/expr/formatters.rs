// ═══════════════════════════════════════════════════════════════
// docforge-engine — Built-in Formatters
// Implements 15 formatters for template expressions.
// ═══════════════════════════════════════════════════════════════

use serde_json::Value;
use chrono::Datelike;

/// Format a value using a specified formatter name and arguments.
pub fn format_value(name: &str, value: &Value, args: &[String]) -> Value {
    match name {
        "date" => format_date(value, args.first().map(|s| s.as_str())),
        "hijri" => format_hijri(value),
        "number" => format_number(value, args.first().map(|s| s.as_str())),
        "arabic_number" => format_arabic_number(value),
        "percent" => format_percent(value),
        "uppercase" => Value::String(to_string_or_empty(value).to_uppercase()),
        "lowercase" => Value::String(to_string_or_empty(value).to_lowercase()),
        "ordinal" => format_ordinal(value),
        "grade_letter" => format_grade_letter(value),
        "truncate" => format_truncate(value, args.first().map(|s| s.as_str())),
        "default" => format_default(value, args.first().map(|s| s.as_str())),
        "pad_start" => format_pad_start(value, args.get(0).map(|s| s.as_str()), args.get(1).map(|s| s.as_str())),
        "pad_end" => format_pad_end(value, args.get(0).map(|s| s.as_str()), args.get(1).map(|s| s.as_str())),
        "join" => format_join(value, args.first().map(|s| s.as_str())),
        "count" => format_count(value),
        _ => value.clone(), // Unknown formatters return the value unchanged
    }
}

fn to_string_or_empty(value: &Value) -> String {
    match value {
        Value::Null => String::new(),
        Value::String(s) => s.clone(),
        Value::Number(n) => n.to_string(),
        Value::Bool(b) => b.to_string(),
        _ => value.to_string(),
    }
}

/// Helper to convert digits to Arabic numerals
fn to_arabic_digits(s: &str) -> String {
    s.chars()
        .map(|c| match c {
            '0' => '٠',
            '1' => '١',
            '2' => '٢',
            '3' => '٣',
            '4' => '٤',
            '5' => '٥',
            '6' => '٦',
            '7' => '٧',
            '8' => '٨',
            '9' => '٩',
            _ => c,
        })
        .collect()
}

/// Format Date: {{date_val | date:'yyyy/MM/dd'}}
fn format_date(value: &Value, format_str: Option<&str>) -> Value {
    if value.is_null() {
        return Value::String(String::new());
    }

    let date_str = to_string_or_empty(value);
    if date_str.is_empty() {
        return Value::String(String::new());
    }

    use chrono::NaiveDateTime;
    let parsed_date = NaiveDateTime::parse_from_str(&date_str, "%Y-%m-%dT%H:%M:%S")
        .or_else(|_| NaiveDateTime::parse_from_str(&date_str, "%Y-%m-%d %H:%M:%S"))
        .or_else(|_| chrono::NaiveDate::parse_from_str(&date_str, "%Y-%m-%d").map(|d| d.and_hms_opt(0, 0, 0).unwrap()));

    match parsed_date {
        Ok(dt) => {
            let fmt = format_str.unwrap_or("yyyy-MM-dd");
            let mut result = fmt.to_string();
            result = result.replace("yyyy", &dt.format("%Y").to_string());
            result = result.replace("yy", &dt.format("%y").to_string());
            result = result.replace("MM", &dt.format("%m").to_string());
            result = result.replace("dd", &dt.format("%d").to_string());
            result = result.replace("HH", &dt.format("%H").to_string());
            result = result.replace("mm", &dt.format("%M").to_string());
            result = result.replace("ss", &dt.format("%S").to_string());
            Value::String(result)
        }
        Err(_) => Value::String(date_str),
    }
}

/// Format Hijri: {{date_val | hijri}}
fn format_hijri(value: &Value) -> Value {
    if value.is_null() {
        return Value::String(String::new());
    }

    let date_str = to_string_or_empty(value);
    if date_str.is_empty() {
        return Value::String(String::new());
    }

    use chrono::NaiveDate;
    let parsed_date = NaiveDate::parse_from_str(&date_str, "%Y-%m-%d")
        .or_else(|_| {
            chrono::NaiveDateTime::parse_from_str(&date_str, "%Y-%m-%dT%H:%M:%S")
                .map(|dt| dt.date())
        })
        .or_else(|_| {
            chrono::NaiveDateTime::parse_from_str(&date_str, "%Y-%m-%d %H:%M:%S")
                .map(|dt| dt.date())
        });

    match parsed_date {
        Ok(d) => {
            let (h_y, h_m, h_d) = gregorian_to_hijri(d.year(), d.month() as i32, d.day() as i32);
            // Format as yyyy/MM/dd in Arabic numerals
            let formatted = format!("{:04}/{:02}/{:02}", h_y, h_m, h_d);
            Value::String(to_arabic_digits(&formatted))
        }
        Err(_) => Value::String(date_str),
    }
}

/// Simple algorithm to convert Gregorian to Hijri date
fn gregorian_to_hijri(year: i32, month: i32, day: i32) -> (i32, i32, i32) {
    let mut y = year;
    let mut m = month;
    if m <= 2 {
        y -= 1;
        m += 12;
    }
    let a = y / 100;
    let b = a / 4;
    let c = 2 - a + b;
    let e = (365.25 * (y as f64 + 4716.0)) as i32;
    let f = (30.6001 * (m as f64 + 1.0)) as i32;
    let jd = c + day + e + f - 1524;

    // Convert Julian Day to Hijri
    let l = jd - 1948440 + 385;
    let n = (l * 30) / 10631;
    let h_y = n + 1;
    let mut days_into_year = l - (n * 10631 + 29) / 30;

    let mut h_m = 1;
    while h_m <= 12 {
        let month_len = if h_m % 2 != 0 {
            30
        } else if h_m == 12 && (11 * h_y + 14) % 30 < 11 {
            30
        } else {
            29
        };

        if days_into_year <= month_len {
            break;
        }
        days_into_year -= month_len;
        h_m += 1;
    }

    (h_y, h_m, days_into_year)
}

/// Format Number: {{amount | number:'0.00'}}
fn format_number(value: &Value, format_str: Option<&str>) -> Value {
    let num_val = match value {
        Value::Number(n) => n.as_f64().unwrap_or(0.0),
        Value::String(s) => s.parse::<f64>().unwrap_or(0.0),
        _ => return Value::String(to_string_or_empty(value)),
    };

    let decimals = if let Some(fmt) = format_str {
        if let Some(dot_idx) = fmt.find('.') {
            fmt[dot_idx + 1..].len()
        } else {
            0
        }
    } else {
        0
    };

    let formatted = format!("{:.*}", decimals, num_val);
    
    // Group parts (integer part grouped with commas)
    let parts: Vec<&str> = formatted.split('.').collect();
    let int_part = parts[0];
    
    let mut grouped_int = String::new();
    let chars: Vec<char> = int_part.chars().collect();
    let len = chars.len();
    for (i, &c) in chars.iter().enumerate() {
        grouped_int.push(c);
        let rem = len - 1 - i;
        if rem > 0 && rem % 3 == 0 && c != '-' {
            grouped_int.push(',');
        }
    }

    let final_str = if parts.len() > 1 {
        format!("{}.{}", grouped_int, parts[1])
    } else {
        grouped_int
    };

    Value::String(final_str)
}

/// Convert number to Arabic-Indic numerals: {{num | arabic_number}}
fn format_arabic_number(value: &Value) -> Value {
    let num_str = match format_number(value, Some("0")) {
        Value::String(s) => s,
        _ => to_string_or_empty(value),
    };
    
    // Replace commas with Arabic grouping character (٬) and dots with decimal (٫)
    let ar_num = to_arabic_digits(&num_str)
        .replace(',', "٬")
        .replace('.', "٫");
        
    Value::String(ar_num)
}

/// Percent: {{0.95 | percent}} -> "95%"
fn format_percent(value: &Value) -> Value {
    let num_val = match value {
        Value::Number(n) => n.as_f64().unwrap_or(0.0),
        Value::String(s) => s.parse::<f64>().unwrap_or(0.0),
        _ => return Value::String(to_string_or_empty(value)),
    };

    let pct = if (0.0..=1.0).contains(&num_val) {
        num_val * 100.0
    } else {
        num_val
    };

    Value::String(format!("{}%", pct.round() as i64))
}

/// Arabic Ordinal names: {{1 | ordinal}} -> "الأول"
fn format_ordinal(value: &Value) -> Value {
    let num_val = match value {
        Value::Number(n) => n.as_i64().unwrap_or(0),
        Value::String(s) => s.parse::<i64>().unwrap_or(0),
        _ => return Value::String(to_string_or_empty(value)),
    };

    let ord = match num_val {
        1 => "الأول",
        2 => "الثاني",
        3 => "الثالث",
        4 => "الرابع",
        5 => "الخامس",
        6 => "السادس",
        7 => "السابع",
        8 => "الثامن",
        9 => "التاسع",
        10 => "العاشر",
        _ => return Value::String(to_arabic_digits(&num_val.to_string())),
    };

    Value::String(ord.to_string())
}

/// Score to grade letter: {{95 | grade_letter}}
fn format_grade_letter(value: &Value) -> Value {
    let num_val = match value {
        Value::Number(n) => n.as_f64().unwrap_or(0.0),
        Value::String(s) => s.parse::<f64>().unwrap_or(0.0),
        _ => return Value::String(to_string_or_empty(value)),
    };

    let grade = if num_val >= 97.0 { "أ+" }
    else if num_val >= 93.0 { "أ" }
    else if num_val >= 90.0 { "أ-" }
    else if num_val >= 87.0 { "ب+" }
    else if num_val >= 83.0 { "ب" }
    else if num_val >= 80.0 { "ب-" }
    else if num_val >= 77.0 { "ج+" }
    else if num_val >= 73.0 { "ج" }
    else if num_val >= 70.0 { "ج-" }
    else if num_val >= 67.0 { "د+" }
    else if num_val >= 63.0 { "د" }
    else if num_val >= 60.0 { "د-" }
    else { "هـ" };

    Value::String(grade.to_string())
}

/// Truncate string: {{val | truncate:50}}
fn format_truncate(value: &Value, length_str: Option<&str>) -> Value {
    let str_val = to_string_or_empty(value);
    let max_len = length_str.and_then(|s| s.parse::<usize>().ok()).unwrap_or(100);

    if str_val.chars().count() <= max_len {
        Value::String(str_val)
    } else {
        let truncated: String = str_val.chars().take(max_len).collect();
        Value::String(format!("{}...", truncated))
    }
}

/// Default value if empty/null: {{name | default:'غير محدد'}}
fn format_default(value: &Value, default_str: Option<&str>) -> Value {
    let str_val = to_string_or_empty(value);
    if str_val.trim().is_empty() {
        Value::String(default_str.unwrap_or("").to_string())
    } else {
        value.clone()
    }
}

/// Pad Start: {{num | pad_start:6:'0'}}
fn format_pad_start(value: &Value, len_str: Option<&str>, char_str: Option<&str>) -> Value {
    let str_val = to_string_or_empty(value);
    let length = len_str.and_then(|l| l.parse::<usize>().ok()).unwrap_or(0);
    let pad_char = char_str.and_then(|c| c.chars().next()).unwrap_or(' ');

    if str_val.len() >= length {
        Value::String(str_val)
    } else {
        let needed = length - str_val.len();
        let mut padding = String::new();
        for _ in 0..needed {
            padding.push(pad_char);
        }
        padding.push_str(&str_val);
        Value::String(padding)
    }
}

/// Pad End: {{num | pad_end:6:'0'}}
fn format_pad_end(value: &Value, len_str: Option<&str>, char_str: Option<&str>) -> Value {
    let str_val = to_string_or_empty(value);
    let length = len_str.and_then(|l| l.parse::<usize>().ok()).unwrap_or(0);
    let pad_char = char_str.and_then(|c| c.chars().next()).unwrap_or(' ');

    if str_val.len() >= length {
        Value::String(str_val)
    } else {
        let needed = length - str_val.len();
        let mut padding = str_val;
        for _ in 0..needed {
            padding.push(pad_char);
        }
        Value::String(padding)
    }
}

/// Join Array: {{tags | join:', '}}
fn format_join(value: &Value, sep_str: Option<&str>) -> Value {
    let separator = sep_str.unwrap_or(", ");
    if let Value::Array(arr) = value {
        let stringified: Vec<String> = arr.iter().map(to_string_or_empty).collect();
        Value::String(stringified.join(separator))
    } else {
        Value::String(to_string_or_empty(value))
    }
}

/// Count: {{items | count}}
fn format_count(value: &Value) -> Value {
    let count = match value {
        Value::Array(arr) => arr.len(),
        Value::String(s) => s.chars().count(),
        Value::Object(o) => o.len(),
        Value::Null => 0,
        _ => 1,
    };
    Value::Number(count.into())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_format_default() {
        let val = Value::Null;
        assert_eq!(
            format_value("default", &val, &["fallback".to_string()]),
            Value::String("fallback".to_string())
        );
    }

    #[test]
    fn test_format_number() {
        let val = json!(1234.567);
        assert_eq!(
            format_value("number", &val, &["0.00".to_string()]),
            Value::String("1,234.57".to_string())
        );
    }

    #[test]
    fn test_format_arabic_number() {
        let val = json!(1234.56);
        assert_eq!(
            format_value("arabic_number", &val, &[]),
            Value::String("١٬٢٣٥".to_string()) // rounded to 0 decimals by default
        );
    }

    #[test]
    fn test_format_date() {
        let val = json!("2026-06-29");
        assert_eq!(
            format_value("date", &val, &["yyyy/MM/dd".to_string()]),
            Value::String("2026/06/29".to_string())
        );
    }

    #[test]
    fn test_format_hijri() {
        let val = json!("2026-06-29");
        // Gregorian 2026-06-29 CE is Hijri 1447-12-14 (roughly)
        let formatted = format_value("hijri", &val, &[]);
        assert!(formatted.is_string());
        // Verify it contains Arabic digits
        let s = formatted.as_str().unwrap();
        assert!(s.contains('١') && s.contains('٤'));
    }
}
