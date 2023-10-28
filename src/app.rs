mod components;
mod layouts;

use layouts::base::BaseLayout;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Link
            rel="shortcut icon"
            type_="image/ico"
            href="/img/favicon-32x32.png"
        />
        <Meta charset="utf-8"/>
        <Meta name="viewport" content="width=device-width"/>
        <Script src="/htmx.js"/>
        <Stylesheet id="leptos" href="/pkg/product-feedback.css"/>
        <Title text="Product Feedback"/>
        <body class="grid overflow-hidden grid-cols-1 w-full h-full">
            <Router>
                <Routes>
                    <Route path="" view=move || view! { <Home/> }/>
                </Routes>
            </Router>
        </body>
    }
}

#[component]
fn Home() -> impl IntoView {
    view! {
        <BaseLayout>
            <div></div>
        </BaseLayout>
    }
}

