use super::messages::{error_messages, ProgressBar};
use std::{fs, path::Path};

// Various sorting algorithms
pub mod sort {

    // use super::super::messages::error_messages;
    use chrono::{DateTime, TimeZone, Utc, Local};
    use std::{path::Path, time::UNIX_EPOCH};
    use walkdir::WalkDir;

    fn get_creation_datetime(path: &Path) -> DateTime<Local> {
        // Return the DateTime instance representing the creation time of PATH
        // in the local time zone.
        let secs = get_creation_epoch_secs(path);
        let ctime = Utc.timestamp(secs, 0);
        let mytime = Local.from_utc_datetime(&ctime.naive_utc());

        mytime
    }

    fn get_creation_epoch_secs(path: &Path) -> i64 {
        // Return the creation date and time as the number of seconds since the
        // UNIX epoch.
        let ctime_system = path.metadata().unwrap().created().expect("Failed to get mtime");
        let secs: i64 = ctime_system.duration_since(UNIX_EPOCH).unwrap().as_secs() as i64;

        secs
    }
    
    pub fn by_date(source: &Path, target: &Path) {
        // Sort all the files in SOURCE (including in all subdirs) by date into TARGET.
        // For now, this only works if SOURCE and TARGET are both outside each other.
        // Does not sort directories

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
        
        // Sort the everything, excluding the directories
        for entry in WalkDir::new(source.display().to_string()) {
            
            let entry = entry.unwrap();
            if !entry.metadata().expect("Failed to get dir metadata").is_dir() {

                // The Path instance we are sorting
                let path = entry.path();
                
                // The creation date and time
                println!("{} was created at {}.", path.display(), get_creation_datetime(&path));
            }
        }
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