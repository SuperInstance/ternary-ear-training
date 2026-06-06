# ternary-ear-training

**Pattern recognition training for ternary agents.**

Inspired by musical ear training — where musicians learn to recognize intervals,
chords, and rhythms by ear — this crate teaches agents to recognize and reproduce
patterns in ternary (−1, 0, +1) sequences by shape.

## Overview

In music, ear training develops a musician's ability to identify pitches, intervals,
chords, and rhythms purely by listening. This crate applies the same principle to
ternary agent systems: instead of hearing notes, agents "hear" ternary sequences and
learn to recognize their structural patterns.

A ternary system uses three values: `Neg` (−1), `Zero` (0), and `Pos` (+1). These
values can represent agent states, decision outcomes, or weight configurations. The
patterns formed by sequences of these trits have characteristic shapes — just as
musical phrases have recognizable contours.

## Core Components

### IntervalTrainer

Analogous to interval ear training in music. Teaches agents to recognize the
transition between consecutive trits in a sequence.

- **Unison**: Same trit repeated (e.g., Pos → Pos)
- **Minor Second**: Step by one (e.g., Neg → Zero)
- **Major Second**: Step by two (e.g., Neg → Pos)
- **Minor/Major Second Descending**: Steps in the negative direction

Tracks accuracy per interval type and overall accuracy.

### ChordRecognizer

Analogous to chord identification. Recognizes named multi-trit patterns in sequences.

Standard chord vocabulary includes 10 patterns:

| Chord Name   | Pattern          | Description                    |
|-------------|------------------|--------------------------------|
| power       | `[+, 0, −]`      | Classic symmetric descent      |
| inverse-power | `[−, 0, +]`    | Symmetric ascent               |
| major-triad | `[+, +, 0]`      | Strong positive start          |
| minor-triad | `[−, −, 0]`      | Strong negative start          |
| suspended   | `[0, +, 0]`      | Positive pulse                 |
| suspended-neg | `[0, −, 0]`    | Negative pulse                 |
| augmented   | `[+, +, +]`      | Maximum ascent                 |
| diminished  | `[−, −, −]`      | Maximum descent                |
| neutral     | `[0, 0, 0]`      | Flat line                      |
| rising-fifth | `[−, 0, +, 0]`  | Extended rise                  |

Supports greedy matching (longest chord first) and custom chord addition.

### DictationExercise

Analogous to musical dictation. The agent hears a ternary sequence and must reproduce
it from memory. Sequences vary in length based on difficulty:

- **Beginner**: 4 trits
- **Intermediate**: 8 trits
- **Advanced**: 16 trits

Scoring uses exact match per position. Passing threshold is 80% accuracy.

### ProgressTracker

Tracks mastery across difficulty levels and pattern types. Records:

- Exercise completion and pass rates per difficulty
- Interval mastery (≥90% accuracy = mastered)
- Chord mastery (≥90% accuracy = mastered)
- Overall progress as a composite score
- Recommends next difficulty based on performance

## Usage

```rust
use ternary_ear_training::*;

// Train interval recognition
let mut trainer = IntervalTrainer::new();
let correct = trainer.quiz(Trit::Neg, Trit::Pos, Interval::MajorSecond);
assert!(correct);

// Recognize chords
let recognizer = ChordRecognizer::new();
let seq = vec![Trit::Pos, Trit::Zero, Trit::Neg];
let chord = recognizer.identify(&seq);
assert_eq!(chord, Some(("power", 3)));

// Dictation exercise
let exercise = DictationExercise::generate(Difficulty::Beginner, 42);
let attempt = exercise.target().to_vec(); // perfect recall
assert!(exercise.passed(&attempt));

// Track progress
let mut tracker = ProgressTracker::new();
tracker.record_exercise(Difficulty::Beginner, true);
tracker.record_interval_mastery(Interval::Unison, 0.95);
assert!(tracker.is_interval_mastered(Interval::Unison));
```

## Design Philosophy

This crate treats ternary pattern recognition as a trainable skill, not a fixed
algorithm. Like a musician who improves with practice, an agent using this crate
can progressively learn to recognize more complex patterns, track which areas need
work, and systematically advance through difficulty levels.

The interval/chord/dictation metaphor maps cleanly to ternary systems:

- **Intervals** = pairwise transitions (the "harmony" between adjacent states)
- **Chords** = multi-trit patterns (the "harmony" of a group of states)
- **Dictation** = full sequence reproduction (the "melody" of a state trajectory)

## Testing

The crate includes comprehensive tests covering all components:

```bash
cargo test
```

All 34 tests pass, covering trit basics, interval classification, chord recognition,
dictation scoring, progress tracking, and difficulty scaling.

## License

MIT
