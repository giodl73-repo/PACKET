//! WP-WEB-001: first-party provisional consumer, not finding authority.
use packet_corpus::{CorpusEntry, Scale};
use packet_score::{Dimension, Rubric};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::BTreeSet;
pub const MAX_REQUEST: usize = 1024 * 1024;
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Request {
    scale: String,
    corpus_json: String,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Corpus {
    documents: Vec<Document>,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Document {
    name: String,
    markdown: String,
}
#[derive(Serialize)]
struct Record {
    name: String,
    status: &'static str,
    reasons: Vec<String>,
    warnings: Vec<String>,
    entry: Option<CorpusEntry>,
}
fn failure(phase: &str, code: &str, message: &str) -> Value {
    json!({"schema":"packet.workbench.v1","ok":false,"phase":phase,"diagnostic":{"code":code,"message":message}})
}
fn audit(text: &str, entry: &CorpusEntry) -> (Vec<String>, Vec<String>) {
    let (mut reasons, mut warnings, mut fields, mut quantities) =
        (Vec::new(), Vec::new(), BTreeSet::new(), 0usize);
    let mut lines = text.trim_start().lines();
    if lines.next() != Some("---") {
        reasons.push("Leading frontmatter block required".into());
        return (reasons, warnings);
    }
    let mut closed = false;
    for line in lines {
        if line.trim() == "---" {
            closed = true;
            break;
        }
        if line.trim().is_empty() {
            continue;
        }
        let Some((key, value)) = line.split_once(':') else {
            reasons.push("Malformed frontmatter line".into());
            continue;
        };
        let (key, value) = (key.trim(), value.trim());
        if key == "quantity" {
            quantities += 1;
            let label = value
                .split('|')
                .nth(2)
                .map(str::trim)
                .unwrap_or("estimated")
                .to_ascii_lowercase();
            if !matches!(
                label.as_str(),
                "estimated" | "cited" | "validated" | "provisional"
            ) {
                warnings.push(format!(
                    "Quantity label {label:?} defaults to Estimated in native parsing"
                ));
            }
            continue;
        }
        let normalized = if key == "type" { "kind" } else { key };
        if !fields.insert(normalized.to_string()) {
            reasons.push(format!("Duplicate frontmatter field {normalized}"));
        }
        if let Some(code) = key.strip_prefix("score.") {
            if !Dimension::all().iter().any(|d| d.code() == code) {
                reasons.push(format!("Unknown score dimension {code}"));
            }
            if value
                .parse::<f64>()
                .ok()
                .is_none_or(|n| !n.is_finite() || !(0.0..=10.0).contains(&n))
            {
                reasons.push(format!("Score {code} must be finite and within 0–10"));
            }
        } else if !matches!(
            key,
            "id" | "type" | "kind" | "scale" | "market" | "tier" | "sla"
        ) {
            warnings.push(format!("Native parser ignores frontmatter field {key}"));
        }
    }
    if !closed {
        reasons.push("Closing frontmatter delimiter required".into());
    }
    if quantities != entry.quantities.len() {
        reasons.push("Native parser omitted a malformed quantity; fix it before analysis".into());
    }
    (reasons, warnings)
}
fn record(document: Document) -> Record {
    let mut out = Record {
        name: document.name,
        status: "held",
        reasons: vec![],
        warnings: vec![],
        entry: None,
    };
    let entry = match CorpusEntry::from_markdown(&document.markdown) {
        Ok(entry) => entry,
        Err(e) => {
            out.status = "rejected";
            out.reasons.push(e.to_string());
            return out;
        }
    };
    if entry.quantities.iter().any(|q| !q.value.is_finite())
        || entry.scores.values().any(|n| !n.is_finite())
    {
        out.status = "rejected";
        out.reasons.push("Non-finite quantity or score".into());
        return out;
    }
    (out.reasons, out.warnings) = audit(&document.markdown, &entry);
    match entry.validate() {
        Ok(holds) => out
            .reasons
            .extend(holds.iter().map(|r| format!("Native hold: {r:?}"))),
        Err(e) => out.reasons.push(e.to_string()),
    }
    if entry.id.trim().is_empty() {
        out.reasons.push("Non-blank stable ID required".into());
    }
    if entry.market.trim().is_empty() {
        out.reasons.push("Market/jurisdiction required".into());
    }
    if entry.kind.trim().is_empty() {
        out.reasons.push("Corpus kind/type required".into());
    }
    if !matches!(entry.tier.as_deref(), Some("T1" | "T2" | "T3" | "T4")) {
        out.reasons.push("Explicit T1–T4 tier required".into());
    }
    if out.reasons.is_empty() {
        out.status = "admitted";
    }
    out.entry = Some(entry);
    out
}
fn authority() -> Value {
    json!({"research_only":true,"public_finding_release_approved":false,"downstream_owner_acceptance":false,"source_labels":"caller supplied; not independently verified","scores":"provisional, uncalibrated ratings","blocked_claims":["build or RF/optical design","regulatory, funding or procurement authority","advocacy or carrier commitment","FCC/NTIA/standards-body endorsement","complete market connectivity or SLA adequacy"]})
}
fn capacity(entry: &CorpusEntry) -> bool {
    entry
        .quantities
        .iter()
        .find(|q| q.unit.to_ascii_lowercase().contains("mbps"))
        .is_some_and(|q| q.unit.eq_ignore_ascii_case("mbps") && q.value >= 0.0)
}
// CR-001: one bounded report assembly keeps native output and its coverage
// context inseparable; parsing/admission and capacity policy are separate helpers.
fn analysis(records: Vec<Record>, scale: Scale) -> Value {
    let held = records.iter().filter(|r| r.status != "admitted").count();
    let mut selected: Vec<_> = records
        .iter()
        .filter_map(|r| r.entry.as_ref())
        .filter(|e| e.scale == Some(scale))
        .cloned()
        .collect();
    let mut report = json!({"schema":"packet.workbench.v1","ok":true,"scale":scale.to_string(),"authority":authority(),"records":records,"held_or_rejected":held,"analysis_json":null});
    if held > 0 {
        report["outcome"] = json!("admission-held");
        return report;
    }
    if selected.is_empty() {
        report["outcome"] = json!("insufficient-data");
        return report;
    }
    selected.sort_by(|a, b| a.id.cmp(&b.id));
    let complete: Vec<_> = Dimension::all()
        .iter()
        .filter(|d| selected.iter().all(|e| e.scores.contains_key(d.code())))
        .map(Dimension::code)
        .collect();
    let coverage:Vec<_>=Dimension::all().iter().map(|d|json!({"dimension":d.code(),"provided":selected.iter().filter(|e|e.scores.contains_key(d.code())).count(),"total":selected.len(),"native_missing_default":0})).collect();
    let tier_gaps: Vec<_> = selected
        .iter()
        .filter(|e| capacity(e))
        .filter_map(packet_tier::tier_sla_gap)
        .collect();
    let native = packet_gap::find_gaps(&selected, &Rubric::v0(), scale, &tier_gaps, false);
    let canonical = match serde_json::to_string(&native) {
        Ok(value) => value,
        Err(e) => return failure("analysis", "Serialization", &e.to_string()),
    };
    let mean: Vec<_> = native
        .regions
        .iter()
        .filter(|r| complete.contains(&r.dimension.code()))
        .collect();
    let tail: Vec<_> = native
        .tail_regions
        .iter()
        .filter(|r| complete.contains(&r.dimension.code()))
        .collect();
    let ids: BTreeSet<_> = selected.iter().map(|e| e.id.as_str()).collect();
    let excluded: Vec<_> = records
        .iter()
        .filter_map(|r| r.entry.as_ref())
        .filter(|e| !ids.contains(e.id.as_str()))
        .map(|e| e.id.as_str())
        .collect();
    let markets: BTreeSet<_> = selected.iter().map(|e| e.market.as_str()).collect();
    report["outcome"] = json!(
        if !mean.is_empty() || !tail.is_empty() || !tier_gaps.is_empty() {
            "provisional-signals"
        } else if complete.is_empty() {
            "incomplete-score-coverage"
        } else {
            "no-complete-dimension-flags"
        }
    );
    report["analysis_json"] = json!(canonical);
    report["selection"] =
        json!({"ids":ids,"markets":markets,"excluded_ids":excluded,"cross_scale":false});
    report["coverage"] = json!(coverage);
    report["summary"] = json!({"complete_dimensions":complete,"mean_regions":mean,"tail_regions":tail,"capacity_gaps":tier_gaps,"capacity_assessed_ids":selected.iter().filter(|e|capacity(e)).map(|e|e.id.as_str()).collect::<Vec<_>>(),"capacity_unassessed_ids":selected.iter().filter(|e|!capacity(e)).map(|e|e.id.as_str()).collect::<Vec<_>>(),"other_sla_terms":"latency, availability, affordability, load and topology unassessed","native_null_result":native.null_result,"rubric":"v0; weights retained by core for provenance, not applied by find_gaps"});
    report
}
fn response(input: &str) -> Value {
    if input.len() > MAX_REQUEST {
        return failure("request", "RequestLimit", "Envelope exceeds 1 MiB UTF-8");
    }
    let request: Request = match serde_json::from_str(input) {
        Ok(req) => req,
        Err(e) => return failure("request", "InvalidRequest", &e.to_string()),
    };
    let scale = match Scale::parse(&request.scale) {
        Some(s) => s,
        None => {
            return failure(
                "scale",
                "InvalidScale",
                "Choose international, national, regional or local",
            )
        }
    };
    if request.corpus_json.len() > 512 * 1024 {
        return failure("corpus", "CorpusLimit", "Corpus JSON exceeds 512 KiB UTF-8");
    }
    let corpus: Corpus = match serde_json::from_str(&request.corpus_json) {
        Ok(c) => c,
        Err(e) => return failure("corpus", "InvalidCorpus", &e.to_string()),
    };
    if corpus.documents.is_empty() || corpus.documents.len() > 32 {
        return failure("corpus", "DocumentLimit", "Supply 1–32 documents");
    }
    let (mut names, mut total) = (BTreeSet::new(), 0usize);
    for doc in &corpus.documents {
        total += doc.markdown.len();
        if doc.name.trim().is_empty() || doc.name.len() > 128 || !names.insert(&doc.name) {
            return failure(
                "corpus",
                "DocumentName",
                "Unique non-blank names up to 128 UTF-8 bytes required",
            );
        }
        if doc.markdown.len() > 32 * 1024
            || doc.markdown.lines().count() > 4096
            || total > 256 * 1024
        {
            return failure(
                "corpus",
                "MarkdownLimit",
                "Each Markdown: 32 KiB/4,096 lines; combined Markdown: 256 KiB",
            );
        }
    }
    let mut records: Vec<_> = corpus.documents.into_iter().map(record).collect();
    let mut ids = BTreeSet::new();
    for row in &mut records {
        if let Some(entry) = &row.entry {
            if !ids.insert(entry.id.clone()) {
                row.status = "held";
                row.reasons
                    .push("Duplicate stable ID prevents analysis".into());
            }
        }
    }
    analysis(records, scale)
}
#[cfg_attr(feature = "wasm", wasm_bindgen::prelude::wasm_bindgen)]
pub fn inspect(input: &str) -> String {
    response(input).to_string()
}
pub fn examples() -> Value {
    let mut map = serde_json::Map::new();
    for name in [
        "tail",
        "mean",
        "null",
        "missing-scores",
        "held",
        "scale-isolation",
        "capacity-screen",
    ] {
        let mut documents = Vec::new();
        for index in 0..4 {
            let rating = match name {
                "tail" if index == 0 => 1,
                "mean" if index < 3 => 1,
                _ => 9,
            };
            let scores = Dimension::all()
                .iter()
                .filter(|d| name != "missing-scores" || d.code() == "DIM-01")
                .map(|d| {
                    format!(
                        "score.{}: {}\n",
                        d.code(),
                        if d.code() == "DIM-01" { rating } else { 8 }
                    )
                })
                .collect::<String>();
            let source = if name == "held" && index == 0 {
                ""
            } else {
                "browser-authored-fixture"
            };
            let mbps = if name == "capacity-screen" { 1 } else { 20 };
            documents.push(json!({"name":format!("synthetic-{index}.md"),"markdown":format!("---\nid: synthetic-{index}\ntype: synthetic-market\nscale: regional\nmarket: Authored market {index}\ntier: T4\n{scores}quantity: {mbps} | Mbps | provisional | {source}\n---\n\nAuthored fixture only; no empirical market claim.\n")}));
        }
        if name == "scale-isolation" {
            documents.push(json!({"name":"local.md","markdown":"---\nid: synthetic-local\ntype: synthetic-market\nscale: local\nmarket: Authored local\ntier: T4\nscore.DIM-01: 0\n---\n"}));
        }
        map.insert(
            name.into(),
            json!({"scale":"regional","corpus_json":format!("{:#}",json!({"documents":documents}))}),
        );
    }
    Value::Object(map)
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn native_mean_tail_null() {
        let tail = response(&examples()["tail"].to_string());
        assert_eq!(tail["summary"]["mean_regions"], json!([]));
        assert_eq!(
            tail["summary"]["tail_regions"].as_array().map(Vec::len),
            Some(1)
        );
        assert_eq!(
            response(&examples()["mean"].to_string())["summary"]["mean_regions"]
                .as_array()
                .map(Vec::len),
            Some(1)
        );
        let null = response(&examples()["null"].to_string());
        assert_eq!(null["summary"]["native_null_result"], true);
        assert_eq!(null["authority"]["public_finding_release_approved"], false);
    }
    #[test]
    fn missing_is_not_complete() {
        let r = response(&examples()["missing-scores"].to_string());
        assert_eq!(r["summary"]["complete_dimensions"], json!(["DIM-01"]));
        assert_eq!(r["summary"]["mean_regions"], json!([]));
        let native: packet_gap::GapAnalysis =
            serde_json::from_str(r["analysis_json"].as_str().unwrap()).unwrap();
        assert_eq!(native.regions.len(), 12);
    }
    #[test]
    fn holds_and_scale() {
        let r = response(&examples()["held"].to_string());
        assert_eq!(r["outcome"], "admission-held");
        assert!(r["analysis_json"].is_null());
        let r = response(&examples()["scale-isolation"].to_string());
        assert_eq!(r["selection"]["excluded_ids"], json!(["synthetic-local"]));
    }
    #[test]
    fn capacity_is_partial() {
        let r = response(&examples()["capacity-screen"].to_string());
        assert_eq!(
            r["summary"]["capacity_gaps"].as_array().map(Vec::len),
            Some(4)
        );
        assert!(r["summary"]["other_sla_terms"]
            .as_str()
            .unwrap()
            .contains("unassessed"));
    }
    #[test]
    fn limits() {
        assert_eq!(response("{}")["ok"], false);
        assert_eq!(
            response(&" ".repeat(MAX_REQUEST + 1))["diagnostic"]["code"],
            "RequestLimit"
        );
        let mut r = examples()["tail"].clone();
        r["scale"] = json!("everywhere");
        assert_eq!(response(&r.to_string())["ok"], false);
    }
    #[test]
    fn malformed_and_unknown_frontmatter_are_visible() {
        let base="---\nid: x\ntype: synthetic\nscale: regional\nmarket: Authored\ntier: T4\nquantity: 20 | Mbps | provisional | browser-authored-fixture\n---\n";
        for text in [
            base.replace("id: x", "id: x\nid: y"),
            base.replace("quantity: 20", "quantity: broken"),
            base.replace("tier: T4", "tier: T4\nscore.DIM-01: broken"),
        ] {
            let row = record(Document {
                name: "case.md".into(),
                markdown: text,
            });
            assert_eq!(row.status, "held");
            assert!(!row.reasons.is_empty());
        }
        let row = record(Document {
            name: "case.md".into(),
            markdown: base
                .replace("tier: T4", "tier: T4\nextra: <img src=x>")
                .replace("| provisional |", "| unknown-label |"),
        });
        assert_eq!(row.status, "admitted");
        assert_eq!(row.warnings.len(), 2);
        assert_eq!(
            row.entry.unwrap().quantities[0].label,
            packet_corpus::EvidenceLabel::Estimated
        );
    }
    #[test]
    fn collection_byte_line_and_capacity_bounds() {
        let req = examples()["tail"].clone();
        let corpus: Value = serde_json::from_str(req["corpus_json"].as_str().unwrap()).unwrap();
        for (markdown, code) in [
            (" ".repeat(32769), "MarkdownLimit"),
            ("\n".repeat(4097), "MarkdownLimit"),
        ] {
            let mut body = corpus.clone();
            body["documents"][0]["markdown"] = json!(markdown);
            let mut r = req.clone();
            r["corpus_json"] = json!(body.to_string());
            assert_eq!(response(&r.to_string())["diagnostic"]["code"], code);
        }
        let mut body = corpus.clone();
        body["documents"]=json!((0..32).map(|i|json!({"name":format!("{i}.md"),"markdown":corpus["documents"][0]["markdown"].as_str().unwrap().replace("synthetic-0",&format!("unique-{i}"))})).collect::<Vec<_>>());
        let mut r = req.clone();
        r["corpus_json"] = json!(body.to_string());
        assert_eq!(response(&r.to_string())["held_or_rejected"], 0);
        body["documents"]
            .as_array_mut()
            .unwrap()
            .push(json!({"name":"extra.md","markdown":""}));
        r["corpus_json"] = json!(body.to_string());
        assert_eq!(
            response(&r.to_string())["diagnostic"]["code"],
            "DocumentLimit"
        );
        let row = record(Document {
            name: "units.md".into(),
            markdown: corpus["documents"][0]["markdown"]
                .as_str()
                .unwrap()
                .replace("| Mbps |", "| not-Mbps |"),
        });
        assert!(!capacity(&row.entry.unwrap()));
    }
    #[test]
    fn deterministic_labels() {
        for (_, req) in examples().as_object().unwrap() {
            assert_eq!(inspect(&req.to_string()), inspect(&req.to_string()));
        }
        let r = response(&examples()["tail"].to_string());
        assert_eq!(
            r["records"][0]["entry"]["quantities"][0]["label"],
            "Provisional"
        );
        assert_eq!(
            r["records"][0]["entry"]["quantities"][0]["source_id"],
            "browser-authored-fixture"
        );
    }
}
