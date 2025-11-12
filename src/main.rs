use clap::{Parser, Subcommand};
use anyhow::Result;
use std::path::PathBuf;

mod models;
mod parser;
mod analyzer;
mod report;

use parser::ProfileParser;
use analyzer::BuildAnalyzer;
use report::ReportGenerator;

#[derive(Parser)]
#[command(name = "gradle-profiler")]
#[command(about = "Analyze Gradle build profiles", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Analyze the latest build profile
    Analyze {
        /// Path to Android project (default: current directory)
        #[arg(short, long, default_value = ".")]
        project: PathBuf,
        
        /// Path to specific profile file
        #[arg(short, long)]
        file: Option<String>,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Analyze { project, file } => {
            let profile_path = match file {
                Some(path) => path,
                None => ProfileParser::find_latest_profile(&project)?,
            };

            println!("📂 Reading profile: {}", profile_path);

            // HTML만 파싱 (JSON 제거)
            let profile = ProfileParser::parse_html(&profile_path)?;

            let analysis = BuildAnalyzer::analyze(&profile);
            ReportGenerator::print_analysis(&analysis);
        }
    }

    Ok(())
}