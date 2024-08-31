use std::{ffi::OsString, fs, io::ErrorKind};

use crate::fre_error::FreError;

/// Transform the contents of a given file.
/// * Parameters:
///     * file_path: The path to the file.
///     * pattern: Pattern to match against.
///     * to: String to replace the matched patters.
///     * edit: If this is true it will write the transformed stirng to the given file.
///     If not it will print the transformed result to stdout and doesn't touch the given file.
///     * delete: If this is true it will ignore the 'to' parameter and delete the line containing the matched
///     pattern.
pub fn transform_file_contents<'a>(
    file_path: &'a OsString,
    pattern: &String,
    replace: &String,
    edit: bool,
    delete: bool,
) -> Result<(), FreError<'a>> {
    // Getting file metadata:
    let metadata = fs::metadata(file_path).map_err(|e| match e.kind() {
        ErrorKind::NotFound => FreError::FileError(file_path, "File not found"),
        _ => FreError::FileError(file_path, "Can't get file metadata"),
    })?;

    // If the given path is a directory:
    if metadata.is_dir() {
        return Err(FreError::FileError(file_path, "Is a directory"));
    }

    // Get the file contents:
    let mut file_contents = fs::read_to_string(file_path).map_err(|e| match e.kind() {
        ErrorKind::InvalidData => FreError::InvalidData(file_path),
        ErrorKind::NotFound => FreError::FileError(file_path, "File not found"),
        _ => FreError::FileError(file_path, "Error while reading file contents"),
    })?;

    // -d flag is set:
    if delete {
        file_contents = file_contents
            .lines()
            .filter(|line| !line.contains(pattern))
            .collect::<Vec<&str>>()
            .join("\n");
    } else {
        file_contents = file_contents.replace(pattern, replace);
    }

    // -e flag is set:
    if edit {
        fs::write(file_path, file_contents).map_err(|e| match e.kind() {
            ErrorKind::InvalidData => FreError::InvalidData(file_path),
            ErrorKind::NotFound => FreError::FileError(file_path, "File not found"),
            ErrorKind::PermissionDenied => FreError::FileError(file_path, "Premission denied"),
            _ => FreError::FileError(file_path, "Error while reading file contents"),
        })?;
    } else {
        println!("{}", file_contents);
    }

    Ok(())
}

/// Collects the file paths as OsStrings from a given directory into a vector.
/// * Parameters
///     * path: Path to the directory.
///     * full: If true will recursively collect the file paths from all the subdirectories too.
///     Otherwise it will print 'fre: \<path\>: is directory'.
pub fn collect_files<'a>(path: &'a OsString, full: bool) -> Result<Vec<OsString>, FreError<'a>> {
    let dir = fs::read_dir(path).map_err(|e| match e.kind() {
        ErrorKind::InvalidInput => FreError::DirError(path, "Not a directory"),
        _ => FreError::DirError(path, "Error while reading directory"),
    })?;

    let mut files: Vec<OsString> = vec![];
    for entry in dir {
        let entry = entry.map_err(|_| FreError::FileError(path, "Unable to get entry"))?;
        let entry_type = entry
            .file_type()
            .map_err(|_| FreError::FileError(path, "Unable to get file type"))?;

        if entry_type.is_dir() {
            if !full {
                println!("fre: {:?}: is directory", entry.path().as_os_str());
            }
            // -rf flag is set:
            else {
                files.extend(collect_files(path, full)?);
            }
        } else {
            files.push(entry.path().as_os_str().to_os_string());
        }
    }

    Ok(files)
}
