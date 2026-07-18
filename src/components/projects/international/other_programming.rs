use crate::components::Localized;
use dioxus::prelude::*;

#[component]
pub(crate) fn OtherProgramming(greeting_is_finished: bool) -> Element {
    rsx! {
        div {
            class: format!("py-4 px-5 flex flex-col items-start sm:flex-row-reverse sm:items-center justify-between gap-4 sm:gap-8 bg-violet-950 text-violet-300 font-mono rounded-xl {}", if greeting_is_finished {
                "opacity-0 animate-fade-in-down [animation-delay:3750ms]"
            } else {
                "hidden"
            }),
            p {
                class: "text-6xl sm:text-8xl text-violet-700",
                "~/"
            }
            div {
                class: "self-stretch flex flex-col items-start gap-6",
                div {
                    class: "flex flex-col gap-2",
                    h2 {
                        class: "text-3xl text-lime-400 font-bold",
                        Localized {
                            czech: rsx! { "Ostatní programování" },
                            english: rsx! { "Other programming" },
                        }
                    }
                    p {
                        Localized {
                            czech: rsx! { "Zbytek. Pokusy, hrátky, osobní potřeby a tak dále. Některé patchuju, o jiné nepečuju, na nějaké peču." },
                            english: rsx! { "The rest. Experiments, toys, personal needs and so on." },
                        }
                    }
                }
                Link {
                    class: "py-2 px-4.5 font-bold hover:bg-lime-400 border border-1 border-violet-700 hover:text-violet-950 duration-150",
                    to: "https://codeberg.org/matous-volf",
                    new_tab: true,
                    Localized {
                        czech: rsx! { "Navštívit" },
                        english: rsx! { "Visit" },
                    }
                }
            }
        }
    }
}
