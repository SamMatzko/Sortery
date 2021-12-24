use crate::structs::File;
use super::messages::{error_messages, ProgressBar};
use std::fs;

// Various sorting algorithms
pub mod sort {

    // use super::super::messages::error_messages;
    use chrono::{DateTime, TimeZone, Utc, Local};
    use crate::{error_messages, messages::ProgressBar, structs::{ConfigData, File}};
    use std::{fs, path::Path, time::UNIX_EPOCH};
    use walkdir::WalkDir;

    #[cfg(test)]
    mod tests {
        // Tests for tools. Each test function is named after the function in
        // tools it tests, with the test_ prefix.

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

    fn get_epoch_secs_access(path: &File) -> i64 {
        // Return the access date and time as the number of seconds since the
        // UNIX epoch.
        let ctime_system = path.pathbuf.metadata().unwrap().accessed().expect("Failed to get atime");
        let secs: i64 = ctime_system.duration_since(UNIX_EPOCH).unwrap().as_secs() as i64;

        secs
    }

    fn get_epoch_secs_creation(path: &File) -> i64 {
        // Return the creation date and time as the number of seconds since the
        // UNIX epoch.
        let ctime_system = path.pathbuf.metadata().unwrap().created().expect("Failed to get ctime");
        let secs: i64 = ctime_system.duration_since(UNIX_EPOCH).unwrap().as_secs() as i64;

        secs
    }

    fn get_epoch_secs_modified(path: &File) -> i64 {
        // Return the modification date and time as the number of seconds since the
        // UNIX epoch.
        let ctime_system = path.pathbuf.metadata().unwrap().modified().expect("Failed to get mtime");
        let secs: i64 = ctime_system.duration_since(UNIX_EPOCH).unwrap().as_secs() as i64;

        secs
    }

    fn get_datetime(path: &File, date_type: &str) -> DateTime<Local> {
        // Return a DateTime instance of the creation, modification, or access
        // time of PATH according to DATE_TYPE.
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

    fn get_new_date_path(
        target: &File,
        old_file: &File,
        date_format: &str,
        date_type: &str,
        preserve_name: bool) -> File {
        // Move FILE into a set of directories in yyyy/mm/ format according to its
        // creation time. Create any required directories that don't already exist.
        // Also rename the file according to its creation date.
        
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

    fn get_sequential_name(path: &File, vec: &Vec<File>) -> File {
        /*
        Return a PathBuf representing the renamed version of PATH. This function is
        called only if PATH already exists, but can't/shouldn't be replaced. The
        naming system: if `/path/to/file` already exists, return `/path/to/file_2`.
        If `/path/to/file_2` already exists, return `/path/to/file_3`, etc.
        */

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

    fn get_sorting_results(
        source: &File,
        target: &File,
        date_format: &str,
        date_type: &str,
        preserve_name: &bool,
        exclude_type: (&str, bool),
        only_type: (&str, bool)) -> (usize, Vec<File>, Vec<File>) {
        // The main sorting algorithm; this checks files for validity and shows
        // the progress bar.

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

    fn is_sortable(path: &File, exclude_type: &(&str, bool), only_type: &(&str, bool)) -> bool {
        /*
        Return true if:
        1) PATH's type is in only_type.0 and only_type.1 is true
        2) PATH's type is not in exclude_type.0, and only_type.1 is false
        */

        if is_type(path, only_type.0) && only_type.1 {
            return true;
        } else if !is_type(path, exclude_type.0) && !only_type.1 {
            return true;
        } else {
            return false;
        }
    }

    fn is_type(path: &File, types: &str) -> bool {
        // Return true if PATH's type is one of the types in TYPES.
        let mut to_return: bool = false;
        for t in types.split("-") {
            if path.extension() == t {
                to_return = true;
            }
        }
        to_return
    }

    pub fn sort_dry_run(
        source: &File,
        target: &File,
        date_format: &str,
        date_type: &str,
        preserve_name: &bool,
        exclude_type: (&str, bool),
        only_type: (&str, bool)) {
        // Show only the output of the intended sort, without acutally sorting

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
            println!("Sorting {}\tto {}.", results.1[i].to_string(), results.2[i].to_string());
        }
    }

    pub fn sort(
        source: &File,
        target: &File,
        date_format: &str,
        date_type: &str,
        preserve_name: &bool,
        exclude_type: (&str, bool),
        only_type: (&str, bool),
        dry_run: bool) {
        // Sort the files using the sorting algorithms

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

    pub fn sort_from_json(json: String, source: File, target: File, dry_run: bool) {
        // Sort according to configuration data in json string JSON

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

pub fn extract(source: &File, target: &File) {
    // Extract the contents of SOURCE to TARGET

    // The number of items we have moved
    let mut items_moved = 0;

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