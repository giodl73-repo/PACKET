use std::collections::BTreeMap;
use std::fmt;

use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Geographic scale at which a corpus entry applies.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Scale {
    International,
    National,
    Regional,
    Local,
}

impl Scale {
    /// Parse a scale from its lowercase name, returning `None` if unrecognized.
    pub fn parse(s: &str) -> Option<Scale> {
        match s {
            "international" => Some(Scale::International),
            "national" => Some(Scale::National),
            "regional" => Some(Scale::Regional),
            "local" => Some(Scale::Local),
            _ => None,
        }
    }
}

impl fmt::Display for Scale {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            Scale::International => "international",
            Scale::National => "national",
            Scale::Regional => "regional",
            Scale::Local => "local",
        };
        f.write_str(name)
    }
}

/// Provenance label describing how strongly a quantity is evidenced.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum EvidenceLabel {
    Estimated,
    Cited,
    Validated,
    Provisional,
}

impl EvidenceLabel {
    /// Parse an evidence label from its lowercase name; unknown values are
    /// treated as `Estimated` (the weakest claim).
    pub fn parse(s: &str) -> EvidenceLabel {
        match s.trim().to_ascii_lowercase().as_str() {
            "cited" => EvidenceLabel::Cited,
            "validated" => EvidenceLabel::Validated,
            "provisional" => EvidenceLabel::Provisional,
            _ => EvidenceLabel::Estimated,
        }
    }
}

/// Parse a pipe-delimited corpus quantity line: `value | unit | label | source_id`.
///
/// `label` is an [`EvidenceLabel`] name; `source_id` is optional and an empty
/// source id yields an uncited quantity.
fn parse_quantity_line(value: &str) -> Option<Quantity> {
    let mut parts = value.split('|').map(str::trim);
    let number: f64 = parts.next()?.parse().ok()?;
    let unit = parts.next().unwrap_or("").to_string();
    let label = EvidenceLabel::parse(parts.next().unwrap_or("estimated"));
    let source_id = match parts.next() {
        Some(source) if !source.is_empty() => Some(source.to_string()),
        _ => None,
    };
    Some(Quantity {
        value: number,
        unit,
        label,
        source_id,
    })
}

/// Basis used when interpreting a load quantity.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum LoadBasis {
    Peak,
    Average,
}

/// A single measured or estimated quantity attached to a corpus entry.
///
/// A `Quantity` whose `source_id` is `None` is considered uncited.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Quantity {
    pub value: f64,
    pub unit: String,
    pub label: EvidenceLabel,
    pub source_id: Option<String>,
}

/// A single entry in the corpus.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct CorpusEntry {
    pub id: String,
    pub kind: String,
    pub scale: Option<Scale>,
    pub market: String,
    pub tier: Option<String>,
    pub sla: Option<String>,
    pub quantities: Vec<Quantity>,
    pub scores: BTreeMap<String, f64>,
}

/// Errors that prevent a corpus entry from being processed at all.
#[derive(Debug, Error)]
pub enum CorpusError {
    #[error("corpus entry is missing a required, non-empty `id` field")]
    MissingId,
}

/// Reasons an otherwise-valid entry must be held for review rather than published.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum HoldReason {
    UncitedQuantity(String),
    MissingScale,
}

impl CorpusEntry {
    /// Validate the entry without mutating it.
    ///
    /// Returns `Err(CorpusError::MissingId)` for an empty `id`. Otherwise returns
    /// the (possibly empty) list of reasons the entry should be held. Validation
    /// never mutates the entry or changes any evidence label.
    pub fn validate(&self) -> Result<Vec<HoldReason>, CorpusError> {
        if self.id.is_empty() {
            return Err(CorpusError::MissingId);
        }

        let mut reasons = Vec::new();

        if self.scale.is_none() {
            reasons.push(HoldReason::MissingScale);
        }

        for quantity in &self.quantities {
            if quantity.source_id.is_none() {
                reasons.push(HoldReason::UncitedQuantity(quantity.unit.clone()));
            }
        }

        Ok(reasons)
    }

