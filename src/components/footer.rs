use dioxus::prelude::*;

use crate::components::Localized;

#[component]
pub(crate) fn Footer(children: Element, class: Option<String>) -> Element {
    rsx! {
        div {
            class: "self-stretch flex flex-col items-center gap-4 sm:gap-8 text-slate-300/50",
            hr {
                class: "self-stretch mx-2 sm:mx-4 text-slate-500/50"
            }
            div {
                class: "flex flex-row gap-12",
                Link {
                    class: "underline",
                    to: "https://status.matousvolf.cz",
                    Localized {
                        czech: rsx! { "Stav služeb" },
                        english: rsx! { "Service status" },
                    }
                }
                Link {
                    class: "underline",
                    to: "https://codeberg.org/matous-volf/personal-web",
                    Localized {
                        czech: rsx! { "Zdrojový kód" },
                        english: rsx! { "Source code" },
                    }
                }
            }
        }
    }
}
