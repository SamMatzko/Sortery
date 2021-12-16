use std::path::{Path, PathBuf};

// The struct used in all the cross-function path functionality
#[derive(Debug)]
#[derive(PartialEq)]
pub struct File {
    pub pathbuf: PathBuf,
}
impl File {
    pub fn copy(&self) -> File {
        // Return an instance of File with the same path as ours
        File { pathbuf: PathBuf::from(&self.pathbuf) }
    }
    pub fn from_path(path: &Path) -> File {
        // Return a new instance of File, with PATH as the path
        File { pathbuf: path.to_path_buf() }
    }

    pub fn from_pathbuf(path: &PathBuf) -> File {
        // Return a new instance of File, with PATH as the path
        File { pathbuf: path.to_path_buf() }
    }

    pub fn exists(&self) -> bool {
        // Return true if this path exists
        if self.pathbuf.exists() {
            return true;
        } else {
            return false;
        }
    }

    pub fn extension(&self) -> String {
        // Return a string representing the extension of the path
        self.pathbuf.as_path().extension()
            .expect("Failed to get data.")
            .to_str()
            .expect("Failed to convert to str")
            .to_string()
    }

    pub fn file_name(&self) -> String {
        // Return the file name of our path
        self.pathbuf.file_name().expect("Failed to get file name.")
            .to_str()
            .expect("Failed to convert to str")
            .to_string()
    }

    pub fn file_stem(&self) -> String {
        // Return a string representing the file stem of the path
        self.pathbuf.as_path().file_stem()
            .expect("Failed to get data.")
            .to_str()
            .expect("Failed to convert to str")
            .to_string()
    }

    pub fn join(&self, path: &Path) -> File {
        // Return the joining of this path and PATH
        File { pathbuf: self.pathbuf.join(path) }
    }

    pub fn join_string(&self, path: &String) -> File {
        // Return the joining of this path and PATH
        File { pathbuf: self.pathbuf.join(Path::new(path)) }
    }

    pub fn new(from: &str) -> File {
        // Return a new instance of File from FROM
        File { pathbuf: PathBuf::from(from) }
    }

    pub fn to_path_buf(&self) -> PathBuf {
        // Return an instance of PathBuf representing our path
        PathBuf::from(&self.pathbuf)
    }
    
    pub fn to_string(&self) -> String {
        // Return a string representing the path
        self.pathbuf.display().to_string()
    }
}