#[tauri::command]
fn download_youtube_videos(links: Vec<String>) {
	println!("Downloading videos from links: {:?}", links);
	
	// You can now use async/await with bollard and tokio here
	// Example: interact with Docker API
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
	tauri::Builder::default()
		.plugin(tauri_plugin_opener::init())
		.invoke_handler(tauri::generate_handler![download_youtube_videos])
		.run(tauri::generate_context!())
		.expect("error while running tauri application");
}
