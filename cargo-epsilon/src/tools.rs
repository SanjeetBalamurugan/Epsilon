use std::process::Command;

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