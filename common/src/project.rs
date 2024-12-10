//! Project format used internally by `chipbox`.
//!
//! The format is designed to be backwards compatible, allowing the
//! introduction of breaking changes to it's internal structure without
//! affecting the users ability to deserialize and operate on
//! projects created with older versions of the software.
//!
//! # `latest` module re-export
//! To create an instance using the latest version of the project
//! format, use the [`latest`] module
//! re-export.
//!
//! This re-export will keep changing its underlying module
//! in the future as new versions of the project format are added.
//! This **will** introduce breaking changes to consumers of the
//! [`latest`] module API, and this is intentional.
//! Consumers must update their code to use the newer API when
//! those changes are made.
//!
//! New versions of the [`latest`] module will have their changes
//! since the last version documented in the module level documentation.
//!
//! If you wish to use a static version of the project format
//! instead, see the [section below](#working-with-older-versions).
//! This may be desirable if breaking changes in the API are not
//! something that is tolerable by your use case.
//!
//! # Working with older versions
//! To convert an older version of the project format to the
//! latest version, use the [`any`] module and its
//! [`AnyProject`](crate::project::any::AnyProject) type.
//! See the module level documentation for [`any`]
//! for more information on exact usage.
//!
//! The `project` module also exports separate modules for
//! each concrete version of the project format, like the
//! [`v1`] module, in case you need to work with older
//! versions of the project format without converting them
//! to the latest version.

// Changing this re-export to another module (like `v2`, `v3` etc)
// will change the behavior of [`any`] and most definitely introduce
// breaking changes.
//
// This is intentional - the latest version of the project
// is effectively defined by this re-export, and consumers
// will need to update their code on latest version changes.
//
// See the module level documentation for more information.
pub use v1 as latest;

pub mod any;
pub mod meta;
pub mod v1;
