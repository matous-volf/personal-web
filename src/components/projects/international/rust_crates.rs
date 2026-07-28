use dioxus::prelude::*;

use crate::components::Localized;

#[component]
pub(crate) fn RustCrates(greeting_is_finished: bool) -> Element {
    rsx! {
        div {
            class: format!("py-4 px-5 flex flex-col sm:flex-row items-center justify-between gap-8 bg-[#313030] text-right rounded-xl {}", if greeting_is_finished {
                "opacity-0 animate-fade-in-down [animation-delay:3750ms]"
            } else {
                "hidden"
            }),
            img {
                class: "h-32",
                src: asset!("/assets/images/projects/rust_crates.png")
            }
            div {
                class: "self-stretch flex flex-col items-end gap-6",
                div {
                    class: "flex flex-col gap-2",
                    h2 {
                        class: "text-3xl font-rust",
                        Localized {
                            czech: rsx! {
                                "Knihovní bedny v "
                                span {
                                    class: "text-[#f74c00]",
                                    "Rustu"
                                }
                            },
                            english: rsx! {
                                span {
                                    class: "text-[#f74c00]",
                                    "Rust"
                                }
                                " library crates"
                            },
                        }
                    }
                    p {
                        Localized {
                            czech: rsx! {
                                "Různé balíčky napsané v nejlepším jazyce pod sluncem."
                            },
                            english: rsx! {
                                "Various packages written in the best language on the planet."
                            },
                        }
                    }
                }
                Link {
                    class: "py-2 px-5 font-bold rounded-lg hover:bg-[#f74c00] border border-2 border-[#f74c00] text-[#f74c00] hover:text-[#313030] duration-150",
                    to: "https://crates.io/users/matous-volf",
                    new_tab: true,
                    Localized {
                        czech: rsx! { "Otevřít a prohlédnout" },
                        english: rsx! { "Open and look inside" },
                    }
                }
            }
        }
    }
}
