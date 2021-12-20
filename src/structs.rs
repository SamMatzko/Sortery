use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

#[cfg(test)]
mod tests {
    // Tests for the structs. Each test is named after the function and/or struct
    // it tests, prefixed with test.

    use std::{env, fs, path::Path};
    use super::ConfigData;
    
    #[test]
    fn test_configdata() {
        // Test the ConfigData struct
        
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
}

// The struct used for getting the config data from a json file
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
    pub fn from_json(json: &String) -> ConfigData {
        // Return an instance of ConfigData from the data in JSON

        let json_data: ConfigData = serde_json::from_str(json.as_str()).expect("Failed to parse json.");

        ConfigData {
            date_format: json_data.date_format,
            date_type: json_data.date_type,
            exclude_type: json_data.exclude_type,
            only_type: json_data.only_type,
            preserve_name: json_data.preserve_name
        }
    }
    // pub fn from_path(path: &String) -> ConfigData {
    //     // Return an instance of ConfigData from the data in json config file PATH
        
    //     println!("Path: {:?}", path);
    //     let json_string = fs::read_to_string(Path::new(&path)).expect("Failed to read file.");
    //     let json_data: ConfigData = serde_json::from_str(json_string.as_str()).expect("Failed to parse json.");

    //     ConfigData {
    //         source: json_data.source,
    //         target: json_data.target,
    //         date_format: json_data.date_format,
    //         date_type: json_data.date_type,
    //         exclude_type: json_data.exclude_type,
    //         only_type: json_data.only_type,
    //         preserve_name: json_data.preserve_name
    //     }
    // }
}

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
        match self.pathbuf.as_path().extension() {
            None => return String::from(""),
            s => return String::from(s.unwrap().to_str().unwrap()),
        }
    }

    pub fn file_name(&self) -> String {
        // Return the file name of our path
        match self.pathbuf.as_path().file_name() {
            None => return String::from(""),
            s => return String::from(s.unwrap().to_str().unwrap()),
        }
    }

    pub fn file_stem(&self) -> String {
        // Return a string representing the file stem of the path
        match self.pathbuf.as_path().file_stem() {
            None => return String::from(""),
            s => return String::from(s.unwrap().to_str().unwrap()),
        }
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