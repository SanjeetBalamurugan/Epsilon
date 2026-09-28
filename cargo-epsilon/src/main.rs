mod tools;
mod compile;
mod epsilon;
mod shells;

use std::{env, fs, path::Path};
use fs_extra::dir::*;

use crate::{compile::run_windows, epsilon::{Config, Project, Version}, tools::input};

fn parse_args(mut args: impl Iterator<Item = String>) {
    let cmd = match args.next().as_deref() {
        Some("epsilon") => args.next(),
        other => other.map(|s| s.to_string())
    };
    
    match cmd.as_deref() {
        Some("init") => init_project(),
        Some("run") => run_project(args),
        _ => {
            println!("Epsilon UI CLI");
            println!("Usage:");
            println!("  cargo epsilon init           - Initializes native shells in current project");
            println!("  cargo epsilon run <platform> <optional:arch> - Builds and runs the app");
        }
    }
}

fn run_project(mut args: impl Iterator<Item = String>) {
    let dot_epsilon = Path::new("./.epsilon");
    if !dot_epsilon.exists() || !dot_epsilon.is_dir() {
        eprintln!("Error: Cannot find .epsilon");
        std::process::exit(1);
    }

    let platform = args.next().unwrap_or_else(|| {
        eprintln!("Error: Missing platform");
        std::process::exit(1);
    });
    let arch = match args.next() {
        Some(a) => a,
        None => get_default_platform(&platform),
    };

    if platform.to_lowercase() == "windows" {
        let _ = run_windows(&arch);
    }
}

fn get_default_platform(platform: &str) -> String {
    if platform == "windows" {
        "x86_64".to_string()
    } else {
        "aarch64".to_string()
    }
}

fn init_project() {
    println!("Initing Epsilon UI Project");

    // Creating .epsilon
    let cwd = env::current_dir().unwrap();
    if cfg!(feature = "dev") && let Ok(root) = env::var("EPSILON_ROOT") {
        let epsilon_root = Path::new(&root);

        let shells = epsilon_root.join("shells");
        let dot_epsilon = cwd.join(".epsilon");
        
        let mut copy_options = CopyOptions::new();
        copy_options.content_only = true;
        copy_options.copy_inside = true;

        if let Err(_) = create_all(&dot_epsilon, false) {
            eprintln!("EpsilonUI already initialized")
        }
        match copy(shells, &dot_epsilon, &copy_options) {
            Ok(_) => println!("Successfully created .epsilon"),
            Err(err) => println!("Cannot create .epsilon: {}", err),
        }
    } else {
        todo!("Download the shell folders from github")
    }

    // create Epsilon.toml
    let config_toml = Path::new(&cwd).join("Epsilon.toml");
    let project_name = input("Project Name: ", "EpsilonProject");

    // getting all input
    let version = Version::new(0, 1, 0);
    let project = Project::new(project_name, version);

    let config = Config::new(project);
    let toml_out = toml::to_string(&config);
    if let Ok(contents) = toml_out {
        let _ = fs::write(config_toml, contents);
    }
}

fn main() {
    let args = env::args().skip(1);
    parse_args(args);
}
