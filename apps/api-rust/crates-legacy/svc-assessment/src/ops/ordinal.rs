//! Ordinal Formatting — localized ranking representation.
//!
//! Ranking is stored as a numeric integer. This module provides
//! presentation utilities for converting numeric ranks to localized
//! ordinal text.
//!
//! English: 1st, 2nd, 3rd, 4th, 5th, …, 21st, 22nd, 23rd, …
//! Arabic:  الأول، الثاني، الثالث، الرابع، …

/// Format a numeric rank as an English ordinal string.
///
/// Examples: 1 → "1st", 2 → "2nd", 3 → "3rd", 11 → "11th", 21 → "21st"
pub fn to_english_ordinal(rank: i32) -> String {
    let suffix = match (rank % 10, rank % 100) {
        (1, 11) => "th",
        (2, 12) => "th",
        (3, 13) => "th",
        (1, _) => "st",
        (2, _) => "nd",
        (3, _) => "rd",
        _ => "th",
    };
    format!("{}{}", rank, suffix)
}

/// Format a numeric rank as an Arabic ordinal string.
///
/// For ranks 1–10, returns the Arabic ordinal word.
/// For ranks > 10, returns the Arabic numeral with ordinal indicator.
///
/// Examples: 1 → "الأول", 2 → "الثاني", 11 → "الحادي عشر", 25 → "الـ25"
pub fn to_arabic_ordinal(rank: i32) -> String {
    match rank {
        1 => "الأول".to_string(),
        2 => "الثاني".to_string(),
        3 => "الثالث".to_string(),
        4 => "الرابع".to_string(),
        5 => "الخامس".to_string(),
        6 => "السادس".to_string(),
        7 => "السابع".to_string(),
        8 => "الثامن".to_string(),
        9 => "التاسع".to_string(),
        10 => "العاشر".to_string(),
        11 => "الحادي عشر".to_string(),
        12 => "الثاني عشر".to_string(),
        13 => "الثالث عشر".to_string(),
        14 => "الرابع عشر".to_string(),
        15 => "الخامس عشر".to_string(),
        16 => "السادس عشر".to_string(),
        17 => "السابع عشر".to_string(),
        18 => "الثامن عشر".to_string(),
        19 => "التاسع عشر".to_string(),
        20 => "العشرون".to_string(),
        _ => format!("الـ{}", rank),
    }
}

/// Format a rank for a given locale.
///
/// Supported locales: "en", "ar". Defaults to numeric for unknown locales.
pub fn format_rank(rank: i32, locale: &str) -> String {
    match locale {
        "ar" => to_arabic_ordinal(rank),
        "en" => to_english_ordinal(rank),
        _ => rank.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_english_ordinals() {
        assert_eq!(to_english_ordinal(1), "1st");
        assert_eq!(to_english_ordinal(2), "2nd");
        assert_eq!(to_english_ordinal(3), "3rd");
        assert_eq!(to_english_ordinal(4), "4th");
        assert_eq!(to_english_ordinal(10), "10th");
        assert_eq!(to_english_ordinal(11), "11th");
        assert_eq!(to_english_ordinal(12), "12th");
        assert_eq!(to_english_ordinal(13), "13th");
        assert_eq!(to_english_ordinal(21), "21st");
        assert_eq!(to_english_ordinal(22), "22nd");
        assert_eq!(to_english_ordinal(23), "23rd");
        assert_eq!(to_english_ordinal(100), "100th");
        assert_eq!(to_english_ordinal(101), "101st");
        assert_eq!(to_english_ordinal(111), "111th");
        assert_eq!(to_english_ordinal(112), "112th");
    }

    #[test]
    fn test_arabic_ordinals() {
        assert_eq!(to_arabic_ordinal(1), "الأول");
        assert_eq!(to_arabic_ordinal(2), "الثاني");
        assert_eq!(to_arabic_ordinal(3), "الثالث");
        assert_eq!(to_arabic_ordinal(10), "العاشر");
        assert_eq!(to_arabic_ordinal(11), "الحادي عشر");
        assert_eq!(to_arabic_ordinal(20), "العشرون");
        assert_eq!(to_arabic_ordinal(25), "الـ25");
    }

    #[test]
    fn test_format_rank_locale() {
        assert_eq!(format_rank(1, "en"), "1st");
        assert_eq!(format_rank(1, "ar"), "الأول");
        assert_eq!(format_rank(5, "fr"), "5"); // unknown locale → numeric
    }
}
