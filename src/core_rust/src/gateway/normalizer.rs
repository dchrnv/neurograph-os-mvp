use crate::bootstrap::BootstrapLibrary;
use crate::gateway::config::{GatewayConfig, UnknownWordStrategy};
use std::sync::Arc;
use parking_lot::RwLock;

/// Result of text normalization
#[derive(Debug, Clone)]
pub struct NormalizationResult {
    /// Final state vector [f32; 8]
    pub state: [f32; 8],
    /// Matched tokens: (word, token_id, confidence)
    pub matched_tokens: Vec<(String, u32, f32)>,
    /// Words that weren't found in bootstrap
    pub unknown_words: Vec<String>,
    /// Overall confidence of normalization
    pub confidence: f32,
}

/// Error during normalization
#[derive(Debug)]
pub enum NormalizationError {
    NoWords,
    AllUnknown,
    BootstrapLocked,
}

/// Normalizer converts text/tokens into state vectors
pub struct Normalizer {
    bootstrap: Arc<RwLock<BootstrapLibrary>>,
    config: GatewayConfig,
}

impl Normalizer {
    pub fn new(bootstrap: Arc<RwLock<BootstrapLibrary>>, config: GatewayConfig) -> Self {
        Self { bootstrap, config }
    }

    /// Normalize text into state vector
    pub fn normalize_text(&self, text: &str) -> Result<NormalizationResult, NormalizationError> {
        let words: Vec<&str> = text
            .split_whitespace()
            .filter(|w| !w.is_empty())
            .collect();

        if words.is_empty() {
            return Err(NormalizationError::NoWords);
        }

        let word_count = words.len();

        let bootstrap = self.bootstrap.read();

        let mut states: Vec<[f32; 8]> = Vec::new();
        let mut matched_tokens: Vec<(String, u32, f32)> = Vec::new();
        let mut unknown_words: Vec<String> = Vec::new();

        for word in words {
            let word_lower = word.to_lowercase();

            if let Some(concept) = bootstrap.get_concept(&word_lower) {
                // Known word - convert coords to state
                let state = self.coords_to_state(&concept.coords, concept.id);
                states.push(state);
                matched_tokens.push((word_lower.clone(), concept.id, 1.0));
            } else {
                // Unknown word - handle according to strategy
                if let Some(state) = self.handle_unknown_word(&word_lower) {
                    states.push(state);
                }
                unknown_words.push(word_lower);
            }
        }

        if states.is_empty() {
            return Err(NormalizationError::AllUnknown);
        }

        // Aggregate multiple states into one
        let final_state = self.aggregate_states(&states);

        // Calculate confidence based on known/unknown ratio
        let confidence = self.calculate_confidence(&states, word_count);

        Ok(NormalizationResult {
            state: final_state,
            matched_tokens,
            unknown_words,
            confidence,
        })
    }

    /// Convert 3D coordinates to 8D state vector
    fn coords_to_state(&self, coords: &[f32; 3], _token_id: u32) -> [f32; 8] {
        let mut state = [0.0; 8];

        // L1 Physical: Use coords directly
        state[0] = coords[0];
        state[1] = coords[1];
        state[2] = coords[2];

        // L4 Emotional: Could be enriched with emotion data if available
        // For now, leave at 0.0
        state[3] = 0.0;

        // L5 Social: Not used yet
        state[4] = 0.0;

        // L6 Conceptual: Use distance from origin as abstraction level
        let distance = (coords[0].powi(2) + coords[1].powi(2) + coords[2].powi(2)).sqrt();
        state[5] = distance.min(1.0);

        // L7 Temporal: Not used in this context
        state[6] = 0.0;

        // L8 Abstract: Use normalized z-coordinate as semantic category indicator
        state[7] = coords[2].abs().min(1.0);

        state
    }

