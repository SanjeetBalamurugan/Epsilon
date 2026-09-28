use std::{default, io::{self, Write}, process::Command};

use anyhow::{bail, Result, Context};

pub fn sh(cmd: &str, args: &[&str]) -> Result<()> {
    let status = Command::new(cmd)
        .args(args)
        .status()
        .with_context(|| format!("Failed to execute process: {}", cmd))?;
        
    if !status.success() {
        bail!("Command failed: {} {:?}", cmd, args);
    }
    
    Ok(())
}

pub fn input(message: &str, default: &str) -> String {
    let mut input = String::new();

    print!("{}", message);
    io::stdout().flush().expect("Failed to flush stdout");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let out = if input.trim().is_empty() {
        default.to_string()
    } else {
        input.trim().to_string()
    };
    out
}