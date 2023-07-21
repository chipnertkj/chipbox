//! Implements the structure of a project `Tree` and methods to load project trees from the filesystem.

use crate::app_state::settings::{project_tree_location, ProjectTreeLocation};
use std::fs::{self, DirEntry};
use std::path::{Path, PathBuf};
use std::{error, fmt, io, result};

/// Result type alias for this module's `Error` type.
pub(crate) type Result<T> = result::Result<T, Error>;

/// Errors encountered during the deserialization of a project tree.
#[derive(Debug)]
pub(crate) enum Error {
    /// See inner type for more information.
    Io(io::Error),
    /// See inner type for more information.
    ProjectTreeLocation(project_tree_location::Error),
    /// Project file was unexpectedly found in root.
    RootIsProject(Project),
    /// Invalid entry type.
    FileEntry(PathBuf),
    /// Invalid entry type.
    SymlinkEntry(PathBuf),
    /// Invalid entry type.
    UnknownEntry(PathBuf),
    /// Root directory has no filesystem parent.
    NoParentDir,
}

impl error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NoParentDir => {
                f.write_str("could not find project tree parent dir")
            }
            Self::Io(e) => write!(f, "io error during project tree read: {e}"),
            Self::ProjectTreeLocation(e) => {
                write!(f, "could not read project tree location setting: {e}")
            }
            Self::RootIsProject(project) => {
                write!(
                    f,
                    "project tree root was detected to be a project: {project:?}"
                )
            }
            Self::FileEntry(path) => {
                write!(f, "found unexpected file entry at: `{path:?}`")
            }
            Self::SymlinkEntry(path) => {
                write!(f, "found unexpected symlink entry at: `{path:?}`")
            }
            Self::UnknownEntry(path) => {
                write!(f, "found unexpected unknown entry at: `{path:?}`")
            }
        }
    }
}

/// A project tree entry. Represents a collection of `Project`s or other `Group`s.
#[derive(Debug)]
pub(crate) struct Group {
    path: PathBuf,
    children: Vec<Entry>,
}

impl Group {
    /// Create a `Group` from a filesystem path.
    fn from_path<P: AsRef<Path>>(path: P) -> Result<Self> {
        fn inner(path: &Path) -> Result<Group> {
            let children = path
                .read_dir()
                .map_err(Error::Io)?
                .map(|x| Entry::from_dir_entry(x.map_err(Error::Io)?))
                .collect::<Result<_>>()?;
            Ok(Group {
                path: path.to_path_buf(),
                children,
            })
        }
        inner(path.as_ref())
    }
}

/// A project tree entry. Represents a chipbox project.
///
/// # Notes
/// This type makes no guarantees on the validity of the directory
/// at `self.path` as a deserialized editor `Project` representation.
#[derive(Debug)]
pub(crate) struct Project {
    path: PathBuf,
}

impl Project {
    /// File name of the project file, located under the project folder.
    const INNER_FILE_NAME: &str = "chipbox_project.json";

    /// Attempt to create a `Project` from a filesystem path.
    /// Returns `Ok(None)` if the file is not a project directory.
    fn try_from_path<P: AsRef<Path>>(path: &P) -> io::Result<Option<Self>> {
        fn inner(path: &Path) -> io::Result<Option<Project>> {
            let dir_entries = path
                .read_dir()?
                .collect::<io::Result<Vec<_>>>()?;
            let entries_meta = dir_entries
                .iter()
                .collect::<Vec<_>>()
                .into_iter()
                .map(|x| x.metadata())
                .collect::<io::Result<Vec<_>>>()?;
            let is_project = dir_entries
                .into_iter()
                .zip(entries_meta)
                .any(|(entry, meta)| {
                    meta.is_file()
                        && entry.file_name() == Project::INNER_FILE_NAME
                });
            if is_project {
                Ok(Some(Project {
                    path: path.to_path_buf(),
                }))
            } else {
                Ok(None)
            }
        }
        inner(path.as_ref())
    }
}

/// Enumeration of the possible items in a project tree.
#[derive(Debug)]
pub(crate) enum Entry {
    Group(Group),
    Project(Project),
}

