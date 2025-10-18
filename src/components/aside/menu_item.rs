use leptos::prelude::*;

#[component]
pub fn MenuItem(
  #[prop(default = "ri-download-line".to_string())]
  icon_name: String,
  #[prop(default = false)]
  is_active: bool,
  #[prop()]
  content: String
) -> impl IntoView {

  // Is active or not
  let classLink = if is_active {
    "bg-red-600 text-white".to_string()
  } else {
    "".to_string()
  };

  view! {
    <li>
      <a href="#" class=format!("flex items-center gap-3 px-4 py-3 rounded-lg transition-colors {}", classLink)>
        <i class=format!("{} text-xl", icon_name)></i>
        <span class="font-medium">{content}</span>
      </a>
    </li>
  }
}