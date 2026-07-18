mod gold_and_steel;
mod pardubice_hacker;
mod timetable;

use crate::components::{
    Terminal,
    projects::czech::{
        gold_and_steel::GoldAndSteel, pardubice_hacker::PardubiceHacker, timetable::Timetable,
    },
};
use dioxus::prelude::*;

#[component]
pub(crate) fn Projects(greeting_is_finished: bool) -> Element {
    rsx! {
        div {
            class: "flex flex-col gap-8 text-pretty",
            Terminal {
                class: if greeting_is_finished {
                    "opacity-0 animate-fade-in-down [animation-delay:2500ms]"
                } else {
                    "hidden"
                },
                span {
                    class: "hidden sm:inline",
                    span {
                        class: "text-lime-400",
                        "matous"
                    }
                    "@optiplex "
                }
                span {
                    class: "text-transparent",
                    background: "linear-gradient(to right, var(--color-lime-500) 50%, var(--color-slate-300) 50%)",
                    "-webkit-background-clip": "text",
                    background_clip: "text",
                    "~>"
                }
                " rg "
                span {
                    class: "text-emerald-400",
                    "'czech|🇨🇿'"
                }
                " "
                span {
                    class: "text-orange-400",
                    "projects"
                }
            }
            PardubiceHacker {
                greeting_is_finished
            }
            Timetable {
                greeting_is_finished
            }
            GoldAndSteel {
                greeting_is_finished
            }
        }
    }
}
