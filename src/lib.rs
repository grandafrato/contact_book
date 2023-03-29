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
use std::collections::{HashMap, HashSet};
use uuid::Uuid;

#[derive(PartialEq, Debug, Clone)]
pub struct Contact;

impl Contact {
    pub fn new(_name: &str) -> Self {
        Self
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct ContactBookEntryId(Uuid);

impl ContactBookEntryId {
    fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

#[derive(Debug, PartialEq)]
pub enum ContactBookError {
    NoSuchContactInBook,
    CannotFavoriteNonexistantContact,
    ContactWasNotAFavorite,
    CannotRemoveNonexistantContact,
}

#[derive(Debug, PartialEq)]
pub struct ContactBook {
    contacts: HashMap<ContactBookEntryId, Contact>,
    favorites: HashSet<ContactBookEntryId>,
}

impl ContactBook {
    pub fn new() -> Self {
        ContactBook {
            contacts: HashMap::new(),
            favorites: HashSet::new(),
        }
    }

    pub fn add_contact(mut self, contact: Contact) -> (ContactBookEntryId, Self) {
        let id = ContactBookEntryId::new();
        self.contacts.insert(id.clone(), contact);

        (id, self)
    }

    pub fn list_contacts(&self) -> Vec<(&ContactBookEntryId, &Contact)> {
        self.contacts
            .iter()
            .map(|(id, contact)| (id, contact))
            .collect()
    }

    pub fn get_contact(
        &self,
        contact_id: &ContactBookEntryId,
    ) -> Result<&Contact, ContactBookError> {
        match self.contacts.get(contact_id) {
            Some(contact) => Ok(&contact),
            None => Err(ContactBookError::NoSuchContactInBook),
        }
    }

    pub fn add_favorite_contact(
        mut self,
        contact_id: &ContactBookEntryId,
    ) -> Result<Self, ContactBookError> {
        match self.contacts.get_mut(contact_id) {
            Some(_contact) => {
                self.favorites.insert(contact_id.clone());
                return Ok(self);
            }
            None => Err(ContactBookError::CannotFavoriteNonexistantContact),
        }
    }

    pub fn remove_favorite_contact(
        mut self,
        contact_id: &ContactBookEntryId,
    ) -> Result<Self, ContactBookError> {
        match self.contacts.get_mut(contact_id) {
            Some(_contact) => {
                if self.favorites.remove(contact_id) {
                    Ok(self)
                } else {
                    Err(ContactBookError::ContactWasNotAFavorite)
                }
            }
            None => Err(ContactBookError::NoSuchContactInBook),
        }
    }

    pub fn get_favorite_contact_ids(&self) -> Vec<ContactBookEntryId> {
        self.favorites.iter().cloned().collect()
    }

    pub fn remove_contact(
        mut self,
        contact_id: ContactBookEntryId,
    ) -> Result<Self, ContactBookError> {
        self.favorites.remove(&contact_id);
        match self.contacts.remove(&contact_id) {
            Some(_) => Ok(self),
            None => Err(ContactBookError::CannotRemoveNonexistantContact),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_two_new_contact_book_entry_ids_are_identical() {
        let id_1 = ContactBookEntryId::new();
        let id_2 = ContactBookEntryId::new();

        assert_ne!(id_1, id_2);
    }

    #[test]
    fn getting_a_nonexistant_contact_entry_returns_an_error() {
        let id = ContactBookEntryId::new();
        let contacts = ContactBook::new();

        let queried_contact = contacts.get_contact(&id);

        assert_eq!(queried_contact, Err(ContactBookError::NoSuchContactInBook));
    }

    #[test]
    fn adding_a_nonexistant_contact_to_favorites_returns_an_error() {
        let id = ContactBookEntryId::new();
        let contacts = ContactBook::new();

        let contacts = contacts.add_favorite_contact(&id);

        assert_eq!(
            contacts,
            Err(ContactBookError::CannotFavoriteNonexistantContact)
        );
    }

    #[test]
    fn removing_a_nonexistant_contact_from_favorites_returns_an_error() {
        let id = ContactBookEntryId::new();
        let contacts = ContactBook::new();

        let contacts = contacts.remove_favorite_contact(&id);

        assert_eq!(contacts, Err(ContactBookError::NoSuchContactInBook));
    }

    #[test]
    fn removing_a_nonfavorited_contact_from_favorites_returns_an_error() {
        let (id, contacts) = ContactBook::new().add_contact(Contact::new("Foo Bar"));

        let contacts = contacts.remove_favorite_contact(&id);

        assert_eq!(contacts, Err(ContactBookError::ContactWasNotAFavorite));
    }

    #[test]
    fn removing_a_nonexistant_contact_returns_an_error() {
        let id = ContactBookEntryId::new();
        let contacts = ContactBook::new();

        let contacts = contacts.remove_contact(id);

        assert_eq!(
            contacts,
            Err(ContactBookError::CannotRemoveNonexistantContact)
        );
    }

    #[test]
    fn removing_a_contact_from_the_book_removes_the_contact_id_from_the_list_of_favorites() {
        let (id, contacts) = ContactBook::new().add_contact(Contact::new("Foo Bar"));

        let contacts = contacts.add_favorite_contact(&id).unwrap();

        assert_eq!(contacts.get_favorite_contact_ids(), vec![id.clone()]);

        let contacts = contacts.remove_contact(id).unwrap();

        assert_eq!(contacts.get_favorite_contact_ids(), Vec::new())
    }
}