impl Entry {
    /// Create an `Entry` from a filesystem directory entry.
    fn from_dir_entry(dir_entry: DirEntry) -> Result<Self> {
        let meta = dir_entry
            .metadata()
            .map_err(Error::Io)?;
        if meta.is_dir() {
            let path = dir_entry.path();
            let project_opt =
                Project::try_from_path(&path).map_err(Error::Io)?;
            match project_opt {
                Some(project) => Ok(Entry::Project(project)),
                None => Ok(Entry::Group(Group::from_path(path)?)),
            }
        } else if meta.is_file() {
            Err(Error::FileEntry(dir_entry.path()))
        } else if meta.is_symlink() {
            Err(Error::SymlinkEntry(dir_entry.path()))
        } else {
            Err(Error::UnknownEntry(dir_entry.path()))
        }
    }
}

/// A logical representation of a user's project tree directory.
#[derive(Debug)]
pub(crate) struct Tree {
    root: Group,
}

impl Tree {
    /// Retrieve a `DirEntry` pointing to the project tree directory.
    /// This function iterates through the parent directory of the project tree in order to access it.
    fn dir_entry<P: AsRef<Path>>(path: P) -> Result<DirEntry> {
        fn inner(path: &Path) -> Result<DirEntry> {
            let exists = path
                .try_exists()
                .map_err(Error::Io)?;
            if !exists {
                fs::create_dir_all(path).map_err(Error::Io)?;
            }
            let path = path
                .canonicalize()
                .map_err(Error::Io)?;
            let root_dir_name = path
                .file_name()
                .expect("canonical path should not end with `..`");
            let parent_dir = path
                .parent()
                .ok_or(Error::NoParentDir)?;
            let parent_dir_entries = parent_dir
                .read_dir()
                .map_err(Error::Io)?
                .collect::<Vec<io::Result<DirEntry>>>()
                .into_iter()
                .collect::<io::Result<Vec<DirEntry>>>()
                .map_err(Error::Io)?;
            let dir_entry_opt = parent_dir_entries
                .into_iter()
                .find(|x| x.file_name() == root_dir_name);
            match dir_entry_opt {
                Some(dir_entry) => Ok(dir_entry),
                _ => unreachable!("see `parent_dir_entries`"),
            }
        }
        inner(path.as_ref())
    }

    /// Construct a `Tree` based on a filesystem directory.
    fn from_path<P: AsRef<Path>>(path: P) -> Result<Self> {
        fn inner(path: &Path) -> Result<Tree> {
            let root_entry = Entry::from_dir_entry(Tree::dir_entry(path)?)?;
            match root_entry {
                Entry::Group(group) => Ok(Tree { root: group }),
                Entry::Project(project) => Err(Error::RootIsProject(project)),
            }
        }
        inner(path.as_ref())
    }

    /// Construct a `Tree` based on a filesystem directory, decided by the `ProjectTreeLocation` setting.
    pub(crate) fn from_setting(setting: &ProjectTreeLocation) -> Result<Self> {
        let path = setting
            .path()
            .map_err(Error::ProjectTreeLocation)?;
        Self::from_path(path)
    }
}

mod display {
    //! Internals related to `Display` impls in `super`.
    use core::fmt;
    use parking_lot::Mutex;
    use std::iter::repeat_n;

    /// String that represents one level of indentation.
    pub(super) const ENTRY_INDENT: &str = "  ";
    /// String that appears before each `Group`.
    pub(super) const GROUP_PREFIX: &str = "> ";
    /// String that appears after each `Group`.
    pub(super) const GROUP_POSTFIX: &str = ":\n";
    /// String that appears before each `Project`.
    pub(super) const PROJECT_PREFIX: &str = "- ";
    /// String that appears after each ``Project`.
    pub(super) const PROJECT_POSTFIX: &str = "\n";

    /// Static mutex used in `write_indent` and `update_indent`.
    /// Represents the current level of entry indentation.
    static DISPLAY_INDENT_MX: Mutex<usize> = Mutex::new(0);

