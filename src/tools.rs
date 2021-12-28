//! All the sorting tools used by the command-line interface. Contains the main 
//! sorting algorithm.

use crate::structs::File;
use super::messages::{error_messages, DryRunMessage, ProgressBar};
use std::fs;

/// The main sorting algorithm, and all the functions it needs to operate.
pub mod sort {

    // use super::super::messages::error_messages;
    use chrono::{DateTime, TimeZone, Utc, Local};
    use crate::{error_messages, messages::{DryRunMessage, ProgressBar}, structs::{ConfigData, File}};
    use std::{fs, path::Path, time::UNIX_EPOCH};
    use walkdir::WalkDir;

    #[cfg(test)]
    mod tests {
        /// Tests for tools. Each test function is named after the function in
        /// tools it tests, with the test_ prefix.

        use std::{env, path::Path};
        use super::*;

        #[test]
        fn test_get_sequential_name() {
            let parent_dir = File::from_pathbuf(&env::current_dir().expect("Failed to get current dir."));
            let old_path = parent_dir.join(Path::new("testing/test.txt"));
            let existing_path = parent_dir.join(Path::new("testing/test_1.txt"));
            let new_path = parent_dir.join(Path::new("testing/test_2.txt"));
            let mut vec = Vec::new();
            vec.push(existing_path);
            
            assert_eq!(new_path, get_sequential_name(&old_path, &vec));
        }

        #[test]
        fn test_get_sorting_results() {

            // The parameters for testing
            let source = File::from_pathbuf(&env::current_dir().expect("Failed to get current dir"));
            let source = source.join(Path::new("testing"));
            let target = source.join(Path::new("target"));
            let date_format = "%Y";
            let date_type = "m";
            let preserve_name = true;
            let exclude_type = ("txt", true);
            let only_type = ("", false);
            
            // Get the sorting results
            let results = get_sorting_results(
                &source,
                &target,
                &date_format,
                &date_type,
                &preserve_name,
                exclude_type,
                only_type
            );
            let (old, new) = (&results.1, &results.2);
            
            // Print all the options in case of a test failure
            for i in 0..4 {
                println!("Old: {:?}, New: {:?}", old[i].copy(), new[i].copy());
            }
            
            // Check that the sorting results are correct
            assert_eq!((old[0].copy(), new[0].copy()), (source.join(Path::new("test.jpg")), source.join(Path::new("target/2021/02/2021 test.jpg"))));
            assert_eq!((old[1].copy(), new[1].copy()), (source.join(Path::new("test")), source.join(Path::new("target/2021/02/2021 test."))));
            assert_eq!((old[2].copy(), new[2].copy()), (source.join(Path::new("files/test")), source.join(Path::new("target/2021/02/2021 test_2."))));
            assert_eq!((old[3].copy(), new[3].copy()), (source.join(Path::new("test.png")), source.join(Path::new("target/2021/02/2021 test.png"))));
            assert_eq!(results.0, 4);
            assert_eq!(old.len(), 4);
            assert_eq!(new.len(), 4);
        }

        #[test]
        fn test_is_sortable() {
            let path = File::new("file.txt");

            assert!(is_sortable(&path, &(&"", false), &(&"txt", true)));
            assert!(is_sortable(&path, &(&"txt", true), &(&"txt", true)));
            assert!(is_sortable(&path, &(&"", false), &(&"", false)));
            assert!(!is_sortable(&path, &(&"txt", true), &(&"", false)));
        }

        #[test]
        fn test_is_type() {
            let path = File::new("file.txt");
            assert!(is_type(&path, &"txt"));
        }
    }
    
    /// Return the access date and time of `path` as the number of seconds since the
    /// UNIX epoch.
    fn get_epoch_secs_access(path: &File) -> i64 {
        let ctime_system = path.pathbuf.metadata().unwrap().accessed().expect("Failed to get atime");
        let secs: i64 = ctime_system.duration_since(UNIX_EPOCH).unwrap().as_secs() as i64;

        secs
    }
    
