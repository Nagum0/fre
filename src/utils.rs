use std::{ffi::OsString, fs, io::ErrorKind, rc::Rc};

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
pub fn transform_file_contents(
    file_path: Rc<OsString>,
    pattern: &String,
    replace: &String,
    edit: bool,
    delete: bool,
) -> Result<(), FreError> {
    // I make a separate reference for the value of file_path because the `fs` functions
    // expect a &OsString but the file_path holds a OsString:
    let reference_to_file_path = &(*file_path);

    // Getting file metadata:
    let metadata = fs::metadata(reference_to_file_path).map_err(|e| match e.kind() {
        ErrorKind::NotFound => FreError::FileError(Rc::clone(&file_path), "File not found"),
        _ => FreError::FileError(Rc::clone(&file_path), "Can't get file metadata"),
    })?;

    // If the given path is a directory:
    if metadata.is_dir() {
        return Err(FreError::FileError(Rc::clone(&file_path), "Is a directory"));
    }

    // Get the file contents:
    let mut file_contents =
        fs::read_to_string(reference_to_file_path).map_err(|e| match e.kind() {
            ErrorKind::InvalidData => FreError::InvalidData(Rc::clone(&file_path)),
            ErrorKind::NotFound => FreError::FileError(Rc::clone(&file_path), "File not found"),
            _ => FreError::FileError(Rc::clone(&file_path), "Error while reading file contents"),
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
        fs::write(reference_to_file_path, file_contents).map_err(|e| match e.kind() {
            ErrorKind::InvalidData => FreError::InvalidData(Rc::clone(&file_path)),
            ErrorKind::NotFound => FreError::FileError(Rc::clone(&file_path), "File not found"),
            ErrorKind::PermissionDenied => {
                FreError::FileError(Rc::clone(&file_path), "Premission denied")
            }
            _ => FreError::FileError(Rc::clone(&file_path), "Error while reading file contents"),
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
pub fn collect_files(path: Rc<OsString>, full: bool) -> Result<Vec<Rc<OsString>>, FreError> {
    let dir = fs::read_dir(&(*path)).map_err(|e| match e.kind() {
        ErrorKind::InvalidInput => FreError::DirError(Rc::clone(&path), "Not a directory"),
        _ => FreError::DirError(Rc::clone(&path), "Error while reading directory"),
    })?;

    let mut files: Vec<Rc<OsString>> = vec![];
    for entry in dir {
        let entry =
            entry.map_err(|_| FreError::FileError(Rc::clone(&path), "Unable to get entry"))?;
        let entry_path = Rc::new(entry.path().as_os_str().to_os_string());
        let entry_type = entry
            .file_type()
            .map_err(|_| FreError::FileError(Rc::clone(&path), "Unable to get file type"))?;

        if entry_type.is_dir() {
            if !full {
                println!("fre: {:?}: is directory", entry_path);
            }
            // -rf flag is set:
            else {
                files.extend(collect_files(entry_path, full)?);
            }
        } else {
            files.push(entry_path);
        }
    }

    Ok(files)
}
