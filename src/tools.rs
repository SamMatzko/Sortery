use super::messages::{error_messages, ProgressBar};
use std::{fs, path::Path};

// Various sorting algorithms
pub mod sort {

    // use super::super::messages::error_messages;
    use chrono::{DateTime, TimeZone, Utc, Local};
    use std::{fs, path::{Path, PathBuf}, time::UNIX_EPOCH};
    use super::ProgressBar;
    use walkdir::WalkDir;

    fn get_epoch_secs_access(path: &Path) -> i64 {
        // Return the access date and time as the number of seconds since the
        // UNIX epoch.
        let ctime_system = path.metadata().unwrap().accessed().expect("Failed to get atime");
        let secs: i64 = ctime_system.duration_since(UNIX_EPOCH).unwrap().as_secs() as i64;

        secs
    }

    fn get_epoch_secs_creation(path: &Path) -> i64 {
        // Return the creation date and time as the number of seconds since the
        // UNIX epoch.
        let ctime_system = path.metadata().unwrap().created().expect("Failed to get ctime");
        let secs: i64 = ctime_system.duration_since(UNIX_EPOCH).unwrap().as_secs() as i64;

        secs
    }

    fn get_epoch_secs_modified(path: &Path) -> i64 {
        // Return the modification date and time as the number of seconds since the
        // UNIX epoch.
        let ctime_system = path.metadata().unwrap().modified().expect("Failed to get mtime");
        let secs: i64 = ctime_system.duration_since(UNIX_EPOCH).unwrap().as_secs() as i64;

        secs
    }

    fn get_datetime(path: &Path, date_type: &str) -> DateTime<Local> {
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

    fn get_sequential_name(path: &Path) -> PathBuf {
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
                path.file_stem().unwrap().to_str().unwrap(),
                num,
                path.extension().unwrap().to_str().unwrap()
            ));

            // Check if it exists, and if so, continue the loop
            if !new_pathbuf.exists() {
                return new_pathbuf;
            }
            num += 1;
        }
    }

    fn sort_into_date_dirs(target: &Path, old_file: &Path, date_format: &str, date_type: &str, preserve_name: bool) {
        // Move FILE into a set of directories in yyyy/mm/ format according to its
        // creation time. Create any required directories that don't already exist.
        // Also rename the file according to its creation date.
        
        // Get the time of old_file and set the names of the directories
        let ctime = get_datetime(old_file, &date_type);
        let dir = target.join(Path::new(&ctime.format("%Y/%m/").to_string()));
        
        // Check if old_file's date dir exists, and if not create it
        if !dir.exists() {
            fs::create_dir_all(&dir).expect("Failed to create dirs.");
        }

        // Preserve the original file name, if we're supposed to.
        let mut name_to_preserve = String::from("");
        if preserve_name {
            name_to_preserve = format!(
                " {}",
                old_file.file_stem().unwrap().to_str().unwrap()
            );
        }

        // Create the new file name
        let mut new_file = dir.join(Path::new(&format!(
            "{}{}.{}",
            &ctime.format(date_format).to_string(),
            name_to_preserve,
            old_file.extension().unwrap().to_str().unwrap()
        )));

        // Get the sequential file name if new_file already exists
        if new_file.exists() {
            new_file = get_sequential_name(&new_file).as_path().to_path_buf();
        }

        // Rename the file
        fs::rename(&old_file, &new_file).expect("Failed to rename file.");
    }
    
    pub fn sort(source: &Path, target: &Path, date_format: &str, date_type: &str, preserve_name: &bool) {
        /*
        Sort all the files in SOURCE (including in all subdirs) by date into TARGET
        according to the arguments. For now, this only works if SOURCE and TARGET
        are both outside each other. Does not sort directories.
        */

        // The number of items we have sorted
        let mut items_sorted = 0;

        // Count the number of items we are going to sort
        let mut items_to_sort = 0;
        for entry in WalkDir::new(source.display().to_string()) {

            let entry = entry.unwrap();
            if !entry.metadata().expect("Failed to get dir metadata").is_dir() {
                items_to_sort += 1;
            }
        }

        // The progress bar
        let progress_bar = ProgressBar {
            completed_message: String::from("Done."),
            message: String::from("Sorting..."),
            total: items_to_sort
        };
        
        // Sort the everything, excluding the directories
        for entry in WalkDir::new(source.display().to_string()) {
            
            let entry = entry.unwrap();
            if !entry.metadata().expect("Failed to get dir metadata").is_dir() {

                // The Path instance we are sorting
                let path = entry.path();
                
                // Sort the file
                sort_into_date_dirs(&target, &path, date_format, date_type, *preserve_name);
                items_sorted += 1;

                // Update the progress bar
                progress_bar.set_progress(items_sorted);
            }
        }
        progress_bar.complete();
        println!("Sucessfully sorted {} items by date into {}.", items_sorted, target.display());
    }
}

pub fn extract(source: &Path, target: &Path) {
    // Extract the contents of SOURCE to TARGET

    // The number of items we have moved
    let mut items_moved = 0;

    // Count the number of items we are going to move
    let mut items_to_move = 0;
    for entry in source.read_dir().expect("Failed to read dir") {

        // The entry path
        let entry = entry.expect("Failed to get dir entry.");
        let old_path = entry.path();

        // Make sure that the path being moved is not the source or target
        if old_path == source || old_path == target { continue }

        items_to_move += 1;
    }

    // The progress bar
    let progress_bar = ProgressBar {
        completed_message: String::from("Completed."),
        message: String::from("Extracting..."),
        total: items_to_move,
    };

    // Move each entry (file or directory) in the directory
    for entry in source.read_dir().expect("Failed to read dir.") {

        // The entry path
        let entry = entry.expect("Failed to get dir entry.");
        let old_path = entry.path();

        // Calculate the new path for the entry
        let new_path = target.join(old_path.file_name().unwrap());

        // Make sure that the path being moved is not the source or target
        if old_path == source || old_path == target { continue }

        // Move the path
        // println!("Moving {} to {}...", &old_path.display(), &new_path.display());
        fs::rename(old_path.display().to_string(), new_path.display().to_string())
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
    println!("Successfully moved {} items to {}.", items_moved, target.display());
}