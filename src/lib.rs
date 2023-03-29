/* Workflow for contact_book:
 *   1. There is a single global contacts book.
 *   2. A contact can be added to the contact book, creating a unique identifier
 *      for the contact in the contact book.
 *   3. The contact book can be queried for a contact based on it's unique
 *      identifier within the contact book.
 *   4. A contact can be labeled as a favorite contact within the contact book.
 *   5. A contact that was favorited can be unfavorited.
 *   6. A contact can be removed from the contact book by its unique id.
 */
use std::collections::HashMap;

#[derive(PartialEq, Debug, Clone)]
pub struct Contact;

impl Contact {
    pub fn new(_name: &str) -> Self {
        Contact
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct ContactBookEntryId;

impl ContactBookEntryId {
    fn new() -> Self {
        Self
    }
}

#[derive(Debug, PartialEq)]
enum FavoritedContactEntry {
    Favorited,
    NotFavorited,
}

#[derive(PartialEq, Debug)]
pub struct ContactBookEntry {
    contact: Contact,
    favorited: FavoritedContactEntry,
}

impl ContactBookEntry {
    pub fn new(contact: Contact) -> Self {
        Self {
            contact,
            favorited: FavoritedContactEntry::NotFavorited,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum ContactBookError {
    NoSuchContactInBook,
}

#[derive(Debug, PartialEq)]
pub struct ContactBook(HashMap<ContactBookEntryId, ContactBookEntry>);

impl ContactBook {
    pub fn new() -> Self {
        ContactBook(HashMap::new())
    }

    pub fn add_contact(
        mut self,
        contact: Contact,
    ) -> Result<(ContactBookEntryId, Self), ContactBookError> {
        let new_contact_entry = ContactBookEntry::new(contact);
        let id = ContactBookEntryId::new();
        self.0.insert(id, new_contact_entry);

        Ok((id, self))
    }

    pub fn list_contacts(&self) -> Vec<(ContactBookEntryId, &Contact)> {
        self.0
            .iter()
            .map(|(id, ContactBookEntry { contact, .. })| (*id, contact))
            .collect()
    }

    pub fn get_contact(
        &self,
        contact_id: &ContactBookEntryId,
    ) -> Result<&Contact, ContactBookError> {
        match self.0.get(contact_id) {
            Some(contact_entry) => Ok(&contact_entry.contact),
            None => Err(ContactBookError::NoSuchContactInBook),
        }
    }

    pub fn add_favorite_contact(
        mut self,
        contact_id: &ContactBookEntryId,
    ) -> Result<Self, ContactBookError> {
        match self.0.get_mut(contact_id) {
            Some(contact_entry) => {
                contact_entry.favorited = FavoritedContactEntry::Favorited;
                return Ok(self);
            }
            None => Err(ContactBookError::NoSuchContactInBook),
        }
    }

    pub fn remove_favorite_contact(
        mut self,
        contact_id: &ContactBookEntryId,
    ) -> Result<Self, ContactBookError> {
        match self.0.get_mut(contact_id) {
            Some(contact_entry) => {
                contact_entry.favorited = FavoritedContactEntry::NotFavorited;
                return Ok(self);
            }
            None => Err(ContactBookError::NoSuchContactInBook),
        }
    }

    pub fn get_favorite_contact_ids(&self) -> Vec<ContactBookEntryId> {
        self.0
            .iter()
            .filter_map(|(id, ContactBookEntry { favorited, .. })| match favorited {
                FavoritedContactEntry::Favorited => Some(*id),
                FavoritedContactEntry::NotFavorited => None,
            })
            .collect()
    }

    pub fn remove_contact(
        mut self,
        contact_id: &ContactBookEntryId,
    ) -> Result<Self, ContactBookError> {
        match self.0.remove(contact_id) {
            Some(_) => Ok(self),
            None => Err(ContactBookError::NoSuchContactInBook),
        }
    }
}
