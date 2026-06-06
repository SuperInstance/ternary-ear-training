//! # Ternary Ear Training
//!
//! Pattern recognition training for ternary agents. Inspired by musical ear training —
//! where musicians learn to recognize intervals, chords, and rhythms by ear — this crate
//! teaches agents to recognize and reproduce patterns in ternary sequences by shape.
//!
//! ## Core Concepts
//!
//! - **Trit**: A ternary digit, one of `Trit::Neg(-1)`, `Trit::Zero(0)`, or `Trit::Pos(1)`.
//! - **Interval**: The relationship between two consecutive trits in a sequence.
//! - **Chord**: A recognizable multi-trit pattern (analogous to a musical chord).
//! - **Dictation**: Reproducing a heard ternary sequence from memory.
//! - **Difficulty**: Beginner (short, simple), Intermediate, Advanced (long, complex).

#![forbid(unsafe_code)]

use std::collections::HashMap;

/// A single ternary digit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Trit {
    Neg,
    Zero,
    Pos,
}

impl Trit {
    /// Numeric value: -1, 0, or +1.
    pub fn value(&self) -> i8 {
        match self {
            Trit::Neg => -1,
            Trit::Zero => 0,
            Trit::Pos => 1,
        }
    }

    /// All possible trit values.
    pub fn all() -> &'static [Trit; 3] {
        &[Trit::Neg, Trit::Zero, Trit::Pos]
    }

    /// Create from i8 value.
    pub fn from_i8(v: i8) -> Option<Self> {
        match v {
            -1 => Some(Trit::Neg),
            0 => Some(Trit::Zero),
            1 => Some(Trit::Pos),
            _ => None,
        }
    }
}

/// Difficulty level for exercises.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Difficulty {
    Beginner,
    Intermediate,
    Advanced,
}

impl Difficulty {
    /// Default sequence length for this difficulty.
    pub fn default_seq_len(&self) -> usize {
        match self {
            Difficulty::Beginner => 4,
            Difficulty::Intermediate => 8,
            Difficulty::Advanced => 16,
        }
    }

    /// Number of unique chord types to test at this level.
    pub fn chord_count(&self) -> usize {
        match self {
            Difficulty::Beginner => 3,
            Difficulty::Intermediate => 6,
            Difficulty::Advanced => 10,
        }
    }
}

/// The interval between two consecutive trits.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Interval {
    /// Same trit repeated (e.g., Pos → Pos).
    Unison,
    /// Step up by one (Neg→Zero, Zero→Pos).
    MinorSecond,
    /// Step up by two (Neg→Pos).
    MajorSecond,
    /// Step down by one (Pos→Zero, Zero→Neg).
    MinorSecondDesc,
    /// Step down by two (Pos→Neg).
    MajorSecondDesc,
}

impl Interval {
    /// Classify the interval between two trits.
    pub fn between(a: Trit, b: Trit) -> Self {
        let diff = b.value() as i8 - a.value() as i8;
        match diff {
            0 => Interval::Unison,
            1 => Interval::MinorSecond,
            2 => Interval::MajorSecond,
            -1 => Interval::MinorSecondDesc,
            -2 => Interval::MajorSecondDesc,
            _ => unreachable!("ternary diff is always in [-2, 2]"),
        }
    }

    /// All interval types.
    pub fn all() -> &'static [Interval; 5] {
        &[
            Interval::Unison,
            Interval::MinorSecond,
            Interval::MajorSecond,
            Interval::MinorSecondDesc,
            Interval::MajorSecondDesc,
        ]
    }

    /// Human-readable name.
    pub fn name(&self) -> &'static str {
        match self {
            Interval::Unison => "unison",
            Interval::MinorSecond => "minor second (up)",
            Interval::MajorSecond => "major second (up)",
            Interval::MinorSecondDesc => "minor second (down)",
            Interval::MajorSecondDesc => "major second (down)",
        }
    }
}

// ── IntervalTrainer ──────────────────────────────────────────────────────────