    /// Write current entry indentation to a buffer.
    pub(super) fn write_indent(f: &mut fmt::Formatter<'_>) -> fmt::Result {
        repeat_n(
            (),
            *DISPLAY_INDENT_MX
                .try_lock()
                .unwrap(),
        )
        .try_for_each(|_| f.write_str(ENTRY_INDENT))
    }

    /// Update current entry indentation using a closure.
    pub(super) fn update_indent<F>(f: F)
    where
        F: FnOnce(&mut usize),
    {
        f(&mut DISPLAY_INDENT_MX
            .try_lock()
            .unwrap())
    }
}

impl fmt::Display for Group {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        display::write_indent(f)?;
        let name = self
            .path
            .file_name()
            .ok_or(fmt::Error)?
            .to_str()
            .ok_or(fmt::Error)?;
        write!(
            f,
            "{prefix}{name}{post}",
            prefix = display::GROUP_PREFIX,
            post = display::GROUP_POSTFIX
        )?;

        display::update_indent(|x| *x += 1);
        self.children
            .iter()
            .try_for_each(|x| write!(f, "{x}"))?;
        display::update_indent(|x| *x -= 1);

        Ok(())
    }
}

impl fmt::Display for Project {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        display::write_indent(f)?;
        let name = self
            .path
            .file_name()
            .ok_or(fmt::Error)?
            .to_str()
            .ok_or(fmt::Error)?;
        write!(
            f,
            "{pre}{name}{post}",
            pre = display::PROJECT_PREFIX,
            post = display::PROJECT_POSTFIX
        )
    }
}

impl fmt::Display for Entry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Group(group) => write!(f, "{group}"),
            Self::Project(project) => write!(f, "{project}"),
        }
    }
}

impl fmt::Display for Tree {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.root)
    }
}

#[cfg(test)]
mod test {
    use super::{display, Project, Tree};
    use crate::app_state::settings::ProjectTreeLocation;
    use crate::path;
    use color_eyre::eyre::{self, eyre};
    use std::fs;

    /// Create a filesystem project tree in a temporary directory, with two projects:
    /// - "project 1" in root
    /// - "project 2" in "group 1"
    ///
    /// Afterwards, try to deserialize it into a `Tree` and assert the result is as expected.
    #[test]
    fn dir_read_from_setting_and_display() -> eyre::Result<()> {
        color_eyre::install()?;
        let _temp_dir = path::create_temp_dir()?;

        let setting = ProjectTreeLocation::default();
        let tree_path = setting.path()?;
        const GROUP1_NAME: &str = "group 1";
        const PROJECT1_NAME: &str = "project 1";
        const PROJECT2_NAME: &str = "project 2";
        let project1_path = tree_path.join(PROJECT1_NAME);
        let project2_path = tree_path
            .join(GROUP1_NAME)
            .join(PROJECT2_NAME);

        fs::create_dir_all(&project1_path)?;
        fs::create_dir_all(&project2_path)?;
        fs::File::create(project1_path.join(Project::INNER_FILE_NAME))?;
        fs::File::create(project2_path.join(Project::INNER_FILE_NAME))?;

        let tree_string = Tree::from_setting(&setting)?.to_string();
        let expected_string = format!(
            "{grp_prfx}{tree}{grp_pofx}{indent}{grp_prfx}{grp1}{grp_pofx}{indent}{indent}{proj_prfx}{proj2}{proj_pofx}{indent}{proj_prfx}{proj1}{proj_pofx}",
            grp_prfx = display::GROUP_PREFIX,
            grp_pofx = display::GROUP_POSTFIX,
            proj_prfx = display::PROJECT_PREFIX,
            proj_pofx = display::PROJECT_POSTFIX,
            indent = display::ENTRY_INDENT,
            proj1 = PROJECT1_NAME,
            proj2 = PROJECT2_NAME,
            grp1 = GROUP1_NAME,
            tree = tree_path
                .canonicalize()?
                .file_name()
                .ok_or_else(|| eyre!("canonical path should not end with `/..`"))?
                .to_str()
                .ok_or_else(|| eyre!("tree path is not valid utf-8"))?
        );

        println!("expected:\n{expected_string}actual:\n{tree_string}");
        assert_eq!(tree_string, expected_string);
        Ok(())
    }
}
