/*
 * Copyright © 2019-today Peter M. Stahl pemistahl@gmail.com
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either expressed or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use crate::config::RegExpConfig;
use crate::grapheme::Grapheme;

mod character_classes;
mod repetitions;
mod segmentation;

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct GraphemeCluster<'a> {
    graphemes: Vec<Grapheme>,
    config: &'a RegExpConfig,
}

impl<'a> GraphemeCluster<'a> {
    pub(crate) fn from(s: &str, config: &'a RegExpConfig) -> Self {
        Self {
            graphemes: segmentation::segment(s, config),
            config,
        }
    }

    pub(crate) fn from_graphemes(graphemes: Vec<Grapheme>, config: &'a RegExpConfig) -> Self {
        Self { graphemes, config }
    }

    pub(crate) fn new(grapheme: Grapheme, config: &'a RegExpConfig) -> Self {
        Self {
            graphemes: vec![grapheme],
            config,
        }
    }

    pub(crate) fn convert_to_char_classes(&mut self) {
        character_classes::convert_to_char_classes(&mut self.graphemes, self.config);
    }

    pub(crate) fn convert_repetitions(&mut self) {
        let mut repetitions = vec![];
        repetitions::convert_repetitions(self.graphemes(), repetitions.as_mut(), self.config);
        if !repetitions.is_empty() {
            self.graphemes = repetitions;
        }
    }

    pub(crate) fn merge(
        first: &GraphemeCluster,
        second: &GraphemeCluster,
        config: &'a RegExpConfig,
    ) -> Self {
        let mut graphemes = vec![];
        graphemes.extend_from_slice(&first.graphemes);
        graphemes.extend_from_slice(&second.graphemes);
        Self { graphemes, config }
    }

    pub(crate) fn graphemes(&self) -> &Vec<Grapheme> {
        &self.graphemes
    }

    pub(crate) fn graphemes_mut(&mut self) -> &mut Vec<Grapheme> {
        &mut self.graphemes
    }

    pub(crate) fn size(&self) -> usize {
        self.graphemes.len()
    }

    pub(crate) fn char_count(&self, is_non_ascii_char_escaped: bool) -> usize {
        self.graphemes
            .iter()
            .map(|it| it.char_count(is_non_ascii_char_escaped))
            .sum()
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.graphemes.is_empty()
    }
}
