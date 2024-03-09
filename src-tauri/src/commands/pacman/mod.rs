use std::process::{ Command, Stdio };
use super::utils::split_by;

pub fn pacman(args: String) -> bool
{
    let vec_args = split_by(args, ",");

    let cmd = Command::new("pacman")
        .args(vec_args)
        .stdout(Stdio::null())
        .status()
        .expect("Failed to execute pacman");

    let is_success = cmd.success();

    is_success
}

#[tauri::command]
pub fn pacman_with_output(args: String) -> String
{
    let vec_args = split_by(args, ",");

    let cmd = Command::new("pacman")
        .args(vec_args)
        .output()
        .expect("Failed to execute pacman");

    let output = cmd.stdout;

    let output = String::from_utf8(output).unwrap();

    output
}
