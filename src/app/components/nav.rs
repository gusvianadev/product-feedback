use leptos::*;

#[component]
pub fn Nav() -> impl IntoView {
    view! {
        <header class="flex flex-row flex-wrap py-4 px-4 w-full text-sm sm:flex-nowrap sm:justify-start main-nav">
            <span
                class="flex flex-row justify-between items-center mx-auto w-full"
                aria-label="Global"
            >
                <div class="flex flex-col flex-none">
                    <h1 class="text-base text-white">Frontend Mentor</h1>
                    <h3 class="text-sm text-white text-opacity-75">
                        Feedback Board
                    </h3>
                </div>
                <div class="flex-none md:hidden">
                    <button
                        type="button"
                        class="text-gray-500 hover:text-gray-600"
                        data-hs-overlay="#docs-sidebar"
                        aria-controls="docs-sidebar"
                        aria-label="Toggle navigation"
                    >
                        <span class="sr-only">Menu</span>
                        <img src="/img/icon-hamburger.svg"/>
                    </button>
                    <div
                        id="docs-sidebar"
                        class="hidden overflow-y-auto fixed top-0 right-0 bottom-0 px-4 pt-7 pb-10 w-64 border-r border-gray-200 transition-all duration-300 transform translate-x-full lg:block lg:bottom-0 lg:right-auto lg:translate-x-0 hs-overlay scrollbar-y z-[60] bg-base hs-overlay-open:translate-x-0"
                    >
                        <div class="flex flex-col gap-4">
                            <Categories/>
                            <Roadmap/>
                        </div>
                    </div>
                </div>
            </span>
        </header>
    }
}

#[component]
fn Categories() -> impl IntoView {
    #[derive(Copy, Clone, Debug, PartialEq, Eq)]
    struct Category<'a> {
        name: &'a str,
        selected: bool,
    }

    let categories = || {
        [
            Category {
                name: "All",
                selected: true,
            },
            Category {
                name: "UI",
                selected: false,
            },
            Category {
                name: "UX",
                selected: false,
            },
            Category {
                name: "Enhancement",
                selected: false,
            },
            Category {
                name: "Bug",
                selected: false,
            },
            Category {
                name: "Feature",
                selected: false,
            },
        ]
    };

    view! {
        <div class="flex flex-wrap gap-3 p-4 bg-white rounded-xl border shadow-sm">
            <For each=categories key=|category| category.name let:category>
                <span class=format!(
                    "inline-flex items-center gap-1.5 rounded-md font-black {} px-3 py-1.5 text-xs",
                    if category.selected {
                        "bg-secondary text-white"
                    } else {
                        "bg-secondary-light text-secondary"
                    },
                )>{category.name}</span>
            </For>
        </div>
    }
}

#[component]
fn Roadmap() -> impl IntoView {
    struct Roadmap<'a> {
        name: &'a str,
        quantity: u16,
        color: &'a str,
    }

    let roadmaps = || {
        [
            Roadmap {
                name: "Planned",
                quantity: 2,
                color: "bg-orange-400",
            },
            Roadmap {
                name: "In-Progress",
                quantity: 3,
                color: "bg-purple-400",
            },
            Roadmap {
                name: "Live",
                quantity: 1,
                color: "bg-blue-300",
            },
        ]
    };

    view! {
        <div class="flex flex-col gap-4 p-4 bg-white rounded-xl border shadow-sm">
            <div class="flex justify-between w-full">
                <h3 class="font-bold text-gray-medium">Roadmap</h3>
                <a
                    href="#"
                    class="font-bold border-b border-b-secondary text-secondary"
                >
                    View
                </a>
            </div>
            <For each=roadmaps key=|roadmap| roadmap.name let:roadmap>
                <div class="flex gap-3 justify-between items-center">
                    <span class=format!(
                        "inline-block h-1.5 w-1.5 rounded-full {}",
                        roadmap.color,
                    )></span>
                    <span class="mr-auto text-base text-gray-light" safe>
                        {roadmap.name}
                    </span>
                    <span class="font-bold text-[16px] text-gray-light">
                        {roadmap.quantity}
                    </span>
                </div>
            </For>
        </div>
    }
}

