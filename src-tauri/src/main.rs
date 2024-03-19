// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::process::{ Command, Stdio };
use users::get_current_uid;

mod commands;
use commands::args;

fn main()
{
    let user = get_current_uid();

    match user
    {
        0 => {
            tauri::Builder::default()
                .invoke_handler(tauri::generate_handler![args::install_package, args::uninstall_package, args::search_package, args::upgrade_system, installed])
                .run(tauri::generate_context!())
                .expect("error while running tauri application");
        },
        _ => println!("Run with sudo privilege!")
    }
}

#[tauri::command]
fn installed(packages_name: Vec<String>) -> Vec<String>
{
    let mut result = Vec::<String>::new();

    for i in packages_name
    {
        let pacman = Command::new("pacman")
            .arg("-Qq")
            .arg(&i)
            .stdout(Stdio::null())
            .status()
            .unwrap();

        if pacman.success()
        {
            result.push(i);
        }
    }

    result
}
