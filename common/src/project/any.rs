//! Convert older versions of the project format to the latest
//! supported version.
//!
//! The [`AnyProject`] type allows you to convert from older versions
//! of the project format. It implements [`From`] for every
//! version, and has a method [`into_latest`](AnyProject::into_latest)
//! which recursively converts a project to use the latest available
//! version, as defined by the [`latest`] module re-export.
//!
//! See the [`project`](crate::project) module documentation
//! for more information on working with the latest version
//! of the format.

use super::{
    latest,
    meta::{ProjectMeta, ProjectVersion},
    v1,
};

/// A wrapper enum around [`latest::Project`] and its older
/// versions. Can be used to contain a project which
/// uses any version of the format.
///
/// This type has a marker variant, [`Latest`](AnyProject::Latest).
/// It is equivalent to one of the other, concrete variants,
/// except it also marks the inner value as using the latest
/// version of the format.
///
/// The latest version of [`Project`](latest::Project) is
/// decided by the [`crate::project::latest`] module re-export.
/// See [`project`](crate::project) module documentation for more information.
///
/// See [`Self::from_json_str`] and [`Self::into_latest`] for information on how
/// to load and convert projects from older versions of the format using this type.
#[derive(derive_more::From, Debug, Clone)]
pub enum AnyProject {
    /// This is a marker variant for a project which uses
    /// the latest version of the format.
    ///
    /// It is equivalent to one of the other, concrete
    /// variants, except it also marks the inner value
    /// as using the latest version of the format.
    ///
    /// The latest version of [`Project`](latest::Project) is
    /// decided by the [`latest`] module re-export.
    /// You can check which version is the latest using
    /// [`latest::Project::project_meta`].
    #[from(skip)] // Handled by `Self::into_latest` instead!
    Latest(latest::Project),
    /// First version of the project format.
    /// See the [`v1`] module for more information.
    V1(v1::Project),
}

impl AnyProject {
    /// Get the version of the underlying project.
    ///
    /// This function will return the same value for two of the
    /// variants, as the [`Latest`](AnyProject::Latest) variant
    /// is a marker wrapper around one of the concrete variants.
    ///
    /// See [`AnyProject`] for more information.
    pub fn version(&self) -> ProjectVersion {
        match self {
            Self::Latest(..) => ProjectVersion::latest(),
            Self::V1(..) => v1::Project::project_meta().version(),
        }
    }

    /// Recursively convert [`self`] into the latest version of
    /// the format, [`latest::Project`].
    ///
    /// This function automatically applies the necessary migration
    /// tasks at every step in the conversion.
    pub fn into_latest(self) -> latest::Project {
        match self {
            // `self` is latest, return!
            Self::Latest(project) => project,
            // Convert self into the next version and recursively
            // attempt to return it as `latest::Project`.
            _ => self.upgrade().into_latest(),
        }
    }

    /// Convert [`self`](AnyProject) into the next version of the format.
    ///
    /// If the format used is already the latest version, this
    /// function will instead wrap the underlying value in [`AnyProject::Latest`].
    ///
    /// # Panics
    /// This function will panic if [`self`](AnyProject) is already
    /// [`Self::Latest`](AnyProject::Latest).
    ///
    /// This is to discourage the pointless operation.
    /// Additionally, this could cause an infinite loop if used improperly.
    fn upgrade(self) -> AnyProject {
        match self {
            // V1 is currently the latest.
            Self::V1(v1) => Self::Latest(v1),
            // Self::Latest is already the latest recognized version.
            Self::Latest(..) => {
                panic!(
                    "attempted to upgrade AnyProject::Latest - this could cause an infinite loop"
                )
            }
        }
    }

    /// Deserialize a project of any supported format version from its
    /// JSON representation.
    ///
    /// This function will never return the value as the
    /// [`Latest`](AnyProject::Latest) variant, even if
    /// the serialized project is using the latest version of
    /// the format. To retrieve the inner project as the
    /// latest version, use [`into_latest`](Self::into_latest)
    /// on the returned value.
    ///
    /// See [`AnyProject`] for more information.
    ///
    /// # Version detection mechanism
    /// Version detection is done by attempting to deserialize
    /// the project as a [`ProjectMeta`] object, and then
    /// checking for the output of [`ProjectMeta::version`].
    /// This means that any project compliant with this API
    /// should additionally deserialize into a
    /// [flattened](https://serde.rs/attr-flatten.html)
    /// version of [`ProjectMeta`].
    ///
    /// This can be done without wasting precious memory, while also being able to use
    /// the [`Serialize`] derive macro like in the following example:
    /// ```
    /// # use serde::{Serialize, Deserialize, Serializer};
    /// # use chipbox_common::project::meta::ProjectMeta;
    /// // Prevent clippy warning from `()` field.
    /// // We are intentionally using a zero-sized type!
    /// #[allow(clippy::manual_non_exhaustive)]
    /// #[derive(Serialize, Deserialize)]
    /// pub struct Project {
    ///     #[serde(flatten)]
    ///     #[serde(serialize_with = "emit_project_meta")]
    ///     project_meta: (),
    ///     // other fields...
    /// }
    ///
    /// fn emit_project_meta<S: Serializer>(_: &(), s: S) -> Result<S::Ok, S::Error> {
    ///     let meta: ProjectMeta = unimplemented!("define meta for this version");
    ///     meta.serialize(s)
    /// }
    /// ```
    /// This lets you deserialize it as a [`ProjectMeta`]
    /// object, without it being actually stored in the struct.
    /// Thanks to this, [`from_json_str`](Self::from_json_str) is now able to
    /// read the metadata and determine which version of the
    /// project format to use.
    pub fn from_json_str(s: &str) -> serde_json::Result<Option<Self>> {
        let meta: ProjectMeta = serde_json::from_str(s)?;
        let any_opt = match meta.version() {
            ProjectVersion::Unrecognized => None,
            ProjectVersion::V1 => Some(Self::V1(serde_json::from_str(s)?)),
        };
        Ok(any_opt)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use v1::song::meta::SongMeta;

    /// Ensure [`AnyProject::from_json_str`] works as expected.
    ///
    /// Attempts to deserialize a [`v1::Project`] to [`AnyProject`].
    #[test]
    fn deserialize_v1_to_any() {
        let project = v1::Project::new(SongMeta::new_now("abc", None::<&str>, []));
        let s = serde_json::to_string(&project).expect("serialize");
        AnyProject::from_json_str(&s).expect("deserialize");
    }

    /// Ensure a project of the latest version can be deserialized back
    /// to the latest version of the format using [`AnyProject::into_latest`].
    #[test]
    fn deserialize_latest_to_latest() {
        let project = latest::Project::new(SongMeta::new_now("abc", None::<&str>, []));
        let s = serde_json::to_string(&project).expect("serialize");
        AnyProject::from_json_str(&s)
            .expect("deserialize")
            .expect("recognize latest version")
            .into_latest();
    }
}
