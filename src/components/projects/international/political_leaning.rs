use crate::components::Localized;
use dioxus::prelude::*;

#[component]
pub(crate) fn PoliticalLeaning(greeting_is_finished: bool) -> Element {
    rsx! {
        div {
            class: format!("py-4 px-5 flex flex-col gap-4 items-start bg-slate-300 text-slate-800 rounded-xl {}", if greeting_is_finished {
                "opacity-0 animate-fade-in-down [animation-delay:3750ms]"
            } else {
                "hidden"
            }),
            h2 {
                class: "text-3xl font-bold",
                Localized {
                    czech: rsx! {
                        "Klasifikace politického zabarvení a političnosti textu pomocí transformerů"
                    },
                    english: rsx! {
                        "Classifying political leaning and politicalness of text using transformer models"
                    },
                }
            }
            p {
                Localized {
                    czech: rsx! {
                        "Analýza strojovým učením s cílem určit, zda text mluví o politice a jestli podporuje "
                        span {
                            class: "py-0.5 px-1 bg-sky-700 text-slate-300 rounded",
                            "levici"
                        }
                        ", "
                        span {
                            class: "py-0.5 px-1 bg-neutral-600 text-slate-300 rounded",
                            "střed"
                        }
                        " nebo "
                        span {
                            class: "py-0.5 px-1 bg-rose-700 text-slate-300 rounded",
                            "pravici"
                        }
                        ". Vědecký článek napsaný mnou a Jakubem Šimkem z "
                        Link {
                            class: "underline",
                            to: "https://kinit.sk",
                            new_tab: true,
                            "KInITu"
                        }
                        ". Zároveň maturitní projekt (na SŠ "
                        Link {
                            class: "underline",
                            to: "https://www.delta-skola.cz",
                            new_tab: true,
                            "DELTA"
                        }
                        ") a práce "
                        Link {
                            class: "underline",
                            to: "https://www.soc.cz",
                            new_tab: true,
                            "SOČ"
                        }
                        "."
                    },
                    english: rsx! {
                        "Machine learning analysis to determine whether a text talks about politics and if it supports the "
                        span {
                            class: "py-0.5 px-1 bg-sky-700 text-slate-300 rounded",
                            "left"
                        }
                        ", "
                        span {
                            class: "py-0.5 px-1 bg-neutral-600 text-slate-300 rounded",
                            "center"
                        }
                        " or "
                        span {
                            class: "py-0.5 px-1 bg-rose-700 text-slate-300 rounded",
                            "right"
                        }
                        ". A paper written by me and Jakub Šimko from "
                        Link {
                            class: "underline",
                            to: "https://kinit.sk",
                            new_tab: true,
                            "KInIT"
                        }
                        ". Also a high school ("
                        Link {
                            class: "underline",
                            to: "https://www.delta-skola.cz",
                            new_tab: true,
                            "DELTA"
                        }
                        ") graduation and "
                        Link {
                            class: "underline",
                            to: "https://www.soc.cz",
                            new_tab: true,
                            "SOČ"
                        }
                        " project."
                    },
                }
            }
            div {
                class: "flex flex-row flex-wrap items-center gap-4",
                Link {
                    class: "py-2 px-4.5 font-semibold rounded-lg bg-slate-300 hover:bg-slate-400 border border-2 border-slate-400 hover:text-slate-900 duration-150",
                    to: "https://github.com/matous-volf/political-leaning-prediction",
                    new_tab: true,
                    Localized {
                        czech: rsx! { "Prozkoumat práci" },
                        english: rsx! { "Explore the work" },
                    }
                }
                Link {
                    class: "py-2.5 px-5 font-semibold rounded-lg bg-gradient-to-r from-sky-700 to-rose-700 hover:from-sky-800 hover:to-rose-800 text-slate-300 duration-150 transition-colors",
                    to: "https://political-leaning.matousvolf.cz",
                    new_tab: true,
                    Localized {
                        czech: rsx! { "Vyzkoušet aplikaci" },
                        english: rsx! { "Try the app" },
                    }
                }
            }
        }
    }
}