    /// Handle unknown word according to strategy
    fn handle_unknown_word(&self, word: &str) -> Option<[f32; 8]> {
        match self.config.unknown_word_strategy {
            UnknownWordStrategy::Ignore => None,
            UnknownWordStrategy::CreateEmpty => Some([0.0; 8]),
            UnknownWordStrategy::TriggerCuriosity => {
                // TODO: In future, add to curiosity queue
                // For now, return empty state
                Some([0.0; 8])
            }
            UnknownWordStrategy::UseNearest => {
                // Find nearest word by edit distance or embedding similarity
                self.find_nearest(word)
                    .map(|(coords, token_id)| self.coords_to_state(&coords, token_id))
            }
        }
    }

    /// Find nearest known word (simple edit distance for now)
    /// Returns (coords, token_id)
    fn find_nearest(&self, _word: &str) -> Option<([f32; 3], u32)> {
        // TODO: Implement proper nearest neighbor search
        // Needs BootstrapLibrary API extension to iterate concepts
        // or provide a nearest_word() method
        None
    }

    /// Aggregate multiple states into single state (centroid)
    fn aggregate_states(&self, states: &[[f32; 8]]) -> [f32; 8] {
        if states.is_empty() {
            return [0.0; 8];
        }

        let mut result = [0.0; 8];
        let count = states.len() as f32;

        for state in states {
            for (i, val) in state.iter().enumerate() {
                result[i] += val;
            }
        }

        for val in result.iter_mut() {
            *val /= count;
        }

        result
    }

    /// Calculate confidence based on known/unknown ratio
    fn calculate_confidence(&self, states: &[[f32; 8]], total_words: usize) -> f32 {
        if total_words == 0 {
            return 0.0;
        }

        let known_words = states.len();
        known_words as f32 / total_words as f32
    }
}

/// Simple Levenshtein distance
fn edit_distance(s1: &str, s2: &str) -> usize {
    let len1 = s1.len();
    let len2 = s2.len();

    if len1 == 0 {
        return len2;
    }
    if len2 == 0 {
        return len1;
    }

    let mut matrix = vec![vec![0; len2 + 1]; len1 + 1];

    for i in 0..=len1 {
        matrix[i][0] = i;
    }
    for j in 0..=len2 {
        matrix[0][j] = j;
    }

    let s1_chars: Vec<char> = s1.chars().collect();
    let s2_chars: Vec<char> = s2.chars().collect();

    for (i, c1) in s1_chars.iter().enumerate() {
        for (j, c2) in s2_chars.iter().enumerate() {
            let cost = if c1 == c2 { 0 } else { 1 };
            matrix[i + 1][j + 1] = (matrix[i][j + 1] + 1)
                .min(matrix[i + 1][j] + 1)
                .min(matrix[i][j] + cost);
        }
    }

    matrix[len1][len2]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_edit_distance() {
        assert_eq!(edit_distance("cat", "cat"), 0);
        assert_eq!(edit_distance("cat", "cut"), 1);
        assert_eq!(edit_distance("cat", "dog"), 3);
        assert_eq!(edit_distance("", "test"), 4);
    }

    #[test]
    fn test_coords_to_state() {
        use crate::bootstrap::BootstrapConfig;
        let bootstrap = Arc::new(RwLock::new(BootstrapLibrary::new(BootstrapConfig::default())));
        let config = GatewayConfig::default();
        let normalizer = Normalizer::new(bootstrap, config);

        let coords = [0.5, -0.3, 0.8];
        let state = normalizer.coords_to_state(&coords, 0);

        assert_eq!(state[0], 0.5);
        assert_eq!(state[1], -0.3);
        assert_eq!(state[2], 0.8);
    }

    #[test]
    fn test_aggregate_states() {
        use crate::bootstrap::BootstrapConfig;
        let bootstrap = Arc::new(RwLock::new(BootstrapLibrary::new(BootstrapConfig::default())));
        let config = GatewayConfig::default();
        let normalizer = Normalizer::new(bootstrap, config);

        let states = vec![
            [1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
        ];

        let result = normalizer.aggregate_states(&states);
        assert_eq!(result[0], 0.5);
        assert_eq!(result[1], 0.5);
    }
}
