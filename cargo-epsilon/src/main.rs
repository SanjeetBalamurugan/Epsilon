mod tools;

use std::{env, path::Path};
use fs_extra::dir::*;

fn parse_args(mut args: impl Iterator<Item = String>) {
    let cmd = match args.next().as_deref() {
        Some("epsilon") => args.next(),
        other => other.map(|s| s.to_string())
    };
    
    match cmd.as_deref() {
        Some("init") => init_project(),
        _ => {
            println!("Epsilon UI CLI");
            println!("Usage:");
            println!("  cargo epsilon init           - Initializes native shells in current project");
            println!("  cargo epsilon run <platform> - Builds and runs the app");
        }
    }
}

fn init_project() {
    println!("Initing Epsilon UI Project");
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
}

fn main() {
    let args = env::args().skip(1);
    parse_args(args);
}
