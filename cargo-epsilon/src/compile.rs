use std::{fs, path::{Path, PathBuf}};

use anyhow::{Context, Result};

use crate::{epsilon::Project, shells::winui3::Solution};

fn verify_epsilon_dir(platform: &str) -> PathBuf {
    let path = Path::new("./.epsilon").join(platform);
    
    if !path.is_dir() {
        eprintln!("Error: Cannot find epsilon platform shell for {}", platform);
        std::process::exit(1);
    }
    
    path
}

pub(crate) fn run_windows(arch: &str) -> Result<()> {
    let dot_epsilon = verify_epsilon_dir("winui3");
    let slnx = dot_epsilon.join("winui3.slnx");
    let slnx_content = fs::read_to_string(slnx)
        .context("Failed to read //shells/winui3/winui3.slnx")?;

    let solution: Solution = quick_xml::de::from_str(&slnx_content)?;
    let project_path = solution.projects.iter().find_map(|project| {
        if project.path.contains("EpsilonShell") {
            Some(dot_epsilon.join("winui3").push(&project.path)) // TODO: Make the path output safer
        } else {
            None
        }
    });

    println!("{:?}", project_path);

    Ok(())
}