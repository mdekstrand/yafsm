use anyhow::Result;
use clap::Parser;

mod dump;
mod monitors;

use dump::DumpOpts;
use monitors::SystemState;

/// System process monitor.
#[derive(Parser, Debug)]
#[command(name = "hypertop")]
struct CLIOptions {
    #[command(flatten)]
    dump: DumpOpts,
}

fn main() -> Result<()> {
    let cli = CLIOptions::parse();

    let mut state = SystemState::init()?;

    if cli.dump.dump(&mut state)? {
        return Ok(());
    }

    Ok(())
}
