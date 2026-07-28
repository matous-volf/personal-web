use crate::{components::Localized, route::Route};
use dioxus::prelude::*;

#[component]
pub(crate) fn NotFound(route: Vec<String>) -> Element {
    rsx! {
        div {
            class: "grow w-full max-w-200 flex flex-col justify-center items-center gap-8",
            h1 {
                class: "text-9xl text-slate-500",
                "404"
            }
            p {
                Localized {
                    czech: rsx! { "Tady není nic k vidění." },
                    english: rsx! { "Nothing to see here." },
                }
            }
            Link {
                class: "px-5 py-3 bg-slate-300 text-slate-800 hover:bg-slate-500 hover: text-slate-950 font-bold rounded-lg duration-150",
                to: Route::Home {},
                Localized {
                    czech: rsx! { "Rychle pryč" },
                    english: rsx! { "Get me out of here" },
                }
            }
        }
    }
}
