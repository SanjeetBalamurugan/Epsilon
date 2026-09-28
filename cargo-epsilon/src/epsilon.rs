use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    project: Project
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Project {
    name: String,
    version: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Version {
    pub major: i32,
    pub minor: i32,
    pub patch: i32
}

impl Config {
    pub fn new(project: Project) -> Config {
        Self { project }
    }
}

impl Project {
    pub fn new(name: String, version: Version) -> Project {
        let v = format!("{}.{}.{}", version.major, version.minor, version.patch);
        Self { name, version: v }
    }
}

impl Version {
    pub fn new(major: i32, minor: i32, patch: i32) -> Version {
        Self { major, minor, patch }
    }
}