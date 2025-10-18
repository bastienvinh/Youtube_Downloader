use leptos::{ prelude::*};
use crate::application::layout::aside::Aside;

#[component]
pub fn Layout() -> impl IntoView {
  view! {
    <div class="flex min-h-screen">
      <Aside />
    </div>
  }
}