/// Trains recognition of intervals (transitions) in ternary sequences.
#[derive(Debug, Clone)]
pub struct IntervalTrainer {
    /// Histogram of correctly identified intervals per type.
    correct: HashMap<Interval, u32>,
    /// Histogram of incorrect attempts per type.
    incorrect: HashMap<Interval, u32>,
}

impl IntervalTrainer {
    pub fn new() -> Self {
        Self {
            correct: HashMap::new(),
            incorrect: HashMap::new(),
        }
    }

    /// Present a training pair: given `first`, the agent must identify the interval
    /// to `second`. Returns whether the guess was correct.
    pub fn quiz(&mut self, first: Trit, second: Trit, guess: Interval) -> bool {
        let actual = Interval::between(first, second);
        if guess == actual {
            *self.correct.entry(actual).or_insert(0) += 1;
            true
        } else {
            *self.incorrect.entry(actual).or_insert(0) += 1;
            false
        }
    }

    /// Accuracy for a specific interval type (0.0 – 1.0).
    pub fn accuracy_for(&self, interval: Interval) -> f64 {
        let c = self.correct.get(&interval).copied().unwrap_or(0) as f64;
        let i = self.incorrect.get(&interval).copied().unwrap_or(0) as f64;
        if c + i == 0.0 {
            0.0
        } else {
            c / (c + i)
        }
    }

    /// Overall accuracy across all interval types.
    pub fn overall_accuracy(&self) -> f64 {
        let total_correct: u32 = self.correct.values().sum();
        let total_incorrect: u32 = self.incorrect.values().sum();
        let total = total_correct + total_incorrect;
        if total == 0 {
            0.0
        } else {
            total_correct as f64 / total as f64
        }
    }

    /// Total number of quiz attempts.
    pub fn total_attempts(&self) -> u32 {
        self.correct.values().sum::<u32>() + self.incorrect.values().sum::<u32>()
    }

    /// Extract the interval sequence from a ternary sequence.
    pub fn extract_intervals(sequence: &[Trit]) -> Vec<Interval> {
        sequence
            .windows(2)
            .map(|w| Interval::between(w[0], w[1]))
            .collect()
    }
}

impl Default for IntervalTrainer {
    fn default() -> Self {
        Self::new()
    }
}

// ── ChordRecognizer ──────────────────────────────────────────────────────────

/// A named ternary chord pattern.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Chord {
    pub name: String,
    pub pattern: Vec<Trit>,
}

impl Chord {
    pub fn new(name: impl Into<String>, pattern: Vec<Trit>) -> Self {
        Self {
            name: name.into(),
            pattern,
        }
    }

    /// Match this chord against the start of a sequence.
    pub fn matches_prefix(&self, seq: &[Trit]) -> bool {
        seq.len() >= self.pattern.len() && seq[..self.pattern.len()] == self.pattern[..]
    }
}

/// Recognizes chord patterns in ternary sequences.
#[derive(Debug, Clone)]
pub struct ChordRecognizer {
    known_chords: Vec<Chord>,
}

impl ChordRecognizer {
    pub fn new() -> Self {
        Self {
            known_chords: Self::standard_chords(),
        }
    }

    /// Standard ternary chord vocabulary.
    pub fn standard_chords() -> Vec<Chord> {
        use Trit::*;
        vec![
            Chord::new("power", vec![Pos, Zero, Neg]),
            Chord::new("inverse-power", vec![Neg, Zero, Pos]),
            Chord::new("major-triad", vec![Pos, Pos, Zero]),
            Chord::new("minor-triad", vec![Neg, Neg, Zero]),
            Chord::new("suspended", vec![Zero, Pos, Zero]),
            Chord::new("suspended-neg", vec![Zero, Neg, Zero]),
            Chord::new("augmented", vec![Pos, Pos, Pos]),
            Chord::new("diminished", vec![Neg, Neg, Neg]),
            Chord::new("neutral", vec![Zero, Zero, Zero]),
            Chord::new("rising-fifth", vec![Neg, Zero, Pos, Zero]),
        ]
    }