    /// Return the creation date and time of `path` as the number of seconds since the
    /// UNIX epoch.
    fn get_epoch_secs_creation(path: &File) -> i64 {
        let ctime_system = path.pathbuf.metadata().unwrap().created().expect("Failed to get ctime");
        let secs: i64 = ctime_system.duration_since(UNIX_EPOCH).unwrap().as_secs() as i64;

        secs
    }

    /// Return the modification date and time of `path` as the number of seconds since the
    /// UNIX epoch.
    fn get_epoch_secs_modified(path: &File) -> i64 {
        let ctime_system = path.pathbuf.metadata().unwrap().modified().expect("Failed to get mtime");
        let secs: i64 = ctime_system.duration_since(UNIX_EPOCH).unwrap().as_secs() as i64;

        secs
    }

    /// Return a [`DateTime`] instance representing the creation, modification,
    /// or access time of `path` according to `date_type`.
    /// 
    /// `date_type` must be one of `"c"` (created), `"a"` (accessed), or `"m"` (modified).
    /// Note that creation time is not available on all filesystems.
    fn get_datetime(path: &File, date_type: &str) -> DateTime<Local> {
        let secs: i64;
        if date_type == "m" {
            secs = get_epoch_secs_modified(path);
        } else if date_type == "a" {
            secs = get_epoch_secs_access(path);
        } else {
            secs = get_epoch_secs_creation(path);
        }
        let ctime = Utc.timestamp(secs, 0);
        let mytime = Local.from_utc_datetime(&ctime.naive_utc());

        mytime
    }

    /// Move `file` into a set of directories in yyyy/mm/ format according to its
    /// creation time. 
    /// 
    /// Create any required directories that don't already exist.
    /// Also rename the file according to its creation date.
    fn get_new_date_path(
        target: &File,
        old_file: &File,
        date_format: &str,
        date_type: &str,
        preserve_name: bool) -> File {
        
        // Get the time of old_file and set the names of the directories
        let ctime = get_datetime(old_file, &date_type);
        let dir = target.join(Path::new(&ctime.format("%Y/%m/").to_string()));

        // Preserve the original file name, if we're supposed to.
        let mut name_to_preserve = String::from("");
        if preserve_name {
            name_to_preserve = format!(
                " {}",
                old_file.file_stem()
            );
        }

        // Create the new file name
        let new_file = dir.join(Path::new(&format!(
            "{}{}.{}",
            &ctime.format(date_format).to_string(),
            name_to_preserve,
            old_file.extension()
        )));

        new_file
    }

    
    /// Return a [`File`] representing the renamed version of `path`.
    /// 
    /// This function is called only if `path` already exists, but can't/shouldn't
    /// be replaced. The naming logic: if `/path/to/file` already exists, return
    /// `/path/to/file_2`. If `/path/to/file_2` already exists, return `/path/to/file_3`, etc.
    fn get_sequential_name(path: &File, vec: &Vec<File>) -> File {

        let mut num = 2;

        loop {

            // Create the new path name
            let mut new_pathbuf = path.to_path_buf();
            new_pathbuf.set_file_name(&format!(
                "{}_{}.{}",
                path.pathbuf.file_stem().unwrap().to_str().unwrap(),
                num,
                path.pathbuf.extension().unwrap().to_str().unwrap()
            ));
            let new_file = File::from_pathbuf(&new_pathbuf);

            // Check if it exists, and if so, continue the loop
            if !vec.contains(&new_file) {
                return new_file;
            }
            num += 1;
        }
    }

