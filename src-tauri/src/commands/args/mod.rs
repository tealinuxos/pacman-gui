use super::pacman::{ pacman, pacman_with_output };
use super::utils::split_and_filter;

#[tauri::command]
pub async fn search_package(package_name: String) -> Vec<String>
{
    let args = format!("-Ssq,^{}", package_name);
    let output = pacman_with_output(args).await;

    let result = split_and_filter(output, "\n", "");

    result
}

#[tauri::command]
pub async fn install_package(package_name: String) -> bool
{
    let args = format!("-S,--noconfirm,{}", package_name);
    let is_success = pacman(args);

    is_success.await
}

#[tauri::command]
pub async fn upgrade_system() -> bool
{
    let args = format!("-Syu,--noconfirm");
    let is_success = pacman(args);

    is_success.await
}
