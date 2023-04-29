// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use rustube::{Id, VideoFetcher};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
async fn get_video(url: &str) -> Result<String, ()> {
   let id = Id::from_raw(url).unwrap();
    let descrambler = VideoFetcher::from_id(id.into_owned()).unwrap()
       .fetch()
       .await.unwrap();

    let video_info = descrambler.video_info();
    let the_only_truth = &video_info.player_response.video_details.view_count;
    println!("{}", the_only_truth);

    Ok(format!("{}", the_only_truth))

}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_video])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
