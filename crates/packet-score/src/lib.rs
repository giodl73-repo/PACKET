use std::collections::BTreeMap;

use packet_corpus::CorpusEntry;
use serde::{Deserialize, Serialize};

/// One of the thirteen PACKET scoring dimensions.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum Dimension {
    Dim01,
    Dim02,
    Dim03,
    Dim04,
    Dim05,
    Dim06,
    Dim07,
    Dim08,
    Dim09,
    Dim10,
    Dim11,
    Dim12,
    Dim13,
}

impl Dimension {
    /// Stable string code for this dimension, e.g. `DIM-01`.
    pub fn code(&self) -> &'static str {
        match self {
            Dimension::Dim01 => "DIM-01",
            Dimension::Dim02 => "DIM-02",
            Dimension::Dim03 => "DIM-03",
            Dimension::Dim04 => "DIM-04",
            Dimension::Dim05 => "DIM-05",
            Dimension::Dim06 => "DIM-06",
            Dimension::Dim07 => "DIM-07",
            Dimension::Dim08 => "DIM-08",
            Dimension::Dim09 => "DIM-09",
            Dimension::Dim10 => "DIM-10",
            Dimension::Dim11 => "DIM-11",
            Dimension::Dim12 => "DIM-12",
            Dimension::Dim13 => "DIM-13",
        }
    }

    /// Every dimension variant, in canonical order.
    pub fn all() -> [Dimension; 13] {
        [
            Dimension::Dim01,
            Dimension::Dim02,
            Dimension::Dim03,
            Dimension::Dim04,
            Dimension::Dim05,
            Dimension::Dim06,
            Dimension::Dim07,
            Dimension::Dim08,
            Dimension::Dim09,
            Dimension::Dim10,
            Dimension::Dim11,
            Dimension::Dim12,
            Dimension::Dim13,
        ]
    }
}

/// A scalar score constrained to the inclusive range `0.0..=10.0`.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Score(f64);

impl Score {
    const MIN: f64 = 0.0;
    const MAX: f64 = 10.0;

    /// Construct a `Score`, returning `None` when `value` is out of range.
    pub fn new(value: f64) -> Option<Score> {
        if (Self::MIN..=Self::MAX).contains(&value) {
            Some(Score(value))
        } else {
            None
        }
    }

    /// Construct a `Score`, clamping `value` into the valid range.
    pub fn clamped(value: f64) -> Score {
        Score(value.clamp(Self::MIN, Self::MAX))
    }

    /// The underlying scalar value.
    pub fn value(&self) -> f64 {
        self.0
    }
}

/// A versioned weighting of every scoring dimension.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Rubric {
    pub version: String,
    pub weights: BTreeMap<Dimension, f64>,
}

impl Rubric {
    /// The baseline `v0` rubric assigning equal weight to every dimension.
    pub fn v0() -> Rubric {
        let mut weights = BTreeMap::new();
        for dimension in Dimension::all() {
            weights.insert(dimension, 1.0);
        }
        Rubric {
            version: "v0".to_string(),
            weights,
        }
    }

    /// A short recorded rationale describing this rubric.
    pub fn rationale(&self) -> &str {
        "v0 baseline: every dimension weighted equally pending calibration"
    }
}

/// Produces a [`Score`] for a corpus entry along a single dimension.
pub trait DimensionScorer {
    fn score(&self, entry: &CorpusEntry, dimension: Dimension) -> Score;
}

/// A scorer that reads raw values from a corpus entry's `scores` map.
///
/// The scores it produces are provisional: they are derived directly from
/// recorded values and clamped into range, without further calibration.
#[derive(Clone, Debug)]
pub struct ProvisionalScorer {
    pub rubric: Rubric,
}

impl DimensionScorer for ProvisionalScorer {
    fn score(&self, entry: &CorpusEntry, dimension: Dimension) -> Score {
        let value = entry.scores.get(dimension.code()).copied().unwrap_or(0.0);
        Score::clamped(value)
    }
}

impl Default for ProvisionalScorer {
    fn default() -> Self {
        ProvisionalScorer {
            rubric: Rubric::v0(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn score_new_enforces_bounds() {
        assert!(Score::new(10.5).is_none());
        assert!(Score::new(-0.5).is_none());
        let inside = Score::new(5.0).unwrap();
        assert_eq!(inside.value(), 5.0);
    }

    #[test]
    fn score_clamped_maps_into_range() {
        assert_eq!(Score::clamped(11.0).value(), 10.0);
        assert_eq!(Score::clamped(-1.0).value(), 0.0);
        assert_eq!(Score::clamped(7.5).value(), 7.5);
    }

    #[test]
    fn rubric_v0_is_complete() {
        let rubric = Rubric::v0();
        assert_eq!(rubric.version, "v0");
        for dimension in Dimension::all() {
            assert_eq!(rubric.weights.get(&dimension).copied(), Some(1.0));
        }
        assert!(!rubric.rationale().is_empty());
    }

    #[test]
    fn provisional_scorer_reads_bounded_score() {
        let scorer = ProvisionalScorer::default();
        let mut entry = CorpusEntry::default();
        entry
            .scores
            .insert(Dimension::Dim01.code().to_string(), 12.0);
        let score = scorer.score(&entry, Dimension::Dim01);
        assert_eq!(score.value(), 10.0);
    }

    #[test]
    fn provisional_scorer_defaults_absent_dimension_to_zero() {
        let scorer = ProvisionalScorer::default();
        let entry = CorpusEntry::default();
        let score = scorer.score(&entry, Dimension::Dim07);
        assert_eq!(score.value(), 0.0);
    }
}
