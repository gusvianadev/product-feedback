use leptos::*;

#[component]
pub fn Filter() -> impl IntoView {
    struct Filter<'a> {
        name: &'a str,
    }

    let filters = || {
        [
            Filter {
                name: "Most Upvotes",
            },
            Filter {
                name: "Least Upvotes",
            },
            Filter {
                name: "Most Comments",
            },
            Filter {
                name: "Least Comments",
            },
        ]
    };

    view! {
        <div class="flex flex-row flex-wrap py-4 px-4 w-full text-sm sm:flex-nowrap sm:justify-start bg-gray-dark">
            <nav
                class="flex flex-row justify-between items-center mx-auto w-full"
                aria-label="Global"
            >
                <div class="flex flex-col flex-none">
                    <div class="inline-flex relative hs-dropdown">
                        <button
                            id="hs-dropdown-with-dividers"
                            type="button"
                            class="inline-flex gap-2 justify-center items-center py-3 text-sm font-medium text-white align-middle bg-transparent rounded-md transition-all hs-dropdown-toggle after:content-[''] hover:bg-gray-50"
                        >
                            <span class="text-sm">
                                Sort by: {" "}
                                <span id="filter-text" class="text-bold"></span>
                            </span>
                            <svg
                                width="10"
                                height="7"
                                xmlns="http://www.w3.org/2000/svg"
                                class="hs-dropdown-open:rotate-180"
                            >
                                <path
                                    d="M1 1l4 4 4-4"
                                    stroke="currentColor"
                                    stroke-width="2"
                                    fill="none"
                                    fill-rule="evenodd"
                                ></path>
                            </svg>
                        </button>
                        <div
                            class="hs-dropdown-menu duration mt-2 hidden min-w-[15rem] divide-y divide-gray-200 rounded-lg bg-white p-2 opacity-0 shadow-md transition-[opacity,margin] hs-dropdown-open:opacity-100"
                            aria-labelledby="hs-dropdown-with-dividers"
                        >
                            <For each=filters key=|filter| filter.name let:filter>
                                <div class="py-2 first:pt-0 last:pb-0">
                                    <span
                                        _="on click set #filter-text.innerHTML to my.innerHTML"
                                        class="flex gap-x-3.5 items-center py-2 px-3 text-sm text-gray-800 rounded-md cursor-pointer hover:bg-gray-100 filter-selector"
                                        safe
                                    >
                                        {filter.name}
                                    </span>
                                </div>
                            </For>
                        </div>
                    </div>
                </div>
                <div class="flex-none md:hidden">
                    <button>+ Add Feedback</button>
                </div>
            </nav>
        </div>
    }
}

