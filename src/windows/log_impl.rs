use simplelog::{Config, LevelFilter, WriteLogger};
use std::fs::File;

pub fn init(filter_level: log::LevelFilter) {
    // Try to ensure the log file is created in the game directory
    if let Ok(file) = File::create("hachimi.log") {
        let _ = WriteLogger::init(filter_level, Config::default(), file);
    } else {
        // Fallback or just ignore if file creation fails
        // If we want to keep windebug_logger, we'd need a combinator,
        // but for now File logging is the requested "improvement".
        // Note: windebug_logger::init_with_level(level) would conflict if both try to set the global logger.
        #[cfg(debug_assertions)]
        if let Some(level) = filter_level.to_level() {
            windebug_logger::init_with_level(level).ok();
        }
    }
}
