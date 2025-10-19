use leptos::{ev::SubmitEvent, prelude::*, reactive::spawn_local};
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

#[wasm_bindgen]
extern "C" {
	#[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
	async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[component]
pub fn DownloadPage() -> impl IntoView {

  let (links_entered, set_links_entered) = signal(String::from("lala"));

  // Submit youtube link
  let on_submit = move |ev: SubmitEvent| {
    ev.prevent_default();

    // Creating my link of youtube links
    let links = links_entered
      .get()
      .split("\n")
      .map(|s| s.trim().to_string())
      .collect::<Vec<String>>();
    
    spawn_local(async move {
      let args = serde_wasm_bindgen::to_value(&serde_json::json!({
        "links": links
      })).unwrap_or(JsValue::NULL);
      invoke("download_youtube_videos", args).await;
    });
  };

  view! {
    <main class="flex-1 flex items-center justify-center p-8">
      <div class="w-full max-w-2xl">
        <div class="text-center mb-8">
          <h1 class="text-4xl font-bold text-gray-800 mb-3">"Download YouTube Videos"</h1>
          <p class="text-gray-600">"Paste one or more YouTube URLs below (one per line)"</p>
        </div>
        
        <form on:submit=on_submit class="bg-white rounded-2xl shadow-xl p-8">
          <textarea
            name="youtube_links"
            placeholder="https://youtu.be/dQw4w9WgXcQ&#10\nhttps://youtu.be/example123&#10\nhttps://youtu.be/another456"
            class="w-full px-6 py-4 text-lg border-2 border-gray-300 rounded-xl focus:outline-none focus:border-red-500 focus:ring-2 focus:ring-red-200 transition-all resize-none"
            rows="8"
            prop:value=links_entered
            on:input:target=move |ev| set_links_entered.set(ev.target().value())
          ></textarea>
          
          <button class="w-full mt-6 bg-red-600 hover:bg-red-700 text-white font-semibold py-4 px-6 rounded-xl transition-colors flex items-center justify-center space-x-2 text-lg shadow-lg hover:shadow-xl">
            <i class="ri-download-line text-2xl"></i>
            <span>"Download Videos"</span>
          </button>
          
          <p class="text-sm text-gray-500 text-center mt-4">
            "Supports YouTube, YouTube Shorts, and YouTube Music links"
          </p>
        </form>
      </div>
    </main>
  }
}