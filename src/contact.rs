/* Workflow for Contact:
 *  1. A contact can be created with a given name. Besides the name, all other
 *     fields are optional.
 *  2. A contact can have an address added to it, which can also be removed.
 *  3. A contact can have an email added to it, which can also be removed.
 *  4. A contact can have an phone number added to it, which can also be removed.
 *  5. A contact can have any number of social profiles, which are either
 *     from a known or unknown source. These contacts can be added or removed.
 */
use std::{collections::HashMap, error::Error, fmt::Display};

#[derive(Debug)]
pub struct ContactError;

impl Error for ContactError {}
impl Display for ContactError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, PartialEq, Clone, Default)]
struct ContactName(String);

impl ContactName {
    fn new(name: &str) -> Result<Self, ContactError> {
        Ok(Self(name.to_owned()))
    }

    fn get_name(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, PartialEq, Clone)]
struct ContactAddress(String);

impl ContactAddress {
    fn new(address: &str) -> Result<Self, ContactError> {
        Ok(Self(address.to_owned()))
    }

    fn get_address(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, PartialEq, Clone)]
struct ContactEmail(String);

impl ContactEmail {
    fn new(email: &str) -> Result<Self, ContactError> {
        Ok(Self(email.to_owned()))
    }

    fn get_email(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, PartialEq, Clone)]
struct ContactPhoneNumber(String);

impl ContactPhoneNumber {
    fn new(phone_number: &str) -> Result<Self, ContactError> {
        Ok(Self(phone_number.to_owned()))
    }

    fn get_phone_number(&self) -> &str {
        &self.0
    }
}

#[derive(PartialEq, Eq, Hash, Debug, Clone, Default)]
struct SocialProfileLink(String);

impl SocialProfileLink {
    fn new(link: &str) -> Result<Self, ContactError> {
        Ok(Self(link.to_owned()))
    }

    fn get_link(&self) -> &str {
        &self.0
    }
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum SocialMediaWebsite {
    Github,
    Twitter,
    MySpace,
    LinkedIn,
    Unknown,
}

#[derive(PartialEq, Debug, Clone, Default)]
struct SocialProfileList(HashMap<SocialProfileLink, SocialMediaWebsite>);

impl SocialProfileList {
    fn as_vec(&self) -> Vec<(SocialMediaWebsite, &str)> {
        self.0
            .iter()
            .map(|(link, social_media_site)| (*social_media_site, link.get_link()))
            .collect()
    }

    fn add_social_profile(
        mut self,
        social_media_site: SocialMediaWebsite,
        link: &str,
    ) -> Result<Self, ContactError> {
        self.0
            .insert(SocialProfileLink::new(link)?, social_media_site);

        Ok(self)
    }

    fn remove(mut self, link: &str) -> Result<Self, ContactError> {
        self.0.remove(&SocialProfileLink::new(link)?);

        Ok(self)
    }
}

#[derive(PartialEq, Debug, Clone, Default)]
pub struct Contact {
    name: ContactName,
    address: Option<ContactAddress>,
    email: Option<ContactEmail>,
    phone_number: Option<ContactPhoneNumber>,
    social_profiles: SocialProfileList,
}

impl Contact {
    pub fn new(name: &str) -> Result<Self, ContactError> {
        let name = ContactName::new(name)?;

        Ok(Self {
            name,
            ..Default::default()
        })
    }

    pub fn get_name(&self) -> &str {
        self.name.get_name()
    }

    // Address Functions

    pub fn add_address(mut self, address: &str) -> Result<Self, ContactError> {
        self.address = Some(ContactAddress::new(address)?);

        Ok(self)
    }

    pub fn get_address(&self) -> Option<&str> {
        match &self.address {
            Some(address) => Some(address.get_address()),
            None => None,
        }
    }

    pub fn remove_address(mut self) -> Self {
        self.address = None;

        self
    }

    // Email Functions

    pub fn add_email(mut self, email: &str) -> Result<Self, ContactError> {
        self.email = Some(ContactEmail::new(email)?);

        Ok(self)
    }

    pub fn get_email(&self) -> Option<&str> {
        match &self.email {
            Some(email) => Some(email.get_email()),
            None => None,
        }
    }

    pub fn remove_email(mut self) -> Self {
        self.email = None;

        self
    }

    // Phone Number Functions

    pub fn add_phone_number(mut self, phone_number: &str) -> Result<Self, ContactError> {
        self.phone_number = Some(ContactPhoneNumber::new(phone_number)?);

        Ok(self)
    }

    pub fn get_phone_number(&self) -> Option<&str> {
        match &self.phone_number {
            Some(phone_number) => Some(phone_number.get_phone_number()),
            None => None,
        }
    }

    pub fn remove_phone_number(mut self) -> Self {
        self.phone_number = None;

        self
    }

    // Social Media Profile Functions

    pub fn add_social_profile(
        mut self,
        social_media_site: SocialMediaWebsite,
        link: &str,
    ) -> Result<Self, ContactError> {
        let social_profiles = self
            .social_profiles
            .add_social_profile(social_media_site, link)?;
        self.social_profiles = social_profiles;

        Ok(self)
    }

    pub fn get_social_media_profiles(&self) -> Vec<(SocialMediaWebsite, &str)> {
        self.social_profiles.as_vec()
    }

    pub fn remove_social_profile(mut self, link: &str) -> Result<Self, ContactError> {
        let social_profiles = self.social_profiles.remove(link);
        self.social_profiles = social_profiles?;

        Ok(self)
    }
}
