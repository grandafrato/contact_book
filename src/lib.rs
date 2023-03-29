pub mod book;

#[derive(Debug)]
pub struct ContactError;

#[derive(PartialEq, Debug, Clone)]
pub struct Contact;

impl Contact {
    pub fn new(_name: &str) -> Self {
        Self
    }
}
