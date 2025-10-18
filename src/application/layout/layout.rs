use leptos::{ prelude::*};
use leptos_router::components::Outlet;
use crate::application::layout::aside::Aside;

#[component]
pub fn Layout() -> impl IntoView {
  view! {
    <div class="flex min-h-screen">
      <Aside />
      <div class="flex-1 bg-gray-50 p-6">
        <Outlet />
      </div>
    </div>
  }
}