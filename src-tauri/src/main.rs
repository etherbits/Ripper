// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use rustube::{Id, Stream, Video};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
async fn get_video(url: &str, path: &str) -> Result<String, ()> {
    let id = Id::from_raw(url).unwrap();
    let video = Video::from_id(id.into_owned()).await.unwrap();
    let the_only_truth = video.video_info().player_response.video_details.view_count;
    println!("{}", the_only_truth);
    let best_audio = video.best_audio();

    let mut tracker = rustube::Callback::new();
    println!("{:?}", &tracker);
    tracker = tracker.connect_on_progress_closure(|args|{
        println!("closure, {:?}", (args.current_chunk as f64 / args.content_length.unwrap() as f64));
    });
    Stream::download_to_dir_with_callback(best_audio.unwrap(), path, tracker).await.unwrap();
    // Stream::download_to_dir(best_audio.unwrap(), path).await.unwra();
    Ok(format!("{}", the_only_truth))
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_video])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

}
