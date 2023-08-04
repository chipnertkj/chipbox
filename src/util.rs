//! Utility functions and helpers for the application.
//!
//! This module contains various utility functions and helpers that are used throughout
//! the application. It provides common functionalities that can be reused in different
//! parts of the codebase.
//!
//! # Examples
//!
//! ```
//! use crate::util::enable_confirm_refresh;
//!
//! // Enable a confirmation dialog for when the user tries to refresh the page.
//! enable_confirm_refresh();
//! ```

pub(crate) use confirm_refresh::enable_confirm_refresh;

mod confirm_refresh; // ok
