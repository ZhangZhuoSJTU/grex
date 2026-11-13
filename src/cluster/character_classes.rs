use crate::char_range::CharRange;
use crate::config::RegExpConfig;
use crate::grapheme::Grapheme;
use crate::unicode_tables::{DECIMAL_NUMBER, WHITE_SPACE, WORD};
use itertools::Itertools;
use std::sync::LazyLock;

pub(super) fn convert_to_char_classes(graphemes: &mut [Grapheme], config: &RegExpConfig) {
    let is_digit_converted = config.is_digit_converted;
    let is_non_digit_converted = config.is_non_digit_converted;
    let is_space_converted = config.is_space_converted;
    let is_non_space_converted = config.is_non_space_converted;
    let is_word_converted = config.is_word_converted;
    let is_non_word_converted = config.is_non_word_converted;

    for grapheme in graphemes.iter_mut() {
        grapheme.chars = grapheme
            .chars
            .iter()
            .map(|it| {
                it.chars()
                    .map(|c| {
                        if is_digit_converted && is_digit(c) {
                            "\\d".to_string()
                        } else if is_word_converted && is_word(c) {
                            "\\w".to_string()
                        } else if is_space_converted && is_space(c) {
                            "\\s".to_string()
                        } else if is_non_digit_converted && !is_digit(c) {
                            "\\D".to_string()
                        } else if is_non_word_converted && !is_word(c) {
                            "\\W".to_string()
                        } else if is_non_space_converted && !is_space(c) {
                            "\\S".to_string()
                        } else {
                            c.to_string()
                        }
                    })
                    .join("")
            })
            .collect_vec();
    }
}

fn is_digit(c: char) -> bool {
    static VALID_NUMERIC_CHARS: LazyLock<Vec<CharRange>> =
        LazyLock::new(|| convert_chars_to_range(DECIMAL_NUMBER));
    VALID_NUMERIC_CHARS.iter().any(|range| range.contains(c))
}

fn is_word(c: char) -> bool {
    static VALID_ALPHANUMERIC_CHARS: LazyLock<Vec<CharRange>> =
        LazyLock::new(|| convert_chars_to_range(WORD));
    VALID_ALPHANUMERIC_CHARS
        .iter()
        .any(|range| range.contains(c))
}

fn is_space(c: char) -> bool {
    static VALID_SPACE_CHARS: LazyLock<Vec<CharRange>> =
        LazyLock::new(|| convert_chars_to_range(WHITE_SPACE));
    VALID_SPACE_CHARS.iter().any(|range| range.contains(c))
}

fn convert_chars_to_range(chars: &[(char, char)]) -> Vec<CharRange> {
    chars
        .iter()
        .map(|&(start, end)| CharRange::closed(start, end))
        .collect_vec()
}
