use contact_book::book::{ContactBook, ContactBookError};
use contact_book::contact::{Contact, ContactError, SocialMediaWebsite};
use std::error::Error;

#[test]
fn contact_book() -> Result<(), Box<dyn Error>> {
    let contact = Contact::new("Foo Bar")?;
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
    let contact = Contact::new("Foo Bar")?;

    assert_eq!(contact.get_name(), "Foo Bar");

    // Adding & removing an address.

    assert_eq!(contact.get_address(), None);

    let contact = contact.add_address("123 Main St.")?;

    assert_eq!(contact.get_address(), Some("123 Main St."));

    let contact = contact.remove_address();

    assert_eq!(contact.get_address(), None);

    // Adding & removing an email.

    assert_eq!(contact.get_email(), None);

    let contact = contact.add_email("foo@example.test")?;

    assert_eq!(contact.get_email(), Some("foo@example.test"));

    let contact = contact.remove_email();

    assert_eq!(contact.get_email(), None);

    // Adding & removing an phone number.

    assert_eq!(contact.get_phone_number(), None);

    let contact = contact.add_phone_number("1234567890")?;

    assert_eq!(contact.get_phone_number(), Some("1234567890"));

    let contact = contact.remove_phone_number();

    assert_eq!(contact.get_phone_number(), None);

    // Adding & removing a social media profile.

    assert_eq!(contact.get_social_media_profiles(), Vec::new());

    let contact =
        contact.add_social_profile(SocialMediaWebsite::Github, "https://github.com/example")?;

    assert_eq!(
        contact.get_social_media_profiles(),
        vec![(SocialMediaWebsite::Github, "https://github.com/example")]
    );

    let contact = contact.remove_social_profile("https://github.com/example")?;

    assert_eq!(contact.get_social_media_profiles(), Vec::new());

    Ok(())
}
