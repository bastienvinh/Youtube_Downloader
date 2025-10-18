use crate::application::Layout;
use leptos::{ prelude::*};
use wasm_bindgen::prelude::*;

use leptos_router::components::*;
use leptos_router::path;
// use leptos_router::hooks::use_params_map;

#[wasm_bindgen]
extern "C" {
	#[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
	async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[component]
pub fn App() -> impl IntoView {
	view! {
		<Router>
			<Routes fallback=|| "Not found.">
				<Route path=path!("") view=Layout />
				<Route path=path!("download") view=|| view! { <h1>"Download Page"</h1> } />
				<Route path=path!("history") view=|| view! { <h1>"History Page"</h1> } />
				<Route path=path!("bookmarks") view=|| view! { <h1>"Bookmarks Page"</h1> } />
				<Route path=path!("settings") view=|| view! { <h1>"Settings Page"</h1> } />
			</Routes>
		</Router>
	}
}
