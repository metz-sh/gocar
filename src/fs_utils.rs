use std::{cmp::Ordering, env, fs, io, path::Path};

type SortedDirEntries = Vec<io::Result<fs::DirEntry>>;

pub fn get_current_directory_name() -> String {
    String::from(
        env::current_dir()
            .unwrap()
            .file_name()
            .unwrap()
            .to_str()
            .unwrap(),
    )
}

pub fn parse_package_to_directory(name: &String) -> Result<&Path, String> {
    let path = Path::new(name);
    match path.is_dir() {
        true => Ok(&path),
        false => Err(String::from("No such package found!")),
    }
}

fn compare_file_metadata(
    a: &io::Result<fs::DirEntry>,
    b: &io::Result<fs::DirEntry>,
) -> io::Result<Ordering> {
    Ok(b.as_ref()
        .unwrap()
        .metadata()?
        .modified()?
        .cmp(&a.as_ref().unwrap().metadata()?.modified()?))
}

pub fn get_latest_file_from_directory(path: &Path) -> Result<String, String> {
    let files = get_files_from_directory(path);
    Ok(files[0]
        .as_ref()
        .unwrap()
        .path()
        .to_str()
        .map(|s| s.to_string())
        .unwrap())
}

pub fn get_files_from_directory(path: &Path) -> SortedDirEntries {
    let mut files: Vec<io::Result<fs::DirEntry>> = path
        .read_dir()
        .unwrap()
        .into_iter()
        .filter(|entry| entry.as_ref().unwrap().path().is_file())
        .collect();
    files.sort_by(move |a, b| compare_file_metadata(a, b).unwrap());
    files
}
