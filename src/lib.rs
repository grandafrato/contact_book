use std::{error::Error, fmt::Display};

pub mod book;

#[derive(Debug)]
pub struct ContactError;

impl Error for ContactError {}
impl Display for ContactError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct Contact;

impl Contact {
    pub fn new(_name: &str) -> Result<Self, ContactError> {
        Ok(Self)
    }
}
