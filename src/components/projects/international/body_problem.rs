use dioxus::prelude::*;

use crate::components::Localized;

#[component]
pub(crate) fn BodyProblem(greeting_is_finished: bool) -> Element {
    rsx! {
        div {
            class: format!("pb-4 pt-60 sm:pt-4 pl-5 sm:pl-65 pr-5 flex flex-col items-end gap-7 bg-top-left sm:bg-[left_20px_center] bg-no-repeat bg-size-[auto_260px] sm:bg-size-[340px_auto] bg-black font-ubuntu text-right rounded-xl {}", if greeting_is_finished {
                "opacity-0 animate-fade-in-down [animation-delay:3750ms]"
            } else {
                "hidden"
            }),
            background_image: format!(
                "url({})",
                asset!("/assets/images/projects/body_problem.png")
            ),
            div {
                class: "max-w-95 flex flex-col gap-2",
                h2 {
                    class: "text-3xl font-bold",
                    "Body problem"
                }
                p {
                    Localized {
                        czech: rsx! {
                            "Simulace "
                            Link {
                                class: "underline",
                                to: "https://en.wikipedia.org/wiki/N-body_problem",
                                "problému n těles"
                            }
                            " (obecného případu problému tří těles) ve webové aplikaci s možností libovolného rozmístění. Původně vyvinuto na "
                            Link {
                                class: "underline",
                                to: "https://kdfls5.troja.mff.cuni.cz/tabor",
                                new_tab: true,
                                "Soustředění mladých fyziků a matematiků"
                            }
                            "."
                        },
                        english: rsx! {
                            "A simulation of the "
                            Link {
                                class: "underline",
                                to: "https://en.wikipedia.org/wiki/N-body_problem",
                                "n-body problem"
                            }
                            " (a general case of the three-body problem) in a web app with customizable body layout. Originally developed at the "
                            Link {
                                class: "underline",
                                to: "https://kdfls5.troja.mff.cuni.cz/tabor",
                                new_tab: true,
                                "SMFM summer camp"
                            }
                            "."
                        },
                    }
                }
            }
            Link {
                class: "py-2.5 px-5 font-semibold rounded-lg bg-slate-300 hover:bg-slate-400 text-slate-800 hover:text-slate-900 duration-150",
                to: "https://body-problem.matousvolf.cz",
                new_tab: true,
                Localized {
                    czech: rsx! {
                        "Zahájit simulaci"
                    },
                    english: rsx! {
                        "Start simulating"
                    },
                }

            }
        }
    }
}
