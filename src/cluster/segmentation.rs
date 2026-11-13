use crate::config::RegExpConfig;
use crate::grapheme::Grapheme;
use itertools::Itertools;
use unicode_general_category::GeneralCategory as GC;
use unicode_segmentation::UnicodeSegmentation;

pub(super) fn segment(s: &str, config: &RegExpConfig) -> Vec<Grapheme> {
    UnicodeSegmentation::graphemes(s, true)
                .flat_map(|it| {
                    let contains_backslash = it.chars().count() == 2 && it.contains('\\');
                    let contains_combining_mark_or_unassigned_chars = it.chars().any(|c| {
                        let category = unicode_general_category::get_general_category(c);
                        matches!(
                            category,
                            // Mark categories
                            GC::NonspacingMark | GC::SpacingMark | GC::EnclosingMark |
                            // Other categories
                            GC::Control | GC::Format | GC::Surrogate | GC::PrivateUse | GC::Unassigned
                        )
                    });

                    if contains_backslash || contains_combining_mark_or_unassigned_chars {
                        it.chars()
                            .map(|c| {
                                Grapheme::from(
                                    &c.to_string(),
                                    config.is_capturing_group_enabled,
                                    config.is_output_colorized,
                                    config.is_verbose_mode_enabled,
                                )
                            })
                            .collect_vec()
                    } else {
                        vec![Grapheme::from(
                            it,
                            config.is_capturing_group_enabled,
                            config.is_output_colorized,
                            config.is_verbose_mode_enabled,
                        )]
                    }
                })
                .collect_vec()
}
