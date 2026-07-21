use serde::{Deserialize, Serialize};

use packet_corpus::{CorpusEntry, EvidenceLabel, LoadBasis};
use packet_network::Network;
use packet_score::Score;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Tier {
    T1,
    T2,
    T3,
    T4,
}

impl Tier {
    pub fn label(&self) -> &'static str {
        match self {
            Tier::T1 => "T1",
            Tier::T2 => "T2",
            Tier::T3 => "T3",
            Tier::T4 => "T4",
        }
    }
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Sla {
    pub capacity_mbps: f64,
    pub latency_ms: f64,
    pub availability: f64,
    pub affordability: f64,
}

pub fn provisional_sla(tier: Tier) -> Sla {
    match tier {
        Tier::T1 => Sla {
            capacity_mbps: 10000.0,
            latency_ms: 5.0,
            availability: 0.99999,
            affordability: 0.2,
        },
        Tier::T2 => Sla {
            capacity_mbps: 1000.0,
            latency_ms: 20.0,
            availability: 0.9999,
            affordability: 0.5,
        },
        Tier::T3 => Sla {
            capacity_mbps: 100.0,
            latency_ms: 50.0,
            availability: 0.999,
            affordability: 0.8,
        },
        Tier::T4 => Sla {
            capacity_mbps: 10.0,
            latency_ms: 100.0,
            availability: 0.99,
            affordability: 1.0,
        },
    }
}

pub fn classify(entry: &CorpusEntry) -> Tier {
    match entry.tier.as_deref() {
        Some("T1") => Tier::T1,
        Some("T2") => Tier::T2,
        Some("T3") => Tier::T3,
        Some("T4") => Tier::T4,
        _ => Tier::T4,
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Dim13 {
    pub score: Score,
    pub load_basis: LoadBasis,
    pub redundancy: bool,
}

fn observed_mbps(entry: &CorpusEntry) -> f64 {
    entry
        .quantities
        .iter()
        .find(|q| q.unit.to_lowercase().contains("mbps"))
        .map(|q| q.value)
        .unwrap_or(0.0)
}

pub fn conformance(entry: &CorpusEntry, network: &Network) -> Dim13 {
    let required = provisional_sla(classify(entry));
    let observed = observed_mbps(entry);
    let redundancy = matches!(network.degree(&entry.id), Some(d) if d >= 2);
    let mut result = (observed / required.capacity_mbps).min(1.0) * 10.0;
    if !redundancy {
        result -= 2.0;
    }
    Dim13 {
        score: Score::clamped(result),
        load_basis: LoadBasis::Peak,
        redundancy,
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TierSlaGap {
    pub entry_id: String,
    pub tier: Tier,
    pub required_mbps: f64,
    pub observed_mbps: f64,
    pub label: EvidenceLabel,
}

pub fn tier_sla_gap(entry: &CorpusEntry) -> Option<TierSlaGap> {
    let tier = classify(entry);
    let required = provisional_sla(tier);
    let observed = observed_mbps(entry);
    if observed < required.capacity_mbps {
        Some(TierSlaGap {
            entry_id: entry.id.clone(),
            tier,
            required_mbps: required.capacity_mbps,
            observed_mbps: observed,
            label: EvidenceLabel::Provisional,
        })
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use packet_corpus::Quantity;
    use packet_network::{Link, Node, NodeKind};

    fn quantity(value: f64, unit: &str) -> Quantity {
        Quantity {
            value,
            unit: unit.to_string(),
            label: EvidenceLabel::Cited,
            source_id: None,
        }
    }

    #[test]
    fn classify_maps_tiers_and_defaults() {
        for (s, t) in [
            ("T1", Tier::T1),
            ("T2", Tier::T2),
            ("T3", Tier::T3),
            ("T4", Tier::T4),
        ] {
            let e = CorpusEntry {
                tier: Some(s.to_string()),
                ..Default::default()
            };
            assert_eq!(classify(&e), t);
        }
        let none = CorpusEntry {
            tier: None,
            ..Default::default()
        };
        assert_eq!(classify(&none), Tier::T4);
    }

    #[test]
    fn conforming_element_no_gap_and_high_score() {
        let entry = CorpusEntry {
            id: "n".to_string(),
            tier: Some("T4".to_string()),
            quantities: vec![quantity(50.0, "Mbps")],
            ..Default::default()
        };
        let mut net = Network::new();
        net.add_node(Node {
            id: "n".to_string(),
            name: "n".to_string(),
            kind: NodeKind::Core,
        })
        .unwrap();
        net.add_node(Node {
            id: "b".to_string(),
            name: "b".to_string(),
            kind: NodeKind::Edge,
        })
        .unwrap();
        net.add_node(Node {
            id: "c".to_string(),
            name: "c".to_string(),
            kind: NodeKind::Edge,
        })
        .unwrap();
        net.add_link(
            "n",
            "b",
            Link {
                id: "l1".to_string(),
                capacity_mbps: 100.0,
                latency_ms: 1.0,
            },
        )
        .unwrap();
        net.add_link(
            "n",
            "c",
            Link {
                id: "l2".to_string(),
                capacity_mbps: 100.0,
                latency_ms: 1.0,
            },
        )
        .unwrap();

        assert!(tier_sla_gap(&entry).is_none());
        let dim = conformance(&entry, &net);
        assert!(dim.score.value() >= 9.0);
    }

    #[test]
    fn shortfall_yields_provisional_gap() {
        let entry = CorpusEntry {
            id: "n".to_string(),
            tier: Some("T2".to_string()),
            quantities: vec![quantity(100.0, "mbps")],
            ..Default::default()
        };
        let gap = tier_sla_gap(&entry).unwrap();
        assert!(matches!(gap.label, EvidenceLabel::Provisional));
        assert_eq!(gap.tier, Tier::T2);
        assert!(gap.observed_mbps < gap.required_mbps);
    }

    #[test]
    fn diverse_path_scores_higher_than_low_degree() {
        let make = |id: &str| CorpusEntry {
            id: id.to_string(),
            tier: Some("T4".to_string()),
            quantities: vec![quantity(100.0, "Mbps")],
            ..Default::default()
        };

        let mut net = Network::new();
        net.add_node(Node {
            id: "lonely".to_string(),
            name: "lonely".to_string(),
            kind: NodeKind::Edge,
        })
        .unwrap();
        net.add_node(Node {
            id: "hub".to_string(),
            name: "hub".to_string(),
            kind: NodeKind::Core,
        })
        .unwrap();
        net.add_node(Node {
            id: "b".to_string(),
            name: "b".to_string(),
            kind: NodeKind::Edge,
        })
        .unwrap();
        net.add_node(Node {
            id: "c".to_string(),
            name: "c".to_string(),
            kind: NodeKind::Exchange,
        })
        .unwrap();
        net.add_link(
            "lonely",
            "b",
            Link {
                id: "l1".to_string(),
                capacity_mbps: 100.0,
                latency_ms: 1.0,
            },
        )
        .unwrap();
        net.add_link(
            "hub",
            "b",
            Link {
                id: "l2".to_string(),
                capacity_mbps: 100.0,
                latency_ms: 1.0,
            },
        )
        .unwrap();
        net.add_link(
            "hub",
            "c",
            Link {
                id: "l3".to_string(),
                capacity_mbps: 100.0,
                latency_ms: 1.0,
            },
        )
        .unwrap();

        let low = conformance(&make("lonely"), &net);
        let high = conformance(&make("hub"), &net);
        assert!(!low.redundancy);
        assert!(high.redundancy);
        assert!(low.score.value() < high.score.value());
    }
}