    /// The main sorting algorithm; this checks files for validity and shows
    /// the progress bar.
    /// 
    /// The parameters are as follows:
    /// 
    /// <ul>
    /// <li>
    /// 
    /// `source` is the directory from which to get all the files to sort.
    /// </li>
    /// 
    /// <li>
    /// 
    /// `target` is the directory into which to sort all the files.
    /// </li>
    /// 
    /// <li>
    /// 
    /// `date_format` is the date *format* with which to rename the files. It shares
    /// the formatting rules with the [`chrono::format::strftime`] module.
    /// </li>
    /// 
    /// <li>
    /// 
    /// `date_type` is the date to sort the files by; one of `"c"` (created),
    /// `"a"` (accessed), or `"m"` modified. Note that creation time is not
    /// available on all filesystems.
    /// </li>
    /// 
    /// <li>
    /// 
    /// `preserve_name`, if set to [`true`], will add the original filename after
    /// the date, separated by a space. For example, sorting a file `test.txt` with
    /// `preserve_name=true` will rename `test.txt` to `2021-04-21 06h34m02s test.txt`
    /// </li>
    /// 
    /// <li>
    /// 
    /// `exclude_type` is a [`tuple`] containing two items. One is a [`str`] representing
    /// the extension of a file type to exclude from sorting. For example, if
    /// `"jpg"` is passed, all files ending in `.jpg` will be ignored during sorting.
    /// The other item is a [`bool`] telling whether `exclude_type` should take effect
    /// or not.
    /// </li>
    /// 
    /// <li>
    /// 
    /// `only_type` is [`tuple`] containing two items. One is a [`str`] representing
    /// the extension of a file type to exclusively sort. For example, if `"jpg"`
    /// is passed, *only* files ending in `.jpg` will be sorted; all others will be
    /// ignored. Overrides the `exclude_type` option. The other item is a [`bool`]
    /// telling whether `only_type` shoud take effect or not.
    /// </li>
    /// </ul>
    /// 
    /// This returns a three-item tuple containing: a [`usize`] representing the
    /// number of items to be sorted, a [`Vec<File>`] of the old file names, and
    /// a [`Vec<File>`] of the new file names. Each item in the old file names
    /// corresponds with the item of the same index in the new file names. So
    /// `old_names[0]` will be renamed to `new_names[0]`, `old_names[1]` will be
    /// renamed to `new_names[1]`, etc.
    fn get_sorting_results(
        source: &File,
        target: &File,
        date_format: &str,
        date_type: &str,
        preserve_name: &bool,
        exclude_type: (&str, bool),
        only_type: (&str, bool)) -> (usize, Vec<File>, Vec<File>) {

        // The vector to return: a tuple of (old_filename, new_filename)
        let mut vec_old: Vec<File> = Vec::new();
        let mut vec_new: Vec<File> = Vec::new();

        // Count the number of items we are going to sort
        let mut items_to_sort = 0;
        for entry in WalkDir::new(source.to_string()) {

            let entry = entry.unwrap();
            if !entry.metadata().expect("Failed to get dir metadata").is_dir() {
                if is_sortable(&File::from_path(entry.path()), &exclude_type, &only_type) {
                    items_to_sort += 1;
               }
            }
        }
        
        // Sort the everything, excluding the directories
        for entry in WalkDir::new(source.to_string()) {
            
            let entry = entry.unwrap();
            if !entry.metadata().expect("Failed to get dir metadata").is_dir() {

                // The File instance we are sorting
                let path = File::from_path(entry.path());

                // Make sure that we sort according to the exclude-type and
                // only-type arguments
                if is_sortable(&File::from_path(entry.path()), &exclude_type, &only_type) {

                    let mut new_file = get_new_date_path(&target, &path, date_format, date_type, *preserve_name);

                    // Get the sequential file name if new_file already exists
                    if vec_new.contains(&new_file) {
                        new_file = get_sequential_name(&new_file, &vec_new);
                    }

                    // Push the new and old file names to their respective vectors
                    vec_old.push(path.copy());
                    vec_new.push(new_file);
                }
            }
        }
        (items_to_sort, vec_old, vec_new)
    }
    