    /// Identify the first chord in a sequence. Returns the matched chord name and its length.
    pub fn identify(&self, seq: &[Trit]) -> Option<(&str, usize)> {
        // Try longest chords first for greedy matching.
        let mut best: Option<(&str, usize)> = None;
        for chord in &self.known_chords {
            if chord.matches_prefix(seq) {
                let len = chord.pattern.len();
                match best {
                    None => best = Some((&chord.name, len)),
                    Some((_, bl)) if len > bl => best = Some((&chord.name, len)),
                    _ => {}
                }
            }
        }
        best
    }

    /// Identify all chords in a sequence, greedily.
    pub fn identify_all(&self, seq: &[Trit]) -> Vec<(&str, usize)> {
        let mut results = Vec::new();
        let mut pos = 0;
        while pos < seq.len() {
            if let Some((name, len)) = self.identify(&seq[pos..]) {
                results.push((name, len));
                pos += len;
            } else {
                pos += 1; // skip unrecognized trit
            }
        }
        results
    }

    /// Add a custom chord to the vocabulary.
    pub fn add_chord(&mut self, chord: Chord) {
        self.known_chords.push(chord);
    }

    /// Number of known chords.
    pub fn chord_count(&self) -> usize {
        self.known_chords.len()
    }
}

impl Default for ChordRecognizer {
    fn default() -> Self {
        Self::new()
    }
}

// ── DictationExercise ────────────────────────────────────────────────────────

/// A dictation exercise: reproduce a heard ternary sequence from memory.
#[derive(Debug, Clone)]
pub struct DictationExercise {
    /// The target sequence to reproduce.
    target: Vec<Trit>,
    /// Difficulty level.
    difficulty: Difficulty,
}

impl DictationExercise {
    /// Create a new dictation exercise with the given target sequence.
    pub fn new(target: Vec<Trit>, difficulty: Difficulty) -> Self {
        Self { target, difficulty }
    }

    /// Generate a random exercise at the given difficulty.
    pub fn generate(difficulty: Difficulty, seed: u64) -> Self {
        let len = difficulty.default_seq_len();
        let mut rng = simple_rng(len, seed);
        let target: Vec<Trit> = (0..len)
            .map(|_| match rng.next() % 3 {
                0 => Trit::Neg,
                1 => Trit::Zero,
                _ => Trit::Pos,
            })
            .collect();
        Self { target, difficulty }
    }

    /// Score a reproduction attempt against the target.
    /// Returns (correct_count, total_count, accuracy).
    pub fn score(&self, attempt: &[Trit]) -> (usize, usize, f64) {
        let total = self.target.len();
        let correct = self
            .target
            .iter()
            .zip(attempt.iter())
            .filter(|(a, b)| a == b)
            .count();
        let accuracy = if total == 0 {
            1.0
        } else {
            correct as f64 / total as f64
        };
        (correct, total, accuracy)
    }

    /// Grade the attempt as pass/fail (>= 80% = pass).
    pub fn passed(&self, attempt: &[Trit]) -> bool {
        self.score(attempt).2 >= 0.8
    }

    pub fn target(&self) -> &[Trit] {
        &self.target
    }

    pub fn difficulty(&self) -> Difficulty {
        self.difficulty
    }
}

/// Simple deterministic RNG for exercise generation.
struct SimpleRng {
    state: u64,
}

fn simple_rng(_len: usize, seed: u64) -> SimpleRng {
    SimpleRng { state: seed }
}

impl SimpleRng {
    fn next(&mut self) -> u64 {
        // xorshift64
        self.state ^= self.state << 13;
        self.state ^= self.state >> 7;
        self.state ^= self.state << 17;
        self.state
    }
}

// ── ProgressTracker ──────────────────────────────────────────────────────────

