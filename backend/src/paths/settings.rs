use crate::paths;
use std::path::PathBuf;

pub(crate) async fn file_path() -> Result<PathBuf, paths::data::DataDirError> {
    let dir_path = paths::data::dir_path().await?;
    Ok(dir_path.join("chipbox-settings.json"))
}