    /// Build an entry from a leading frontmatter block delimited by lines that
    /// contain exactly three dashes (`---`).
    ///
    /// Recognized `key: value` lines are `id`, `kind` (also `type`), `scale`,
    /// `market`, and `tier`. Quantities and scores are left empty. Returns
    /// `Err(CorpusError::MissingId)` when `id` is absent or empty.
    pub fn from_markdown(input: &str) -> Result<CorpusEntry, CorpusError> {
        let mut entry = CorpusEntry::default();
        let mut started = false;

        for line in input.lines() {
            let trimmed = line.trim();

            if trimmed == "---" {
                if started {
                    break;
                }
                started = true;
                continue;
            }

            if !started {
                continue;
            }

            if let Some((key, value)) = trimmed.split_once(':') {
                let key = key.trim();
                let value = value.trim();
                match key {
                    "id" => entry.id = value.to_string(),
                    "kind" | "type" => entry.kind = value.to_string(),
                    "scale" => entry.scale = Scale::parse(value),
                    "market" => entry.market = value.to_string(),
                    "tier" => entry.tier = Some(value.to_string()),
                    "sla" => entry.sla = Some(value.to_string()),
                    "quantity" => {
                        if let Some(quantity) = parse_quantity_line(value) {
                            entry.quantities.push(quantity);
                        }
                    }
                    other => {
                        // `score.DIM-01: 8.4` records a provisional dimension score.
                        if let Some(code) = other.strip_prefix("score.") {
                            if let Ok(parsed) = value.parse::<f64>() {
                                entry.scores.insert(code.trim().to_string(), parsed);
                            }
                        }
                    }
                }
            }
        }

        if entry.id.is_empty() {
            return Err(CorpusError::MissingId);
        }

        Ok(entry)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_id_is_rejected() {
        let entry = CorpusEntry::default();
        let result = entry.validate();
        assert!(matches!(result, Err(CorpusError::MissingId)));
    }

    #[test]
    fn missing_scale_is_held() {
        let entry = CorpusEntry {
            id: "e1".to_string(),
            scale: None,
            ..Default::default()
        };
        let reasons = entry.validate().unwrap();
        assert!(reasons.contains(&HoldReason::MissingScale));
    }

    #[test]
    fn uncited_quantity_is_held() {
        let entry = CorpusEntry {
            id: "e1".to_string(),
            scale: Some(Scale::Regional),
            quantities: vec![Quantity {
                value: 12.5,
                unit: "kW".to_string(),
                label: EvidenceLabel::Estimated,
                source_id: None,
            }],
            ..Default::default()
        };
        let reasons = entry.validate().unwrap();
        assert!(reasons.contains(&HoldReason::UncitedQuantity("kW".to_string())));
    }

    #[test]
    fn validate_does_not_change_evidence_label() {
        let entry = CorpusEntry {
            id: "e1".to_string(),
            scale: Some(Scale::Local),
            quantities: vec![Quantity {
                value: 3.0,
                unit: "MW".to_string(),
                label: EvidenceLabel::Estimated,
                source_id: None,
            }],
            ..Default::default()
        };
        let _ = entry.validate().unwrap();
        assert_eq!(entry.quantities[0].label, EvidenceLabel::Estimated);
    }

    #[test]
    fn from_markdown_parses_frontmatter() {
        let input = "---\nid: e1\nkind: datacenter\nscale: regional\nmarket: us\ntier: gold\n---\nbody text\n";
        let entry = CorpusEntry::from_markdown(input).unwrap();
        assert_eq!(entry.id, "e1");
        assert_eq!(entry.scale, Some(Scale::Regional));
    }

    #[test]
    fn from_markdown_parses_scores_and_quantities() {
        let input = "---\nid: us-ms\nscale: regional\nmarket: Mississippi\n\
            score.DIM-01: 2.0\n\
            quantity: 86.5 | percent | cited | acs-s2801-2023\n\
            quantity: 50.0 | percent | estimated |\n---\n";
        let entry = CorpusEntry::from_markdown(input).unwrap();
        assert_eq!(entry.scores.get("DIM-01"), Some(&2.0));
        assert_eq!(entry.quantities.len(), 2);
        assert_eq!(entry.quantities[0].value, 86.5);
        assert_eq!(entry.quantities[0].label, EvidenceLabel::Cited);
        assert_eq!(
            entry.quantities[0].source_id.as_deref(),
            Some("acs-s2801-2023")
        );
        // An empty source id yields an uncited quantity.
        assert!(entry.quantities[1].source_id.is_none());
    }
}
