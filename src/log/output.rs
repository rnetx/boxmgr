use std::{fs, io};


pub enum LogOutput {
    File(fs::File),
    Stdout(io::Stdout),
    Stderr(io::Stderr),
    Nop,
}

impl LogOutput {
    pub fn file(path: &str) -> Result<Self, io::Error> {
        let file = fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(path)?;
        Ok(Self::File(file))
    }

    pub fn stdout() -> Self {
        Self::Stdout(io::stdout())
    }

    pub fn stderr() -> Self {
        Self::Stderr(io::stderr())
    }

    pub fn nop() -> Self {
        Self::Nop
    }

    pub(crate) fn to_box_writer(self) -> Box<dyn io::Write> {
        match self {
            Self::File(f) => Box::new(f),
            Self::Stdout(s) => Box::new(s),
            Self::Stderr(s) => Box::new(s),
            Self::Nop => Box::new(io::sink()),
        }
    }
}
