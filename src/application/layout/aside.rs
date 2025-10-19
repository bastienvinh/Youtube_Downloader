use leptos::prelude::*;

use crate::components::aside::menu_item::MenuItem;

#[component]
pub fn Aside() -> impl IntoView {
  view! {
    <aside class="w-75 bg-white border-r border-gray-200 flex flex-col">
      <div class="p-6 border-b border-gray-200">
        <div class="flex items-center gap-2">
          <i class="ri-youtube-fill text-3xl text-red-600"></i>
          <h1 class="text-xl font-bold text-gray-900">"YT Downloader"</h1>
        </div>
      </div>

      <nav class="flex-1 p-4">
        <ul class="space-y-2">
          <MenuItem href="/".to_string() icon_name="ri-download-line".to_string() content="Download".to_string() />
          <MenuItem href="/history".to_string() icon_name="ri-history-line".to_string() content="History".to_string() />
          <MenuItem href="/saved".to_string() icon_name="ri-star-line".to_string() content="Saved".to_string() />
          <MenuItem href="/settings".to_string() icon_name="ri-settings-3-line".to_string() content="Settings".to_string() />
        </ul>
      </nav>

      <div class="p-4 border-t border-gray-200">
        <p class="text-xs text-gray-500 text-center">"© 2025 YT Downloader"</p>
      </div>
    </aside>
  }
}