//! Internal logging support.
use std::path::Path;
use std::{fs::File, io::stderr};

use anyhow::Result;
use fern::Dispatch;
use log::LevelFilter;

pub fn initialize<P: AsRef<Path>>(
    file: Option<P>,
    term_filter: Option<LevelFilter>,
    file_filter: LevelFilter,
) -> Result<()> {
    let mut log = Dispatch::new().level(file_filter).format(|out, msg, rec| {
        out.finish(format_args!("[{}] {}", rec.level(), msg));
    });
    if let Some(path) = file {
        let f = File::options()
            .write(true)
            .create(true)
            .truncate(true)
            .open(path)?;
        log = log.chain(f);
    }

    if let Some(level) = term_filter {
        log = log.chain(Dispatch::new().level(level).chain(stderr()));
    }
    log.apply()?;

    Ok(())
}
