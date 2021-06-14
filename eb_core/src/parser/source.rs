use std::{fs::read_to_string, io};

pub enum Source {
    File(SourceFile),
    String(String),
}

pub struct SourceFile {
    name: String,
    body: String,
}

impl Source {
    pub fn as_file(&self) -> Option<&SourceFile> {
        match self {
            Self::File(file) => Some(file),
            Self::String(_) => None,
        }
    }

    pub fn as_str(&self) -> Option<&String> {
        match self {
            Self::File(_) => None,
            Self::String(s) => Some(s),
        }
    }

    pub fn body(&self) -> &String {
        match self {
            Self::File(file) => file.body(),
            Self::String(s) => &s,
        }
    }
}

impl SourceFile {
    pub fn new(name: String) -> Result<Self, io::Error> {
        Ok(Self {
            body: read_to_string(name.as_str())?,
            name,
        })
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    pub fn body(&self) -> &String {
        &self.body
    }
}
