use std::{ffi::OsString, fs};

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
    file_path: &OsString,
    pattern: &String,
    replace: &String,
    edit: bool,
    delete: bool,
) {
    let mut file_contents = fs::read_to_string(file_path).unwrap();

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
        fs::write(file_path, file_contents).unwrap();
    } else {
        println!("{}", file_contents);
    }
}

/// Collects the file paths as OsStrings from a given directory into a vector.
/// * Parameters
///     * path: Path to the directory.
///     * full: If true will recursively collect the file paths from all the subdirectories too.
///     Otherwise it will print 'fre: \<path\>: is directory'.
pub fn collect_files(path: &OsString, full: bool) -> Vec<OsString> {
    let dir = fs::read_dir(path).unwrap();

    dir.fold(Vec::new(), |mut acc, entry| {
        let entry = entry.unwrap();
        let entry_type = entry.file_type().unwrap();
        let path = entry.path().as_os_str().to_os_string();

        if entry_type.is_dir() {
            if !full {
                println!("fre: {:?}: is directory", entry.path().as_os_str());
            }
            // -rf flag is set:
            else {
                acc.extend(collect_files(&path, full));
            }
        } else {
            acc.push(path);
        }

        acc
    })
}
