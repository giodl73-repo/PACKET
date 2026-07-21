//! Gap analysis for the PACKET project.
//!
//! This crate locates regions of a [`packet_corpus`] corpus whose evidence
//! scores fall below the adequacy threshold for one or more scoring
//! dimensions, and folds in any tier SLA gaps that apply to the same entries.
//!
//! Cross-scale analysis requires `cross_scale` to be set to `true`. When it is
//! `false`, [`find_gaps`] only considers entries whose scale matches the
//! requested [`packet_corpus::Scale`].

use serde::{Deserialize, Serialize};

use packet_score::DimensionScorer;

/// A region of the evidence space where the mean score for a single dimension
/// falls below the adequacy threshold for a given scale.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GapRegion {
    pub dimension: packet_score::Dimension,
    pub scale: packet_corpus::Scale,
    pub mean_score: f64,
    pub member_ids: Vec<String>,
    pub label: packet_corpus::EvidenceLabel,
}

/// A dimension whose *under-served tail* falls below the adequacy threshold even
/// if the corpus mean does not. This catches intra-corpus inequity that the
/// mean-based [`GapRegion`] test misses on a corpus split between well- and
/// under-served entries.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TailGapRegion {
    pub dimension: packet_score::Dimension,
    pub scale: packet_corpus::Scale,
    /// Mean score of the bottom-quartile (worst-served) entries.
    pub tail_mean: f64,
    /// Ids of entries scoring below the adequacy threshold on this dimension.
    pub tail_member_ids: Vec<String>,
    /// Fraction of scored entries below threshold. A small share is a genuine
    /// tail (act on `tail_member_ids`); a large share is a systemic deficit.
    pub share_below_threshold: f64,
    /// True when the share crosses [`SYSTEMIC_SHARE`]: the deficit is the
    /// majority, so "tail" understates it and the whole class needs the upgrade.
    pub systemic: bool,
    pub label: packet_corpus::EvidenceLabel,
}

/// The result of a gap analysis over a corpus at a particular scale.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GapAnalysis {
    pub scale: packet_corpus::Scale,
    pub regions: Vec<GapRegion>,
    pub tail_regions: Vec<TailGapRegion>,
    pub tier_sla_gaps: Vec<packet_tier::TierSlaGap>,
    pub null_result: bool,
}

/// Adequacy threshold: scores at or above this are considered adequate.
const ADEQUACY_THRESHOLD: f64 = 5.0;

/// Share of scored entries below threshold at or above which a dispersion gap is
/// reclassified from a concentrated *tail* to a *systemic* deficit.
const SYSTEMIC_SHARE: f64 = 0.5;

