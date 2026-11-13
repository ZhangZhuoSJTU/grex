use crate::config::RegExpConfig;
use crate::grapheme::Grapheme;
use itertools::Itertools;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::ops::Range;

pub(super) fn convert_repetitions(
    graphemes: &[Grapheme],
    repetitions: &mut Vec<Grapheme>,
    config: &RegExpConfig,
) {
    let repeated_substrings = collect_repeated_substrings(graphemes);
    let ranges_of_repetitions = create_ranges_of_repetitions(repeated_substrings, config);
    let coalesced_repetitions = coalesce_repetitions(ranges_of_repetitions);
    replace_graphemes_with_repetitions(coalesced_repetitions, graphemes, repetitions, config)
}

fn collect_repeated_substrings(graphemes: &[Grapheme]) -> HashMap<Vec<String>, Vec<usize>> {
    let mut map = HashMap::new();

    for i in 0..graphemes.len() {
        let suffix = &graphemes[i..];
        for j in 1..=graphemes.len() / 2 {
            if suffix.len() >= j {
                let prefix = suffix[..j].iter().map(|it| it.value()).collect_vec();
                let indices = map.entry(prefix).or_insert_with(Vec::new);
                indices.push(i);
            }
        }
    }
    map
}

fn create_ranges_of_repetitions(
    repeated_substrings: HashMap<Vec<String>, Vec<usize>>,
    config: &RegExpConfig,
) -> Vec<(Range<usize>, Vec<String>)> {
    let mut repetitions = Vec::<(Range<usize>, Vec<String>)>::new();

    for (prefix_length, group) in &repeated_substrings
        .iter()
        .filter(|&(prefix, indices)| {
            indices
                .iter()
                .tuple_windows()
                .all(|(first, second)| (second - first) >= prefix.len())
        })
        .sorted_by_key(|&(prefix, _)| prefix.len())
        .rev()
        .chunk_by(|&(prefix, _)| prefix.len())
    {
        for (prefix, indices) in group.sorted_by_key(|&(_, indices)| indices[0]) {
            indices
                .iter()
                .map(|it| *it..it + prefix_length)
                .coalesce(|x, y| {
                    if x.end == y.start {
                        Ok(x.start..y.end)
                    } else {
                        Err((x, y))
                    }
                })
                .filter(|range| {
                    let count = ((range.end - range.start) / prefix_length) as u32;
                    count > config.minimum_repetitions
                })
                .for_each(|range| repetitions.push((range, prefix.clone())));
        }
    }
    repetitions
}

fn coalesce_repetitions(
    ranges_of_repetitions: Vec<(Range<usize>, Vec<String>)>,
) -> Vec<(Range<usize>, Vec<String>)> {
    ranges_of_repetitions
        .iter()
        .sorted_by(|&(first_range, _), &(second_range, _)| {
            match second_range.end.cmp(&first_range.end) {
                Ordering::Equal => first_range.start.cmp(&second_range.start),
                other => other,
            }
        })
        .coalesce(|first_tup, second_tup| {
            let first_range = &first_tup.0;
            let second_range = &second_tup.0;

            if (first_range.contains(&second_range.start)
                || first_range.contains(&second_range.end))
                && second_range.end != first_range.start
            {
                Ok(first_tup)
            } else {
                Err((first_tup, second_tup))
            }
        })
        .map(|(range, substr)| (range.clone(), substr.clone()))
        .collect_vec()
}

fn replace_graphemes_with_repetitions(
    coalesced_repetitions: Vec<(Range<usize>, Vec<String>)>,
    graphemes: &[Grapheme],
    repetitions: &mut Vec<Grapheme>,
    config: &RegExpConfig,
) {
    if coalesced_repetitions.is_empty() {
        return;
    }

    for grapheme in graphemes {
        repetitions.push(grapheme.clone());
    }

    for (range, substr) in coalesced_repetitions.iter() {
        if range.end > repetitions.len() {
            break;
        }

        let count = ((range.end - range.start) / substr.len()) as u32;

        if substr.len() < config.minimum_substring_length as usize {
            continue;
        }

        repetitions.splice(
            range.clone(),
            [Grapheme::from_repeated_unit(
                substr.clone(),
                count,
                config.is_capturing_group_enabled,
                config.is_output_colorized,
                config.is_verbose_mode_enabled,
            )]
            .iter()
            .cloned(),
        );
    }

    for new_grapheme in repetitions.iter_mut() {
        convert_repetitions(
            &new_grapheme
                .chars
                .iter()
                .map(|it| {
                    Grapheme::from(
                        it,
                        config.is_capturing_group_enabled,
                        config.is_output_colorized,
                        config.is_verbose_mode_enabled,
                    )
                })
                .collect_vec(),
            new_grapheme.repetitions.as_mut(),
            config,
        );
    }
}
