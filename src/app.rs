use crate::application::Layout;
use leptos::{ prelude::*};
use wasm_bindgen::prelude::*;

use leptos_router::components::*;
use leptos_router::path;

use crate::application::pages::DownloadPage;

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
				<ParentRoute path=path!("") view=Layout>
					<Route path=path!("") view=DownloadPage />
					<Route path=path!("download") view=DownloadPage />
					<Route path=path!("history") view=|| view! { <h1>"History Page"</h1> } />
					<Route path=path!("saved") view=|| view! { <h1>"Saved Page"</h1> } />
					<Route path=path!("settings") view=|| view! { <h1>"Settings Page"</h1> } />
				</ParentRoute>
			</Routes>
		</Router>
	}
}
