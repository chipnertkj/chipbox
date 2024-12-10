//! Initial implementation of the project format.
//!
//! This module defines [`Project`], a structure
//! that contains information about a `chipbox` project.
//!
//! See the [`project`](crate::project) module documentation
//! for more information.

use super::meta::ProjectMeta;
use serde::{Deserialize, Serialize, Serializer};
use song::{meta::SongMeta, Song};

pub mod cmd;
pub mod song;

/// Holds all project data, including song metadata, note, automation, node graph data,
/// as well as metadata for the assets used within the project.
///
/// A [`Project`] is a container describing a [`Song`], the primary output of the program.
/// The wrapper is primarily used for versioning, but may contain things like workspace-specific configuration.
#[allow(clippy::manual_non_exhaustive)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Project {
    /// Zero-sized field for metadata serialization.
    /// See [`Self::emit_project_meta`].
    #[serde(flatten)]
    #[serde(serialize_with = "Project::emit_project_meta")]
    project_meta: (),
    /// The song described by this project.
    pub song: Song,
}

impl Project {
    /// Create an empty project with the given song metadata.
    pub fn new(song_meta: SongMeta) -> Self {
        Self {
            project_meta: (),
            song: Song::new(song_meta),
        }
    }

    /// Get the metadata for this project format version.
    ///
    /// See [`ProjectMeta`] for more information.
    pub const fn project_meta() -> &'static ProjectMeta {
        ProjectMeta::v1()
    }

    /// Add a field to the serializer containing project
    /// format metadata.
    /// This is later used to choose which version of the
    /// format to deserialize a project with.
    ///
    /// See [`ProjectMeta`] and
    /// [`AnyProject::from_json_str`](crate::project::any::AnyProject::from_json_str)
    /// for more information.
    fn emit_project_meta<S: Serializer>(_: &(), s: S) -> Result<S::Ok, S::Error> {
        Self::project_meta().serialize(s)
    }
}
