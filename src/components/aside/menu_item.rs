use leptos::prelude::*;
use leptos_router::components::A;
use leptos_router::hooks::use_location;

#[component]
pub fn MenuItem(
  #[prop()]
  href: String,
  #[prop(default = "ri-download-line".to_string())]
  icon_name: String,
  #[prop()]
  content: String
) -> impl IntoView {

  // Retrieve the current location
  let location = use_location();
  let href_copy = href.clone();
  let is_active = move || href_copy == location.pathname.get();

  // Is active or not
  let class_link = move || if is_active() {
    "bg-red-600 text-white".to_string()
  } else {
    "".to_string()
  };

  view! {
    <li>
      <A href=href attr:class=move || format!("flex items-center gap-3 px-4 py-3 rounded-lg transition-colors {}", class_link())>
        <i class=format!("{} text-xl", icon_name)></i>
        <span class="font-medium">{content}</span>
      </A>
    </li>
  }
}