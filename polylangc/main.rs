mod linux;
mod windows;

use std::path::PathBuf;
use clap::Parser;
use uuid::Uuid;
use serde::Serialize;
use std::fs::{self, create_dir_all};
use chrono::Utc;

/// PolyLang Compiler: Converts a .poly file to a CaL ABP WorldPack
#[derive(Parser)]
struct Args {
    /// Path to .poly source file
    input: PathBuf,

    /// Worldpack name
    #[arg(short, long, default_value = "default")]
    name: String,
}

#[cfg(target_os = "linux")]
use crate::linux::get_output_path;
#[cfg(target_os = "windows")]
use crate::windows::get_output_path;

// [ ... include data structures here as in previous response ... ]

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let output = get_output_path(&args.name);
    
    println!("📦 Outputting to: {}", output.display());

    // ... everything else stays the same ...
    
    // (You can copy all logic from the previous `main.rs` here and just replace `args.output` with `output`)
    
    Ok(())
}