/// Find evidence gaps in `corpus` for the requested `scale`.
///
/// When `cross_scale` is `false`, only entries whose scale equals
/// `Some(scale)` are analysed; cross-scale analysis (considering every entry
/// regardless of its own scale) requires `cross_scale` to be `true`.
///
/// Each selected entry is scored on every [`packet_score::Dimension`] using a
/// [`packet_score::ProvisionalScorer`]. For each dimension whose mean score
/// across the selected entries is strictly below `5.0`, a [`GapRegion`] is
/// emitted. Tier SLA gaps whose `entry_id` belongs to a selected entry are
/// collected as well. The analysis is a null result when neither regions nor
/// tier gaps are produced.
///
/// The `rubric` parameter is retained for provenance.
pub fn find_gaps(
    corpus: &[packet_corpus::CorpusEntry],
    rubric: &packet_score::Rubric,
    scale: packet_corpus::Scale,
    tier_gaps: &[packet_tier::TierSlaGap],
    cross_scale: bool,
) -> GapAnalysis {
    let _ = rubric;

    let scorer = packet_score::ProvisionalScorer::default();

    let selected: Vec<&packet_corpus::CorpusEntry> = corpus
        .iter()
        .filter(|entry| cross_scale || entry.scale == Some(scale))
        .collect();

    let selected_ids: Vec<String> = selected.iter().map(|entry| entry.id.clone()).collect();

    let mut regions = Vec::new();
    let mut tail_regions = Vec::new();
    if !selected.is_empty() {
        let count = selected.len() as f64;
        for dimension in packet_score::Dimension::all() {
            let mut scored: Vec<(&str, f64)> = selected
                .iter()
                .map(|entry| (entry.id.as_str(), scorer.score(entry, dimension).value()))
                .collect();
            let mean = scored.iter().map(|(_, value)| value).sum::<f64>() / count;
            if mean < ADEQUACY_THRESHOLD {
                regions.push(GapRegion {
                    dimension,
                    scale,
                    mean_score: mean,
                    member_ids: selected_ids.clone(),
                    label: packet_corpus::EvidenceLabel::Provisional,
                });
            }
            // Tail (dispersion) gap: even when the mean clears the bar, flag the
            // dimension if its worst-served quartile is inadequate.
            let under: Vec<String> = scored
                .iter()
                .filter(|(_, value)| *value < ADEQUACY_THRESHOLD)
                .map(|(id, _)| (*id).to_string())
                .collect();
            if !under.is_empty() {
                scored.sort_by(|a, b| a.1.total_cmp(&b.1));
                let quartile = selected.len().div_ceil(4).max(1);
                let tail_mean = scored
                    .iter()
                    .take(quartile)
                    .map(|(_, value)| value)
                    .sum::<f64>()
                    / quartile as f64;
                if tail_mean < ADEQUACY_THRESHOLD {
                    let share = under.len() as f64 / selected.len() as f64;
                    tail_regions.push(TailGapRegion {
                        dimension,
                        scale,
                        tail_mean,
                        tail_member_ids: under,
                        share_below_threshold: share,
                        systemic: share >= SYSTEMIC_SHARE,
                        label: packet_corpus::EvidenceLabel::Provisional,
                    });
                }
            }
        }
    }

    let tier_sla_gaps: Vec<packet_tier::TierSlaGap> = tier_gaps
        .iter()
        .filter(|gap| selected_ids.contains(&gap.entry_id))
        .cloned()
        .collect();

    let null_result = regions.is_empty() && tail_regions.is_empty() && tier_sla_gaps.is_empty();

    GapAnalysis {
        scale,
        regions,
        tail_regions,
        tier_sla_gaps,
        null_result,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeMap;

    fn entry_with_uniform_scores(
        id: &str,
        scale: packet_corpus::Scale,
        value: f64,
    ) -> packet_corpus::CorpusEntry {
        let mut scores = BTreeMap::new();
        for dimension in packet_score::Dimension::all() {
            scores.insert(dimension.code().to_string(), value);
        }
        packet_corpus::CorpusEntry {
            id: id.to_string(),
            scale: Some(scale),
            scores,
            ..Default::default()
        }
    }

    fn entry_empty_scores(id: &str, scale: packet_corpus::Scale) -> packet_corpus::CorpusEntry {
        packet_corpus::CorpusEntry {
            id: id.to_string(),
            scale: Some(scale),
            ..Default::default()
        }
    }

    #[test]
    fn low_scores_yield_at_least_one_region() {
        let corpus = vec![
            entry_empty_scores("e1", packet_corpus::Scale::Regional),
            entry_with_uniform_scores("e2", packet_corpus::Scale::Regional, 1.0),
        ];
        let rubric = packet_score::Rubric::v0();

        let analysis = find_gaps(&corpus, &rubric, packet_corpus::Scale::Regional, &[], false);

        assert!(!analysis.regions.is_empty());
        assert!(!analysis.null_result);
        let region = analysis.regions.first().unwrap();
        assert_eq!(region.label, packet_corpus::EvidenceLabel::Provisional);
        assert!(region.member_ids.contains(&"e1".to_string()));
        assert!(region.member_ids.contains(&"e2".to_string()));
    }

    #[test]
    fn adequate_market_is_null_result() {
        let corpus = vec![
            entry_with_uniform_scores("e1", packet_corpus::Scale::National, 7.0),
            entry_with_uniform_scores("e2", packet_corpus::Scale::National, 5.0),
        ];
        let rubric = packet_score::Rubric::v0();

        let analysis = find_gaps(&corpus, &rubric, packet_corpus::Scale::National, &[], false);

        assert!(analysis.regions.is_empty());
        assert!(analysis.tier_sla_gaps.is_empty());
        assert!(analysis.null_result);
    }

    #[test]
    fn split_corpus_flags_tail_gap_even_when_mean_clears_bar() {
        // Two under-served (1.0) and two well-served (9.0) entries: the per-
        // dimension mean is 5.0 (not < 5.0) so no mean-based GapRegion fires, but
        // the under-served tail must still be flagged.
        let corpus = vec![
            entry_with_uniform_scores("low1", packet_corpus::Scale::Regional, 1.0),
            entry_with_uniform_scores("low2", packet_corpus::Scale::Regional, 1.0),
            entry_with_uniform_scores("high1", packet_corpus::Scale::Regional, 9.0),
            entry_with_uniform_scores("high2", packet_corpus::Scale::Regional, 9.0),
        ];
        let rubric = packet_score::Rubric::v0();

        let analysis = find_gaps(&corpus, &rubric, packet_corpus::Scale::Regional, &[], false);

        assert!(
            analysis.regions.is_empty(),
            "mean is 5.0, not below the bar"
        );
        assert!(!analysis.tail_regions.is_empty(), "the tail is inadequate");
        assert!(!analysis.null_result);
        let tail = analysis.tail_regions.first().unwrap();
        assert!(tail.tail_mean < 5.0);
        assert!(tail.tail_member_ids.contains(&"low1".to_string()));
        assert!(!tail.tail_member_ids.contains(&"high1".to_string()));
    }

    #[test]
    fn adequate_market_has_no_tail_gap() {
        let corpus = vec![
            entry_with_uniform_scores("e1", packet_corpus::Scale::National, 7.0),
            entry_with_uniform_scores("e2", packet_corpus::Scale::National, 5.0),
        ];
        let rubric = packet_score::Rubric::v0();

        let analysis = find_gaps(&corpus, &rubric, packet_corpus::Scale::National, &[], false);

        assert!(analysis.regions.is_empty());
        assert!(analysis.tail_regions.is_empty());
        assert!(analysis.null_result);
    }

    #[test]
    fn tail_share_classifies_minority_vs_systemic() {
        // 1 under-served of 4 (25%) is a genuine tail; 3 of 4 (75%) is systemic.
        let minority = vec![
            entry_with_uniform_scores("low1", packet_corpus::Scale::Regional, 1.0),
            entry_with_uniform_scores("hi1", packet_corpus::Scale::Regional, 9.0),
            entry_with_uniform_scores("hi2", packet_corpus::Scale::Regional, 9.0),
            entry_with_uniform_scores("hi3", packet_corpus::Scale::Regional, 9.0),
        ];
        let rubric = packet_score::Rubric::v0();
        let a = find_gaps(
            &minority,
            &rubric,
            packet_corpus::Scale::Regional,
            &[],
            false,
        );
        let tail = a.tail_regions.first().expect("tail present");
        assert!((tail.share_below_threshold - 0.25).abs() < 1e-9);
        assert!(!tail.systemic);

        let majority = vec![
            entry_with_uniform_scores("low1", packet_corpus::Scale::Regional, 1.0),
            entry_with_uniform_scores("low2", packet_corpus::Scale::Regional, 1.0),
            entry_with_uniform_scores("low3", packet_corpus::Scale::Regional, 1.0),
            entry_with_uniform_scores("hi1", packet_corpus::Scale::Regional, 9.0),
        ];
        let b = find_gaps(
            &majority,
            &rubric,
            packet_corpus::Scale::Regional,
            &[],
            false,
        );
        let tail = b.tail_regions.first().expect("tail present");
        assert!((tail.share_below_threshold - 0.75).abs() < 1e-9);
        assert!(tail.systemic);
    }
}