    /// Return [`true`] if:
    /// 1) `path`'s type is in `only_type.0` and `only_type.1` is [`true`]
    /// 2) `path`'s type is not in `exclude_type.0`, and `only_type.1` is [`false`]
    /// 
    /// "Type" refers to the file extension, as in `"jpg"`, `"png"`, etc. `exclude_type`
    /// and `only_type` correspond with `exclude_type` and `only_type` in [`get_sorting_results`],
    /// respectively.
    fn is_sortable(path: &File, exclude_type: &(&str, bool), only_type: &(&str, bool)) -> bool {

        if is_type(path, only_type.0) && only_type.1 {
            return true;
        } else if !is_type(path, exclude_type.0) && !only_type.1 {
            return true;
        } else {
            return false;
        }
    }

    /// Return [`true`] if `path`'s type is one of the types in `types`.
    /// "Type" refers to the file extension, as in `"jpg"`, `"png"`, etc.
    fn is_type(path: &File, types: &str) -> bool {
        let mut to_return: bool = false;
        for t in types.split("-") {
            if path.extension() == t {
                to_return = true;
            }
        }
        to_return
    }

    /// Print the intended sort, without acutally sorting. Each parameter
    /// corresponds with the parameter in [`get_sorting_results`] with the same name.
    pub fn sort_dry_run(
        source: &File,
        target: &File,
        date_format: &str,
        date_type: &str,
        preserve_name: &bool,
        exclude_type: (&str, bool),
        only_type: (&str, bool)) {

        let results = get_sorting_results(
            source,
            target,
            date_format,
            date_type,
            preserve_name,
            exclude_type,
            only_type
        );

        for i in 0..results.0 {
            println!("{}", DryRunMessage {
                from_file: results.1[i].copy(),
                to_file: results.2[i].copy(),
            }.to_string());
        }
    }

    /// The main sort function used by the command-line interface. Each parameter
    /// corresponds with the parameter in [`get_sorting_results`] of the same name,
    /// with the exception of `dry_run`.
    /// 
    /// If `dry_run` is [`true`], will print the intended sort without acutally sorting.
    pub fn sort(
        source: &File,
        target: &File,
        date_format: &str,
        date_type: &str,
        preserve_name: &bool,
        exclude_type: (&str, bool),
        only_type: (&str, bool),
        dry_run: bool) {

        // Do a dry run, if specified
        if dry_run {
            sort_dry_run(
                source,
                target,
                date_format,
                date_type,
                preserve_name,
                exclude_type,
                only_type
            );
            return;
        }

        // The results of the sorting algorithm
        let results = get_sorting_results(
            source,
            target,
            date_format,
            date_type,
            preserve_name,
            exclude_type,
            only_type
        );

        // The number of items to sort
        let items_to_sort = results.0;
        
        // The number of items we have sorted
        let mut items_sorted = 0;

        // The progress bar
        let progress_bar = ProgressBar {
            completed_message: String::from("Done."),
            message: String::from("Sorting..."),
            total: items_to_sort
        };

        for (old, new) in results.1.iter().zip(results.2.iter()) {

            // The file paths
            let old_file = old.pathbuf.as_path();
            let new_file = new.pathbuf.as_path();

            // Create the directory for the file, if it doesn't exist already
            let dir = new_file.parent().expect("Failed to get parent dir.");
            if !dir.exists() {
                fs::create_dir_all(&dir).expect("Failed to create dirs.");
            }

            // Rename the file
            fs::rename(&old_file, &new_file).expect(
                &error_messages::PathMoveFailedError {
                    source: &File::from_path(old_file),
                    target: &File::from_path(new_file),
                }.to_string()
            );
            items_sorted += 1;

            // Update the progress bar
            progress_bar.set_progress(items_sorted);
        }
        progress_bar.complete();
        println!("Sucessfully sorted {} items by date into {}.", items_sorted, target.to_string());
    }