/// Tracks mastery of ternary patterns across difficulty levels.
#[derive(Debug, Clone)]
pub struct ProgressTracker {
    /// Number of exercises completed per difficulty.
    completed: HashMap<Difficulty, u32>,
    /// Number of exercises passed (>= 80%) per difficulty.
    passed: HashMap<Difficulty, u32>,
    /// Mastered interval types.
    mastered_intervals: HashMap<Interval, bool>,
    /// Mastered chord names.
    mastered_chords: HashMap<String, bool>,
}

impl ProgressTracker {
    pub fn new() -> Self {
        Self {
            completed: HashMap::new(),
            passed: HashMap::new(),
            mastered_intervals: HashMap::new(),
            mastered_chords: HashMap::new(),
        }
    }

    /// Record the result of an exercise.
    pub fn record_exercise(&mut self, difficulty: Difficulty, did_pass: bool) {
        *self.completed.entry(difficulty).or_insert(0) += 1;
        if did_pass {
            *self.passed.entry(difficulty).or_insert(0) += 1;
        }
    }

    /// Record interval mastery (>= 90% accuracy considered mastered).
    pub fn record_interval_mastery(&mut self, interval: Interval, accuracy: f64) {
        self.mastered_intervals
            .insert(interval, accuracy >= 0.9);
    }

    /// Record chord mastery.
    pub fn record_chord_mastery(&mut self, chord_name: impl Into<String>, accuracy: f64) {
        self.mastered_chords
            .insert(chord_name.into(), accuracy >= 0.9);
    }

    /// Pass rate for a difficulty level.
    pub fn pass_rate(&self, difficulty: Difficulty) -> f64 {
        let c = self.completed.get(&difficulty).copied().unwrap_or(0) as f64;
        let p = self.passed.get(&difficulty).copied().unwrap_or(0) as f64;
        if c == 0.0 { 0.0 } else { p / c }
    }

    /// Whether a specific interval is mastered.
    pub fn is_interval_mastered(&self, interval: Interval) -> bool {
        self.mastered_intervals.get(&interval).copied().unwrap_or(false)
    }

    /// Whether a specific chord is mastered.
    pub fn is_chord_mastered(&self, chord_name: &str) -> bool {
        self.mastered_chords.get(chord_name).copied().unwrap_or(false)
    }

    /// Count of mastered intervals.
    pub fn mastered_interval_count(&self) -> usize {
        self.mastered_intervals.values().filter(|b| **b).count()
    }

    /// Count of mastered chords.
    pub fn mastered_chord_count(&self) -> usize {
        self.mastered_chords.values().filter(|b| **b).count()
    }

    /// Overall progress summary (0.0 – 1.0).
    pub fn overall_progress(&self) -> f64 {
        let interval_progress = self.mastered_interval_count() as f64 / Interval::all().len() as f64;
        let chord_count = 10; // standard chords
        let chord_progress = self.mastered_chord_count() as f64 / chord_count as f64;
        (interval_progress + chord_progress) / 2.0
    }

    /// Recommend the next difficulty level based on performance.
    pub fn recommend_difficulty(&self) -> Difficulty {
        if self.pass_rate(Difficulty::Advanced) >= 0.8 {
            Difficulty::Advanced
        } else if self.pass_rate(Difficulty::Intermediate) >= 0.8 {
            Difficulty::Advanced
        } else if self.pass_rate(Difficulty::Beginner) >= 0.8 {
            Difficulty::Intermediate
        } else {
            Difficulty::Beginner
        }
    }
}

impl Default for ProgressTracker {
    fn default() -> Self {
        Self::new()
    }
}

