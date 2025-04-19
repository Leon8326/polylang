use std::path::PathBuf;

pub fn get_output_path(worldpack_name: &str) -> PathBuf {
    let home = std::env::var("HOME").expect("HOME not set");
    PathBuf::from(home)
        .join(".config")
        .join("unity3d")
        .join("Last Quarter Studios")
        .join("Cats are Liquid - A Better Place")
        .join("Custom")
        .join(worldpack_name)
}
