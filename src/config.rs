use crate::constants::*;
use crate::structs::*;
use std::error::Error;
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;

pub fn import() -> (Config, bool) {
        let conf_path: PathBuf;
        let conf_file_was_created: bool;
        (conf_path, conf_file_was_created) = get_conf_path_from_sys();

        let configtemp = deserialize_conf(&conf_path).expect("Failed to deserialize config");

        return (Config::new(configtemp), conf_file_was_created);
}

fn get_conf_path_from_sys() -> (PathBuf, bool) {
        let mut conf_file_was_created: bool = false;
        let mut path = get_home_path().expect("Failed to get home directory");

        let conf_file_name = format!(".{}", APP_NAME);
        path.push(&conf_file_name);

        if path.exists() {
                return (path, conf_file_was_created);
        }

        create_conf_file(&path).expect("Failed to create config file");
        conf_file_was_created = true;
        return (path, conf_file_was_created);
}

fn create_conf_file(path: &PathBuf) -> std::io::Result<()> {
        let mut file = File::create_new(path)?;
        file.write_all(CONF_FILE_DEF.as_bytes())?;
        Ok(())
}

fn get_home_path() -> Result<PathBuf, Box<dyn Error>> {
        let home_path: PathBuf;
        match home::home_dir() {
                Some(path) if !path.as_os_str().is_empty() => home_path = path,
                _ => panic!("Unable to fetch location of home directory!"),
        }
        Ok(home_path)
}

fn deserialize_conf(path: &Path) -> Result<ConfigTemp, Box<dyn std::error::Error>> {
        let contents = fs::read_to_string(path)?;
        let config = toml::from_str(&contents)?;
        Ok(config)
}
