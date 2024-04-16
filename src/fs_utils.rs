use std::{cmp::Ordering, env, fs, path::Path};

use thiserror::Error;

type Files = Vec<File>;

pub struct File(fs::DirEntry);

impl File {
    fn compare_file_modified(&self, other: &Self) -> Result<Ordering, FSError> {
        Ok(self
            .0
            .metadata()?
            .modified()?
            .cmp(&other.0.metadata()?.modified()?))
    }

    pub fn get_file_name(&self) -> Result<String, FSError> {
        self.0
            .file_name()
            .into_string()
            .map_err(|_| FSError::FailedToFetchFileName)
    }
}

#[derive(Error, Debug)]
pub enum FSError {
    #[error("No such package found")]
    NoPackageFound,

    #[error("IO error occured")]
    IOError(#[from] std::io::Error),

    #[error("Failed to fetch file name")]
    FailedToFetchFileName,

    #[error("Unknown error occured")]
    UnknownErrorOccured,
}
pub fn get_current_directory_name() -> Result<String, FSError> {
    env::current_dir()?
        .file_name()
        .and_then(|f| f.to_str())
        .map(|str| str.to_string())
        .ok_or(FSError::UnknownErrorOccured)
}

pub fn parse_package_to_directory(name: &String) -> Result<&Path, FSError> {
    let path = Path::new(name);
    match path.is_dir() {
        true => Ok(&path),
        false => Err(FSError::NoPackageFound),
    }
}

pub fn get_latest_file_from_directory(path: &Path) -> Result<String, FSError> {
    let files = get_files_from_directory(path)?;
    files[0]
        .0
        .path()
        .to_str()
        .map(|s| s.to_string())
        .ok_or(FSError::UnknownErrorOccured)
}

pub fn get_files_from_directory(path: &Path) -> Result<Files, FSError> {
    let files: Result<Vec<File>, FSError> = path
        .read_dir()?
        .into_iter()
        .map(|entry| Ok(File(entry?)))
        .filter(|file| file.as_ref().unwrap().0.path().is_file())
        .collect();
    files.map(|mut files| {
        files.sort_by(|a, b| {
            b.compare_file_modified(a)
                .expect("Failed to compare file modified time")
        });
        files
    })
}
