use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
use std::sync::Mutex;
use std::time::{SystemTime, UNIX_EPOCH};

static LOGGER: Mutex<Option<RecordLogger>> = Mutex::new(None);

pub struct RecordLogger {
    file: std::fs::File,
}

impl RecordLogger {
    /// Initialise global recorder that appends to `path`.
    pub fn init(path: &Path) -> std::io::Result<()> {
        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(path)?;

        *LOGGER.lock().unwrap() = Some(RecordLogger { file });
        Ok(())
    }

    /// Returns true when the global logger is active (i.e. --log-file was supplied).
    pub fn is_enabled() -> bool {
        LOGGER.lock().unwrap().is_some()
    }
}

pub fn log_error(message: &str) {
    log_with_level("ERROR", message);
}

pub fn log_warn(message: &str) {
    log_with_level("WARN", message);
}

pub fn log_info(message: &str) {
    log_with_level("INFO", message);
}

fn log_with_level(level: &str, message: &str) {
    if let Ok(mut guard) = LOGGER.lock() {
        if let Some(logger) = guard.as_mut() {
            let timestamp = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs();
            let _ = writeln!(logger.file, "[{}] [RECORD] {}: {}", timestamp, level, message);
            let _ = logger.file.flush();
        }
    }
}
