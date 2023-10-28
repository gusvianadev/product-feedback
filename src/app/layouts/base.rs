use super::super::components::{filter::Filter, nav::Nav};
use leptos::*;

#[component]
pub fn BaseLayout(children: Children) -> impl IntoView {
    view! {
        <Nav/>
        <main class="flex overflow-hidden flex-col bg-base">
            <Filter/>
            <div class="flex overflow-hidden w-full h-full">{children()}</div>
        </main>
    }
}

