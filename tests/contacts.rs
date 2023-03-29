use contact_book::{Contact, ContactBook, ContactBookError};

#[test]
fn contact_book() -> Result<(), ContactBookError> {
    let contact = Contact::new("Foo Bar");
    let contact_book = ContactBook::new();

    let (contact_id, contact_book) = contact_book.add_contact(contact.clone());

    assert_eq!(contact_book.list_contacts(), vec![(contact_id, &contact)]);

    let queried_contact = contact_book.get_contact(&contact_id)?;

    assert_eq!(*queried_contact, contact);

    let contact_book = contact_book.add_favorite_contact(&contact_id)?;
    let favorite_contacts = contact_book.get_favorite_contact_ids();

    assert!(favorite_contacts.contains(&contact_id));

    let contact_book = contact_book.remove_favorite_contact(&contact_id)?;
    let favorite_contacts = contact_book.get_favorite_contact_ids();

    assert_eq!(favorite_contacts, Vec::new());

    let contact_book = contact_book.remove_contact(&contact_id)?;
    let queried_contact = contact_book.get_contact(&contact_id);

    assert_eq!(queried_contact, Err(ContactBookError::NoSuchContactInBook));

    Ok(())
}
