use dioxus::prelude::*;

use crate::components::Localized;

#[component]
pub(crate) fn AlexAndDerMurder(greeting_is_finished: bool) -> Element {
    rsx! {
        div {
            class: format!("bg-bottom font-pixel text-sm rounded-xl {}", if greeting_is_finished {
                "opacity-0 animate-fade-in-down [animation-delay:3750ms]"
            } else {
                "hidden"
            }),
            background_image: format!(
                "url({})",
                asset!("/assets/images/projects/alex_and_der_murder.png")
            ),
            div {
                class: "py-4 px-5 flex flex-col items-start gap-4 bg-linear-[to_bottom,_rgba(0,_0,_0,_0.75)_0px,_transparent_15rem,_transparent_calc(100%_-_50px),_rgba(0,_0,_0,_0.75)_100%] sm:bg-linear-[to_bottom,_rgba(0,_0,_0,_0.75)_0px,_transparent_14rem,_transparent_calc(100%_-_40px),_rgba(0,_0,_0,_0.75)_100%] rounded-xl",
                h2 {
                    class: "text-2xl",
                    Localized {
                        czech: rsx! { "Bach a vrah" },
                        english: rsx! { "Alex and der murder" },
                    }
                }
                p {
                    class: "leading-6.5",
                    Localized {
                        czech: rsx! {
                            "Saša, skoky a sloky, pády a páky. Napětí, skutečné životy, "
                            span {
                                class: "text-lg",
                                " 🇨🇿 "
                            }
                            " i "
                            span {
                                class: "text-lg",
                                " 🇬🇧 "
                            }
                            " dabing."
                        },
                        english: rsx! {
                            "Sasha, rhymes and rakes, leaps and levers. Fully voiced in both "
                            span {
                                class: "text-lg",
                                " 🇬🇧 "
                            }
                            " and "
                            span {
                                class: "text-lg",
                                " 🇨🇿 "
                            }
                            "."
                        },
                    }
                }
                p {
                    Localized {
                        czech: rsx! { "Od studia Hryziko." },
                        english: rsx! { "From Hryziko games." },
                    }

                }
                Link {
                    class: "mt-40 py-2.75 px-5 bg-[#ce0000] hover:bg-slate-300 hover:text-[#ce0000] duration-150",
                    to: "https://hryziko.dev",
                    new_tab: true,
                    Localized {
                        czech: rsx! { "Hrát" },
                        english: rsx! { "Play" },
                    }
                }
            }
        }
    }
}
