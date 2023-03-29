use contact_book::book::{ContactBook, ContactBookError};
use contact_book::{Contact, ContactError};
use std::error::Error;

#[test]
fn contact_book() -> Result<(), Box<dyn Error>> {
    let contact = Contact::new("Foo Bar");
    let contact_book = ContactBook::new();

    let (contact_id, contact_book) = contact_book.add_contact(contact.clone());

    assert_eq!(contact_book.list_contacts(), vec![(&contact_id, &contact)]);

    let queried_contact = contact_book.get_contact(&contact_id)?;

    assert_eq!(*queried_contact, contact);

    let contact_book = contact_book.add_favorite_contact(&contact_id)?;
    let favorite_contacts = contact_book.get_favorite_contact_ids();

    assert!(favorite_contacts.contains(&contact_id));

    let contact_book = contact_book.remove_favorite_contact(&contact_id)?;
    let favorite_contacts = contact_book.get_favorite_contact_ids();

    assert_eq!(favorite_contacts, Vec::new());

    let contact_book = contact_book.remove_contact(contact_id.clone())?;
    let queried_contact = contact_book.get_contact(&contact_id);

    assert_eq!(queried_contact, Err(ContactBookError::NoSuchContactInBook));

    Ok(())
}

#[test]
fn contact() -> Result<(), ContactError> {
    todo!();
    Ok(())
}
