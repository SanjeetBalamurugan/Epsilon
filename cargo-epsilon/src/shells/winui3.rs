use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Solution {
    #[serde(rename = "Configurations")]
    pub configurations: SolutionConfigurations,

    #[serde(rename = "Project")]
    pub projects: Vec<SolutionProject>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SolutionProject {
    #[serde(rename = "@Path")]
    pub path: String,

    #[serde(rename = "Platform")]
    pub platforms: Vec<ProjectPlatform>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectPlatform {
    #[serde(rename = "@Solution")]
    pub solution: String,
    
    #[serde(rename = "@Project")]
    pub project: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SolutionConfigurations {
    #[serde(rename = "Platform")]
    pub platforms: Vec<ConfigPlatform>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigPlatform {
    #[serde(rename = "@Name")]
    pub name: String
}