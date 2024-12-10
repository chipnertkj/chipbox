//! Song metadata for this version of the project format.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

mod author;

pub use author::{Author, Contact};

/// Additional information about a song, including
/// the name, description, author, etc.
#[derive(PartialEq, Serialize, Deserialize, Debug, Clone)]
pub struct SongMeta {
    /// The name of the song.
    pub name: String,
    /// A description of the song that may contain details such as artist notes
    /// or cultural and historical context.
    pub description: Option<String>,
    /// The authors contributing to the song.
    pub authors: Vec<Author>,
    /// The date and time the song was created.
    pub datetime_created: Option<DateTime<Utc>>,
}

impl SongMeta {
    /// Creates a new `SongMeta` with a valid creation date.
    ///
    /// The date will be set to the current date and time as
    /// of calling this function.
    pub fn new_now(
        name: impl Into<String>,
        description: Option<impl Into<String>>,
        authors: impl Into<Vec<Author>>,
    ) -> Self {
        Self {
            name: name.into(),
            description: description.map(|d| d.into()),
            authors: authors.into(),
            datetime_created: Some(Utc::now()),
        }
    }
}
