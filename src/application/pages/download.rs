use leptos::prelude::*;

#[component]
pub fn DownloadPage() -> impl IntoView {
  let (downloads, set_downloads) = create_signal(Vec::<String>::new());

  view! {
    <main class="flex-1 flex items-center justify-center p-8">
      <div class="w-full max-w-2xl">
        <div class="text-center mb-12">
          <h2 class="text-4xl font-bold text-gray-900 mb-3">Download YouTube Videos</h2>
          <p class="text-gray-600 text-lg">Paste your YouTube URL below and click download</p>
        </div>

        <div class="bg-white rounded-2xl shadow-lg p-8">
          <div class="space-y-6">
            <div>
              <label for="youtube-url" class="block text-sm font-medium text-gray-700 mb-2">
                YouTube URL
              </label>
              <input
                type="text"
                id="youtube-url"
                placeholder="https://youtu.be/dQw4w9WgXcQ"
                class="w-full px-4 py-3 border border-gray-300 rounded-lg focus:ring-2 focus:ring-red-500 focus:border-transparent outline-none transition-all"
              />
            </div>

            <button class="w-full bg-red-600 hover:bg-red-700 text-white font-semibold py-4 px-6 rounded-lg transition-colors flex items-center justify-center gap-2 shadow-md hover:shadow-lg">
              <i class="ri-download-line text-xl"></i>
              <span>Download Video</span>
            </button>
          </div>

          <div class="mt-6 p-4 bg-blue-50 rounded-lg">
            <p class="text-sm text-blue-800">
              <strong>Tip:</strong> Supports various video qualities and formats. The download will start automatically once processed.
            </p>
          </div>
        </div>
      </div>
    </main>
  }
}