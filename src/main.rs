use clap::Parser;
use std::io::{BufRead, BufReader, Result};
use std::path::PathBuf;
use indicatif::{ProgressBar, ProgressStyle};

#[derive(Parser)]
struct Cli {
    pattern: String,
    path: PathBuf,
}

fn main() -> Result<()> {
    let args = Cli::parse();

    // Create a spinner progress bar
    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::default_spinner()
            .template("{spinner} {msg}")
            .unwrap(),
    );
    pb.enable_steady_tick(std::time::Duration::from_millis(120));
    pb.set_message(format!("🔍 Searching for '{}' in {:?}", args.pattern, args.path));

    // Open the file
    let file = std::fs::File::open(&args.path)?;
    let reader = BufReader::new(file);

    let pattern = args.pattern.to_lowercase();

    let found = reader
        .lines()
        .filter_map(Result::ok)
        .any(|line| {
            pb.set_message(format!("🔍 Searching... found {} lines so far", line.len()));
            line.to_lowercase().contains(&pattern)
        });

    pb.finish_and_clear();

    if found {
        println!("✅ String '{}' found in file {:?}", args.pattern, args.path);
    } else {
        println!("❌ String '{}' not found in file {:?}", args.pattern, args.path);
    }

    Ok(())
}
