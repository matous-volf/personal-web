use dioxus::prelude::*;

use crate::components::Localized;

#[component]
pub(crate) fn PardubiceHacker(greeting_is_finished: bool) -> Element {
    rsx! {
        div {
            class: format!("py-4 px-5 flex flex-col items-end gap-6 bg-sky-700 bg-no-repeat bg-position-[left_-0.5rem_top_-2rem] sm:bg-position-[left_2rem_top_-3rem] bg-size-[256] font-ubuntu font-medium text-right rounded-xl {}", if greeting_is_finished {
                "opacity-0 animate-fade-in-down [animation-delay:3750ms]"
            } else {
                "hidden"
            }),
            background_image: format!("url({})", asset!("/assets/images/projects/pardubice_hacker.png")),
            div {
                class: "flex flex-col gap-1",
                h2 {
                    class: "mt-36 sm:mt-0 text-3xl font-bold",
                    Localized {
                        czech: rsx! { "Pardubický hacker" },
                        english: rsx! { "Pardubice hacker" },
                    }
                }
                p {
                    Localized {
                        czech: rsx! {
                            "Korespondenční seminář a soutěž z programování pro žáky základních škol. Připravil jsem studijní texty, úlohy a automatizované testy pro "
                            Link {
                                class: "underline",
                                to: "https://github.com/delta-cs/seminar",
                                new_tab: true,
                                "první ročník"
                            }
                            "."
                        },
                        english: rsx! {
                            "A correspondence programming seminar and contest for middle school students. I prepared the study materials, problem sets, and automated tests for the "
                            Link {
                                class: "underline",
                                to: "https://github.com/delta-cs/seminar",
                                new_tab: true,
                                "first year"
                            }
                            "."
                        },
                    }
                }
            }
            Link {
                class: "py-2 px-4.5 order border-2 border-slate-300 hover:bg-slate-300 hover:text-sky-700 font-mono font-semibold rounded-tl-xl rounded-br-xl hover:rounded-tr-xl hover:rounded-bl-xl hover:rounded-tl-none hover:rounded-br-none duration-150",
                to: "https://pardubicky-hacker.cz",
                new_tab: true,
                Localized {
                    czech: rsx! { "Aktuální ročník" },
                    english: rsx! { "Current season" },
                }
            }
        }
    }
}
