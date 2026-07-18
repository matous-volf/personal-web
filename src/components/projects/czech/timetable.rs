use crate::components::Localized;
use dioxus::prelude::*;

#[component]
pub(crate) fn Timetable(greeting_is_finished: bool) -> Element {
    rsx! {
        div {
            class: format!("py-4 px-5 flex flex-col items-start sm:flex-row-reverse sm:items-center justify-between gap-4 sm:gap-8 bg-neutral-700 font-sans rounded-xl {}", if greeting_is_finished {
                "opacity-0 animate-fade-in-down [animation-delay:3750ms]"
            } else {
                "hidden"
            }),
            img {
                class: "h-24 sm:h-36 rounded-lg",
                src: asset!("/assets/images/projects/timetable.png")
            }
            div {
                class: "self-stretch flex flex-col justify-between items-start gap-6",
                div {
                    class: "flex flex-col gap-2",
                    h2 {
                        class: "text-3xl font-bold",
                        Localized {
                            czech: rsx! { "Rozvrh" },
                            english: rsx! { "Timetable" },
                        }
                    }
                    p {
                        Localized {
                            czech: rsx! {
                                "Zobrazuje odpočet doby zbývající do přestávky nebo další hodiny školního rozvrhu spolu s následujícími předměty. Podporuje školy v systému Bakaláři. Inspirován "
                            },
                            english: rsx! {
                                "Displays a countdown to the next break or class along with upcoming subjects. Supports schools using the Bakaláři system. Inspired by "
                            },
                        }
                        Link {
                            class: "underline",
                            to: "https://github.com/czM1K3/DeltaTime",
                            new_tab: true,
                            "DeltaTime"
                        }
                        "."
                    }
                }
                Link {
                    class: "py-2.5 px-5 font-semibold bg-[#40a351] hover:bg-[#317d3e] text-white rounded duration-150",
                    to: "https://rozvrh.matousvolf.cz",
                    new_tab: true,
                    Localized {
                        czech: rsx! {
                            "Kdy už skončí hodina?"
                        },
                        english: rsx! {
                            "When will the class be over?"
                        },
                    }
                }
            }
        }
    }
}
