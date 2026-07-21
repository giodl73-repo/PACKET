use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "packet")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Corpus {
        path: PathBuf,
    },
    Score {
        path: PathBuf,
    },
    #[command(name = "tier-sla")]
    TierSla {
        path: PathBuf,
    },
    Gap {
        #[arg(long)]
        scale: String,
        /// Directory of corpus `*.md` entries to analyze (defaults to none).
        #[arg(long)]
        corpus: Option<PathBuf>,
    },
}

fn load_corpus(
    dir: &std::path::Path,
) -> Result<Vec<packet_corpus::CorpusEntry>, Box<dyn std::error::Error>> {
    let mut entries = Vec::new();
    for item in std::fs::read_dir(dir)? {
        let path = item?.path();
        if path.extension().and_then(|ext| ext.to_str()) == Some("md") {
            let input = std::fs::read_to_string(&path)?;
            // Only files that open with a `---` frontmatter block are corpus
            // entries; skip docs like SCHEMA.md that share the directory.
            if !input.trim_start().starts_with("---") {
                continue;
            }
            entries.push(packet_corpus::CorpusEntry::from_markdown(&input)?);
        }
    }
    entries.sort_by(|a, b| a.id.cmp(&b.id));
    Ok(entries)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Corpus { path } => {
            let input = std::fs::read_to_string(&path)?;
            let entry = packet_corpus::CorpusEntry::from_markdown(&input)?;
            let result = match entry.validate() {
                Ok(_) => "holds",
                Err(_) => "reject",
            };
            println!("{} {}", entry.id, result);
        }
        Commands::Score { path } => {
            let input = std::fs::read_to_string(&path)?;
            let entry = packet_corpus::CorpusEntry::from_markdown(&input)?;
            let scorer = packet_score::ProvisionalScorer::default();
            for dimension in packet_score::Dimension::all() {
                let score = packet_score::DimensionScorer::score(&scorer, &entry, dimension);
                println!("{} {}", dimension.code(), score.value());
            }
        }
        Commands::TierSla { path } => {
            let input = std::fs::read_to_string(&path)?;
            let entry = packet_corpus::CorpusEntry::from_markdown(&input)?;
            let tier = packet_tier::classify(&entry);
            let has_gap = packet_tier::tier_sla_gap(&entry).is_some();
            println!("{:?} {}", tier, has_gap);
        }
        Commands::Gap { scale, corpus } => {
            let scale = packet_corpus::Scale::parse(&scale)
                .ok_or_else(|| format!("invalid scale: {scale}"))?;
            let entries = match &corpus {
                Some(dir) => load_corpus(dir)?,
                None => Vec::new(),
            };
            // Only dimensions that at least one entry actually scores are
            // "assessed"; absent scores default to 0.0 and would otherwise flag
            // spurious gaps where there is simply no data.
            let assessed: std::collections::BTreeSet<String> = entries
                .iter()
                .flat_map(|entry| entry.scores.keys().cloned())
                .collect();
            let rubric = packet_score::Rubric::v0();
            let tier_gaps: Vec<packet_tier::TierSlaGap> = Vec::new();
            let analysis = packet_gap::find_gaps(&entries, &rubric, scale, &tier_gaps, false);
            let regions: Vec<_> = analysis
                .regions
                .iter()
                .filter(|region| assessed.contains(region.dimension.code()))
                .collect();
            println!("scale: {scale} | entries: {}", entries.len());
            println!(
                "assessed dimensions: {}",
                assessed.iter().cloned().collect::<Vec<_>>().join(", ")
            );
            println!("systemic gap regions (assessed): {}", regions.len());
            for region in &regions {
                println!(
                    "region {} | mean_score={:.2} | members: {}",
                    region.dimension.code(),
                    region.mean_score,
                    region.member_ids.join(", ")
                );
            }
            let tail_regions: Vec<_> = analysis
                .tail_regions
                .iter()
                .filter(|region| assessed.contains(region.dimension.code()))
                .collect();
            println!("tail gap regions (assessed): {}", tail_regions.len());
            for region in &tail_regions {
                println!(
                    "{}-gap {} | tail_mean={:.2} | {:.0}% below bar | under-served: {}",
                    if region.systemic { "systemic" } else { "tail" },
                    region.dimension.code(),
                    region.tail_mean,
                    region.share_below_threshold * 100.0,
                    region.tail_member_ids.join(", ")
                );
            }
            // Per-entry deviation against the corpus mean for each assessed
            // dimension: this surfaces intra-corpus disparities that the
            // dimension-mean gap test (mean < 5.0) does not flag on its own.
            let scorer = packet_score::ProvisionalScorer::default();
            for dimension in packet_score::Dimension::all() {
                if !assessed.contains(dimension.code()) {
                    continue;
                }
                let mut scored: Vec<(String, f64)> = entries
                    .iter()
                    .map(|entry| {
                        (
                            entry.id.clone(),
                            packet_score::DimensionScorer::score(&scorer, entry, dimension).value(),
                        )
                    })
                    .collect();
                let mean = scored.iter().map(|(_, v)| v).sum::<f64>() / scored.len().max(1) as f64;
                scored.sort_by(|a, b| a.1.total_cmp(&b.1));
                println!("dimension {} | corpus_mean={:.2}", dimension.code(), mean);
                for (id, value) in &scored {
                    println!("  {id} | score={value:.2} | deviation={:+.2}", value - mean);
                }
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::CommandFactory;

    #[test]
    fn verify_cli() {
        Cli::command().debug_assert();
    }

    #[test]
    fn parses_gap() {
        let result = Cli::try_parse_from(["packet", "gap", "--scale", "national"]);
        assert!(result.is_ok());
    }

    #[test]
    fn parses_corpus() {
        let result = Cli::try_parse_from(["packet", "corpus", "some.md"]);
        assert!(result.is_ok());
    }
}
