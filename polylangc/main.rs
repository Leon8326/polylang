mod linux;
mod windows;

use clap::Parser;
use serde::Serialize;
use std::fs::{self, create_dir_all, File};
use std::io::Write;
use std::path::PathBuf;
use uuid::Uuid;
use chrono::Utc;

/// PolyLang Compiler: Converts a .poly file to a CaL ABP WorldPack
#[derive(Parser)]
struct Args {
    /// Path to .poly source file (not used in this example yet)
    input: PathBuf,

    /// Worldpack name
    #[arg(short, long, default_value = "default")]
    name: String,
}

#[cfg(target_os = "linux")]
use crate::linux::get_output_path;
#[cfg(target_os = "windows")]
use crate::windows::get_output_path;

// ----------- JSON Structures -----------

#[derive(Serialize)]
struct WorldPackSettings {
    worldPackGUID: String,
    worldPackName: String,
    startingWorld: String,
    speedrunnable: bool,
    worldProgression: Vec<String>,
    timestamp: i64,
    featureImage: String,
    creator: String,
    steamWorkshopID: String,
    contentVersion: u8,
}

#[derive(Serialize)]
struct WorldSettings {
    worldGUID: String,
    worldName: String,
    startingRoom: String,
    roomProgression: Vec<String>,
}

#[derive(Serialize)]
struct RoomSettings {
    roomGUID: String,
    roomName: String,
    roomDisplayTitle: String,
    availableFriends: [u8; 3],
    companionHealthDegradationMultiplier: u8,
    themeID: u8,
    musicID: u8,
    abilityID: u8,
    blueLiquid: bool,
    redDamageFlash: bool,
    hasVoid: bool,
    voidHeight: f32,
}

// ----------- File Writing -----------

fn write_json<T: Serialize>(path: &PathBuf, data: &T) -> anyhow::Result<()> {
    let json = serde_json::to_string(data)?;
    let mut file = File::create(path)?;
    file.write_all(json.as_bytes())?;
    Ok(())
}

fn write_raw(path: &PathBuf, contents: &[&str]) -> anyhow::Result<()> {
    let mut file = File::create(path)?;
    for line in contents {
        writeln!(file, "{line}")?;
    }
    Ok(())
}

// ----------- Main -----------

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let base_path = get_output_path(&args.name);

    // Folder paths
    let worldpack_dir = &base_path;
    let world_dir = base_path.join("World 1");
    let room_dir = world_dir.join("Room 1");

    // Generate GUIDs
    let worldpack_guid = Uuid::new_v4().to_string();
    let world_guid = Uuid::new_v4().to_string();
    let room_guid = Uuid::new_v4().to_string();

    // Create directories
    create_dir_all(&room_dir)?;

    // Write polymap.data
    let polymap_path = room_dir.join("polymap.data");
    write_raw(&polymap_path, &[
        r#"{"guid":"","id":2,"position":{"x":0.0,"y":0.0},"rotation":{"x":0.0,"y":0.0,"z":0.0,"w":1.0},"additionalComponentCount":1}"#,
        "PolyMap.Shape",
        r#"{"pointData":[{"position":{"x":-4.0,"y":-5.0},"curved":false},{"position":{"x":-4.0,"y":-1.0},"curved":false},{"position":{"x":20.0,"y":-1.0},"curved":false},{"position":{"x":20.0,"y":-5.0},"curved":false}]}"#,
        r#"{"guid":"","id":3,"position":{"x":0.0,"y":0.0},"rotation":{"x":0.0,"y":0.0,"z":0.0,"w":1.0},"additionalComponentCount":0}"#,
        r#"{"guid":"","id":4,"position":{"x":18.0,"y":0.5},"rotation":{"x":0.0,"y":0.0,"z":0.0,"w":1.0},"additionalComponentCount":1}"#,
        "Door",
        r#"{"roomToLoad":"","endPack":true}"#,
        r#"{"guid":"","id":1,"position":{"x":0.0,"y":0.0},"rotation":{"x":0.0,"y":0.0,"z":0.0,"w":1.0},"additionalComponentCount":2}"#,
        "PolyMap.Shape",
        r#"{"pointData":[{"position":{"x":5.0,"y":-1.0},"curved":false},{"position":{"x":11.0,"y":-1.0},"curved":false}]}"#,
        "TogglePlatform",
        r#"{"active":false,"onCollision":false,"onCollisionHiddenTime":10,"inverted":false,"needsGreen":false,"frequency":1.0,"turbo":false}"#,
    ])?;

    // Write settings.data files
    write_json(&room_dir.join("settings.data"), &RoomSettings {
        roomGUID: room_guid.clone(),
        roomName: "Room 1".into(),
        roomDisplayTitle: "".into(),
        availableFriends: [1, 1, 1],
        companionHealthDegradationMultiplier: 0,
        themeID: 0,
        musicID: 0,
        abilityID: 0,
        blueLiquid: false,
        redDamageFlash: false,
        hasVoid: true,
        voidHeight: -10.0,
    })?;

    write_json(&world_dir.join("settings.data"), &WorldSettings {
        worldGUID: world_guid.clone(),
        worldName: "World 1".into(),
        startingRoom: "Room 1".into(),
        roomProgression: vec![room_guid],
    })?;

    write_json(&worldpack_dir.join("settings.data"), &WorldPackSettings {
        worldPackGUID: worldpack_guid.clone(),
        worldPackName: args.name.clone(),
        startingWorld: "World 1".into(),
        speedrunnable: false,
        worldProgression: vec![world_guid],
        timestamp: Utc::now().timestamp(),
        featureImage: "".into(),
        creator: "Leon_to_ziomal".into(),
        steamWorkshopID: "".into(),
        contentVersion: 0,
    })?;

    println!("✅ PolyLang compiled to {}", worldpack_dir.display());
    Ok(())
}
