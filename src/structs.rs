//! Commonly-used structs that multiple modules use.

use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

#[cfg(test)]
/// Tests for the structs. Each test is named after the function and/or struct
/// it tests, prefixed with test.
mod tests {

    use std::{env, fs, path::Path};
    use super::{ConfigData, File};
    
    #[test]
    /// Test the [`ConfigData`] struct
    fn test_configdata() {
        
        // Read the json string from template.json
        let current_dir = env::current_dir().expect("Failed to get current dir.");
        let path = current_dir.join(Path::new("template.json"));
        let json_string = fs::read_to_string(path).expect("Failed to parse json.");

        // Create the ConfigData instance and test it's fields
        let config_data = ConfigData::from_json(&json_string);
        assert_eq!(config_data.date_format, String::from("%Y-%m-%d %Hh%Mm%Ss"));
        assert_eq!(config_data.date_type, String::from("m"));
        assert_eq!(config_data.exclude_type.len(), 1);
        assert_eq!(config_data.exclude_type[0], String::from("png"));
        assert_eq!(config_data.only_type.len(), 2);
        assert_eq!(config_data.only_type[0], String::from("json"));
        assert_eq!(config_data.only_type[1], String::from("py"));
        assert_eq!(config_data.preserve_name, false);
    }

    #[test]
    /// Test the [`File`] struct
    fn test_file() {

        // The variables used for testing
        let path = Path::new("my_file.txt");
        let joined_path = Path::new("my_file.txt/my_file.txt");
        let file = File::from_path(path);

        // Test the methods
        assert!(!file.exists());
        assert_eq!(file.copy(), File { pathbuf: path.to_path_buf() });
        assert_eq!(File::from_path(path), File { pathbuf: path.to_path_buf() });
        assert_eq!(File::from_pathbuf(&path.to_path_buf()), File { pathbuf: path.to_path_buf() });
        assert_eq!(file.extension(), String::from("txt"));
        assert_eq!(file.file_name(), String::from("my_file.txt"));
        assert_eq!(file.file_stem(), String::from("my_file"));
        assert_eq!(file.join(path), File { pathbuf: joined_path.to_path_buf() });
        assert_eq!(file.join_string(&String::from("my_file.txt")), File { pathbuf: joined_path.to_path_buf() });
        assert_eq!(File::new("my_file.txt"), File { pathbuf: path.to_path_buf() });
        assert_eq!(file.to_path_buf(), path.to_path_buf());
        assert_eq!(file.to_string(), String::from("my_file.txt"));
    }
}

/// The struct used for getting the config data from a json file
#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct ConfigData {
    pub date_format: String,
    pub date_type: String,
    pub exclude_type: Vec<String>,
    pub only_type: Vec<String>,
    pub preserve_name: bool
}
impl ConfigData {

    /// Return an instance of ConfigData from the data in [`String`] `json`.
    pub fn from_json(json: &String) -> ConfigData {

        let json_data: ConfigData = serde_json::from_str(json.as_str()).expect("Failed to parse json.");

        ConfigData {
            date_format: json_data.date_format,
            date_type: json_data.date_type,
            exclude_type: json_data.exclude_type,
            only_type: json_data.only_type,
            preserve_name: json_data.preserve_name
        }
    }
}

/// The struct used in all the cross-function path functionality
#[derive(Debug)]
#[derive(PartialEq)]
pub struct File {
    pub pathbuf: PathBuf,
}
impl File {

    /// Return an instance of File with the same path as ours
    pub fn copy(&self) -> File {
        File { pathbuf: PathBuf::from(&self.pathbuf) }
    }

    /// Return a new instance of [`File`], with `path` as the path.
    pub fn from_path(path: &Path) -> File {
        File { pathbuf: path.to_path_buf() }
    }

    /// Return a new instance of [`File`], with `path` as the path.
    pub fn from_pathbuf(path: &PathBuf) -> File {
        File { pathbuf: path.to_path_buf() }
    }

    /// Return [`true`] if our path exists
    pub fn exists(&self) -> bool {
        if self.pathbuf.exists() {
            return true;
        } else {
            return false;
        }
    }

    /// Return a [`String`] representing the extension of our path
    pub fn extension(&self) -> String {
        match self.pathbuf.as_path().extension() {
            None => return String::from(""),
            s => return String::from(s.unwrap().to_str().unwrap()),
        }
    }

    /// Return the file name of our path
    pub fn file_name(&self) -> String {
        match self.pathbuf.as_path().file_name() {
            None => return String::from(""),
            s => return String::from(s.unwrap().to_str().unwrap()),
        }
    }

    /// Return a [`String`] representing the file stem of our path
    pub fn file_stem(&self) -> String {
        match self.pathbuf.as_path().file_stem() {
            None => return String::from(""),
            s => return String::from(s.unwrap().to_str().unwrap()),
        }
    }

    /// Return the joining of our path and `path` as a [`File`].
    pub fn join(&self, path: &Path) -> File {
        File { pathbuf: self.pathbuf.join(path) }
    }

    /// Return the joining of our path and `path`.
    pub fn join_string(&self, path: &String) -> File {
        File { pathbuf: self.pathbuf.join(Path::new(path)) }
    }

    /// Return a new instance of [`File`] from `from`
    pub fn new(from: &str) -> File {
        File { pathbuf: PathBuf::from(from) }
    }

    /// Return an instance of [`PathBuf`] representing our path
    pub fn to_path_buf(&self) -> PathBuf {
        PathBuf::from(&self.pathbuf)
    }
    
    /// Return a [`String`] representing our path
    pub fn to_string(&self) -> String {
        self.pathbuf.display().to_string()
    }
}