    /// Sort according to configuration data in JSON [`String`] `json`. `source`
    /// and `target` correspond with the same-name parameters in [`get_sorting_results`].
    /// See [`crate::structs::ConfigData`] for more information on JSON configuration.
    /// 
    /// If `dry_run` is [`true`], will print the intended sort without acutally sorting.
    pub fn sort_from_json(json: String, source: File, target: File, dry_run: bool) {

        // Get the json data
        let data = ConfigData::from_json(&json);

        // Make sure that the directories actually exist
        let mut errors = false;
        if !source.exists() {
            println!("{}", error_messages::PathDoesNotExistError { path: &source }.to_string());
            errors = true;
        }
        if !target.exists() {
            println!("{}", error_messages::PathDoesNotExistError { path: &target }.to_string());
            errors = true;
        }
        if errors { return }

        // Run the sorting algorithm with the data, doing a dry run if specified
        let exclude_type: (&str, bool) = (&data.exclude_type.join("-"), data.exclude_type.len() > 0);
        let only_type: (&str, bool) = (&data.only_type.join("-"), data.only_type.len() > 0);
        if dry_run {
            sort_dry_run(
                &source,
                &target,
                data.date_format.as_str(),
                data.date_type.as_str(),
                &data.preserve_name,
                exclude_type,
                only_type
            );
        } else {
            sort(
                &source,
                &target,
                data.date_format.as_str(),
                data.date_type.as_str(),
                &data.preserve_name,
                exclude_type,
                only_type,
                false
            )
        }
    }
}

/// Move all the contents of SOURCE to TARGET, maintaining subdirectory structure.
/// If `dry_run` is [`true`], will print the intended sort without acutally sorting.
pub fn extract(source: &File, target: &File, dry_run: bool) {

    // The number of items we have moved
    let mut items_moved = 0;

    // Make a dry run, if specified
    if dry_run {
        for entry in source.pathbuf.as_path().read_dir().expect("Failed to read dir.") {

            // The entry path
            let entry = entry.expect("Failed to get dir entry.");
            let old_path = File::from_pathbuf(&entry.path());

            // Calculate the new path for the entry
            let new_path = target.join_string(&old_path.file_name());

            // Make sure that the path being moved is not the source or target
            if &old_path == source || &old_path == target { continue }

            // Show the output of the dry run
            println!("{}", DryRunMessage { from_file: old_path, to_file: new_path }.to_string());
        }
        return;
    }

    // Count the number of items we are going to move
    let mut items_to_move = 0;
    for entry in source.pathbuf.as_path().read_dir().expect("Failed to read dir") {

        // The entry path
        let entry = entry.expect("Failed to get dir entry.");
        let old_path = File::from_pathbuf(&entry.path());

        // Make sure that the path being moved is not the source or target
        if &old_path == source || &old_path == target { continue }

        items_to_move += 1;
    }

    // The progress bar
    let progress_bar = ProgressBar {
        completed_message: String::from("Completed."),
        message: String::from("Extracting..."),
        total: items_to_move,
    };

    // Move each entry (file or directory) in the directory
    for entry in source.pathbuf.as_path().read_dir().expect("Failed to read dir.") {

        // The entry path
        let entry = entry.expect("Failed to get dir entry.");
        let old_path = File::from_pathbuf(&entry.path());

        // Calculate the new path for the entry
        let new_path = target.join_string(&old_path.file_name());

        // Make sure that the path being moved is not the source or target
        if &old_path == source || &old_path == target { continue }

        // Move the path
        // println!("Moving {} to {}...", &old_path.to_string(), &new_path.to_string());
        fs::rename(old_path.to_string(), new_path.to_string())
            .expect(
                &error_messages::PathMoveFailedError {
                    source: &old_path,
                    target: &new_path,
                }.to_string()
            );
        
        // Add to the count of items moved
        items_moved += 1;

        // Show the progress
        progress_bar.set_progress(items_moved);
    }
    // Show success status
    progress_bar.complete();
    println!("Successfully moved {} items to {}.", items_moved, target.to_string());
}