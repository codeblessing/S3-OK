use once_cell::sync::OnceCell;
use std::sync::RwLock;

#[derive(Default)]
pub struct Settings {
    pub prompt: bool,
    pub unbuffered: bool,
    pub verbosity: u8,
    pub input_files: Vec<String>,
    pub log_file: String,
    pub kill_time: u16,
}

static SETTINGS: OnceCell<RwLock<Settings>> = OnceCell::new();

impl Settings {
    pub fn init(prompt: bool, unbuffered: bool, verbosity: u8, files: Vec<String>, log_file: String, kill_time: u16) {
        match SETTINGS.set(RwLock::new(Settings {
            prompt,
            unbuffered,
            verbosity,
            input_files: files,
            log_file,
            kill_time,
        })) {
            _ => return,
        }
    }

    pub fn get() -> Result<&'static RwLock<Settings>, ()> {
        if SETTINGS.get().is_none() {
            return Err(());
        }

        Ok(SETTINGS.get().unwrap())
    }
}