// ══════════════════════════════════════════════════════════════════════════════
// Tests
// ══════════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trit_values() {
        assert_eq!(Trit::Neg.value(), -1);
        assert_eq!(Trit::Zero.value(), 0);
        assert_eq!(Trit::Pos.value(), 1);
    }

    #[test]
    fn trit_from_i8() {
        assert_eq!(Trit::from_i8(-1), Some(Trit::Neg));
        assert_eq!(Trit::from_i8(0), Some(Trit::Zero));
        assert_eq!(Trit::from_i8(1), Some(Trit::Pos));
        assert_eq!(Trit::from_i8(2), None);
    }

    #[test]
    fn trit_all() {
        assert_eq!(Trit::all().len(), 3);
    }

    // ── Interval tests ──

    #[test]
    fn interval_classification() {
        assert_eq!(Interval::between(Trit::Pos, Trit::Pos), Interval::Unison);
        assert_eq!(Interval::between(Trit::Neg, Trit::Zero), Interval::MinorSecond);
        assert_eq!(Interval::between(Trit::Neg, Trit::Pos), Interval::MajorSecond);
        assert_eq!(Interval::between(Trit::Pos, Trit::Zero), Interval::MinorSecondDesc);
        assert_eq!(Interval::between(Trit::Pos, Trit::Neg), Interval::MajorSecondDesc);
    }

    #[test]
    fn interval_all_count() {
        assert_eq!(Interval::all().len(), 5);
    }

    #[test]
    fn interval_names() {
        for interval in Interval::all() {
            assert!(!interval.name().is_empty());
        }
    }

    #[test]
    fn extract_intervals_from_sequence() {
        use Trit::*;
        let seq = vec![Pos, Pos, Zero, Neg, Pos];
        let intervals = IntervalTrainer::extract_intervals(&seq);
        assert_eq!(intervals.len(), 4);
        assert_eq!(intervals[0], Interval::Unison);
        assert_eq!(intervals[1], Interval::MinorSecondDesc);
        assert_eq!(intervals[2], Interval::MinorSecondDesc);
        assert_eq!(intervals[3], Interval::MajorSecond);
    }

    // ── IntervalTrainer tests ──

    #[test]
    fn interval_trainer_correct_quiz() {
        let mut trainer = IntervalTrainer::new();
        assert!(trainer.quiz(Trit::Neg, Trit::Pos, Interval::MajorSecond));
        assert!(trainer.quiz(Trit::Pos, Trit::Pos, Interval::Unison));
        assert_eq!(trainer.total_attempts(), 2);
        assert_eq!(trainer.overall_accuracy(), 1.0);
    }

    #[test]
    fn interval_trainer_incorrect_quiz() {
        let mut trainer = IntervalTrainer::new();
        assert!(!trainer.quiz(Trit::Neg, Trit::Pos, Interval::Unison));
        assert_eq!(trainer.overall_accuracy(), 0.0);
    }

    #[test]
    fn interval_trainer_mixed_accuracy() {
        let mut trainer = IntervalTrainer::new();
        trainer.quiz(Trit::Pos, Trit::Pos, Interval::Unison); // correct
        trainer.quiz(Trit::Pos, Trit::Pos, Interval::Unison); // correct
        trainer.quiz(Trit::Pos, Trit::Neg, Interval::Unison); // wrong (actual is MajorSecondDesc)
        // Overall: 2 correct out of 3 total
        assert!((trainer.overall_accuracy() - 2.0 / 3.0).abs() < 1e-9);
        // Unison specifically: 2 correct, 0 incorrect
        assert!((trainer.accuracy_for(Interval::Unison) - 1.0).abs() < 1e-9);
    }

    #[test]
    fn interval_trainer_empty_accuracy() {
        let trainer = IntervalTrainer::new();
        assert_eq!(trainer.overall_accuracy(), 0.0);
        assert_eq!(trainer.accuracy_for(Interval::Unison), 0.0);
    }

    // ── ChordRecognizer tests ──

    #[test]
    fn chord_matches_prefix() {
        use Trit::*;
        let chord = Chord::new("test", vec![Pos, Zero, Neg]);
        assert!(chord.matches_prefix(&[Pos, Zero, Neg, Pos]));
        assert!(!chord.matches_prefix(&[Pos, Zero, Pos]));
    }

    #[test]
    fn recognize_power_chord() {
        use Trit::*;
        let recognizer = ChordRecognizer::new();
        let result = recognizer.identify(&[Pos, Zero, Neg]);
        assert_eq!(result.map(|(n, l)| (n.to_string(), l)), Some(("power".to_string(), 3)));
    }

    #[test]
    fn recognize_augmented() {
        use Trit::*;
        let recognizer = ChordRecognizer::new();
        let result = recognizer.identify(&[Pos, Pos, Pos]);
        assert_eq!(result.map(|(n, _)| n.to_string()), Some("augmented".to_string()));
    }

    #[test]
    fn recognize_no_match() {
        use Trit::*;
        let recognizer = ChordRecognizer::new();
        let result = recognizer.identify(&[Neg, Pos]); // too short for any 3-trit chord
        assert!(result.is_none());
    }

    #[test]
    fn identify_all_chords() {
        use Trit::*;
        let recognizer = ChordRecognizer::new();
        let seq = vec![Pos, Zero, Neg, Neg, Neg, Zero]; // power + minor-triad
        let results = recognizer.identify_all(&seq);
        assert_eq!(results.len(), 2);
        assert_eq!(results[0].0, "power");
        assert_eq!(results[1].0, "minor-triad");
    }

    #[test]
    fn add_custom_chord() {
        use Trit::*;
        let mut recognizer = ChordRecognizer::new();
        let initial_count = recognizer.chord_count();
        recognizer.add_chord(Chord::new("custom", vec![Pos, Neg, Pos, Neg]));
        assert_eq!(recognizer.chord_count(), initial_count + 1);
        let result = recognizer.identify(&[Pos, Neg, Pos, Neg]);
        assert_eq!(result.map(|(n, _)| n.to_string()), Some("custom".to_string()));
    }

    #[test]
    fn standard_chords_count() {
        let chords = ChordRecognizer::standard_chords();
        assert_eq!(chords.len(), 10);
    }

    // ── DictationExercise tests ──

    #[test]
    fn dictation_perfect_score() {
        use Trit::*;
        let target = vec![Pos, Zero, Neg, Pos];
        let exercise = DictationExercise::new(target.clone(), Difficulty::Beginner);
        let (correct, total, accuracy) = exercise.score(&target);
        assert_eq!(correct, 4);
        assert_eq!(total, 4);
        assert_eq!(accuracy, 1.0);
    }

    #[test]
    fn dictation_partial_score() {
        use Trit::*;
        let exercise = DictationExercise::new(vec![Pos, Zero, Neg, Pos], Difficulty::Beginner);
        let attempt = vec![Pos, Zero, Pos, Pos];
        let (correct, total, accuracy) = exercise.score(&attempt);
        assert_eq!(correct, 3);
        assert_eq!(total, 4);
        assert!((accuracy - 0.75).abs() < 1e-9);
    }

    #[test]
    fn dictation_pass_fail() {
        use Trit::*;
        let exercise = DictationExercise::new(vec![Pos, Zero, Neg, Pos], Difficulty::Beginner);
        assert!(exercise.passed(&[Pos, Zero, Neg, Pos]));
        assert!(!exercise.passed(&[Pos, Zero, Neg, Neg])); // 3/4 = 75% — not passed
        assert!(!exercise.passed(&[Neg, Neg, Neg, Neg])); // 1/4 = 25%
    }

    #[test]
    fn dictation_generate_deterministic() {
        let a = DictationExercise::generate(Difficulty::Beginner, 42);
        let b = DictationExercise::generate(Difficulty::Beginner, 42);
        assert_eq!(a.target(), b.target());
    }

    #[test]
    fn dictation_generate_respects_difficulty() {
        let beginner = DictationExercise::generate(Difficulty::Beginner, 1);
        let intermediate = DictationExercise::generate(Difficulty::Intermediate, 1);
        let advanced = DictationExercise::generate(Difficulty::Advanced, 1);
        assert_eq!(beginner.target().len(), 4);
        assert_eq!(intermediate.target().len(), 8);
        assert_eq!(advanced.target().len(), 16);
    }

    #[test]
    fn dictation_difficulty_accessor() {
        let exercise = DictationExercise::generate(Difficulty::Advanced, 1);
        assert_eq!(exercise.difficulty(), Difficulty::Advanced);
    }

    // ── ProgressTracker tests ──

    #[test]
    fn progress_tracker_exercises() {
        let mut tracker = ProgressTracker::new();
        tracker.record_exercise(Difficulty::Beginner, true);
        tracker.record_exercise(Difficulty::Beginner, true);
        tracker.record_exercise(Difficulty::Beginner, false);
        assert!((tracker.pass_rate(Difficulty::Beginner) - 2.0 / 3.0).abs() < 1e-9);
    }

    #[test]
    fn progress_tracker_empty_pass_rate() {
        let tracker = ProgressTracker::new();
        assert_eq!(tracker.pass_rate(Difficulty::Beginner), 0.0);
    }

    #[test]
    fn progress_tracker_interval_mastery() {
        let mut tracker = ProgressTracker::new();
        assert!(!tracker.is_interval_mastered(Interval::Unison));
        tracker.record_interval_mastery(Interval::Unison, 0.95);
        assert!(tracker.is_interval_mastered(Interval::Unison));
        tracker.record_interval_mastery(Interval::MinorSecond, 0.85);
        assert!(!tracker.is_interval_mastered(Interval::MinorSecond));
    }

    #[test]
    fn progress_tracker_chord_mastery() {
        let mut tracker = ProgressTracker::new();
        assert!(!tracker.is_chord_mastered("power"));
        tracker.record_chord_mastery("power", 0.92);
        assert!(tracker.is_chord_mastered("power"));
        tracker.record_chord_mastery("augmented", 0.89);
        assert!(!tracker.is_chord_mastered("augmented"));
    }

    #[test]
    fn progress_tracker_overall_progress() {
        let mut tracker = ProgressTracker::new();
        assert_eq!(tracker.overall_progress(), 0.0);
        // Master all intervals
        for interval in Interval::all() {
            tracker.record_interval_mastery(*interval, 1.0);
        }
        // Progress should be 0.5 (intervals done, chords not)
        assert!((tracker.overall_progress() - 0.5).abs() < 1e-9);
    }

    #[test]
    fn progress_tracker_recommend_difficulty() {
        let mut tracker = ProgressTracker::new();
        assert_eq!(tracker.recommend_difficulty(), Difficulty::Beginner);
        // Pass enough beginner exercises
        for _ in 0..10 {
            tracker.record_exercise(Difficulty::Beginner, true);
        }
        assert_eq!(tracker.recommend_difficulty(), Difficulty::Intermediate);
    }

    #[test]
    fn progress_tracker_recommend_advanced() {
        let mut tracker = ProgressTracker::new();
        for _ in 0..10 {
            tracker.record_exercise(Difficulty::Intermediate, true);
        }
        assert_eq!(tracker.recommend_difficulty(), Difficulty::Advanced);
    }

    #[test]
    fn difficulty_ordering() {
        assert!(Difficulty::Beginner < Difficulty::Intermediate);
        assert!(Difficulty::Intermediate < Difficulty::Advanced);
    }

    #[test]
    fn difficulty_default_seq_lengths() {
        assert_eq!(Difficulty::Beginner.default_seq_len(), 4);
        assert_eq!(Difficulty::Intermediate.default_seq_len(), 8);
        assert_eq!(Difficulty::Advanced.default_seq_len(), 16);
    }

    #[test]
    fn difficulty_chord_counts() {
        assert_eq!(Difficulty::Beginner.chord_count(), 3);
        assert_eq!(Difficulty::Intermediate.chord_count(), 6);
        assert_eq!(Difficulty::Advanced.chord_count(), 10);
    }
}
