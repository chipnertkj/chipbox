use email_address::EmailAddress;
use serde::{Deserialize, Serialize};
use url::Url;

/// Contact information for a person.
#[derive(PartialEq, Serialize, Deserialize, Debug, Clone)]
pub struct Contact {
    /// The username of the person.
    pub username: String,
    /// The full name of the person.
    pub full_name: Option<String>,
    /// The email address used for contacting the person.
    pub email: Option<EmailAddress>,
    /// The URL of a website associated with the person.
    pub url: Option<Url>,
}

/// Information about an author and their role on a project.
#[derive(PartialEq, Serialize, Deserialize, Debug, Clone)]
pub struct Author {
    /// Contact information for the author.
    pub contact: Contact,
    /// The role of the author in creating the project.
    pub role: Option<String>,
}
