use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Title text="Product Feedback"/>
        <Stylesheet id="leptos" href="/pkg/product-feedback.css"/>
        <Link rel="shortcut icon" type_="image/ico" href="/favicon-32x32.png"/>
        <Router>
            <Routes>
                <Route path="" view=  move || view! { <Home /> }/>
            </Routes>
        </Router>
    }
}

#[component]
fn Home() -> impl IntoView {
    let files = ["a.txt", "b.txt", "c.txt"];
    let labels = files.iter().copied().map(Into::into).collect();
    let tabs = move || {
        files
            .into_iter()
            .enumerate()
            .map(|(index, filename)| {
                let content = std::fs::read_to_string(filename).unwrap();
                view! {
                    <Tab index>
                        <h2>{filename.to_string()}</h2>
                        <p>{content}</p>
                    </Tab>
                }
            })
            .collect_view()
    };

    view! {
        <main>
            <div class="bg-gradient-to-tl from-blue-800 to-blue-500 text-white font-mono flex flex-col min-h-screen">
                <div class="flex flex-row-reverse flex-wrap m-auto">
                    <Tabs labels>
                        <div>{tabs()}</div>
                    </Tabs>
                </div>
            </div>
        </main>
    }
}

#[island]
fn Tabs(labels: Vec<String>, children: Children) -> impl IntoView {
    let (selected, set_selected) = create_signal(0);
    provide_context(selected);

    let buttons = labels
        .into_iter()
        .enumerate()
        .map(|(index, label)| {
            view! {
                <button
                    class="rounded px-3 py-2 m-1 border-b-4 border-l-2 shadow-lg bg-blue-700 border-blue-800 text-white"
                    on:click=move |_| set_selected(index)
                >
                    {label}
                </button>
            }
        })
        .collect_view();

    view! {
        <div>
            <div style="display: flex; width: 100%; justify-content: space-between;">
                {buttons}
            </div>
            {children()}
        </div>
    }
}

#[island]
fn Tab(index: usize, children: Children) -> impl IntoView {
    let selected = expect_context::<ReadSignal<usize>>();

    view! {
        <div
            style:display=move || if selected() == index {
                "block"
            } else {
                "none"
            }
        >{children()}</div>
    }
}
