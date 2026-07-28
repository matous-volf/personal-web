mod alex_and_der_murder;
mod body_problem;
mod other_programming;
mod political_leaning;
mod rust_crates;

use crate::components::{
    Terminal,
    projects::international::{
        alex_and_der_murder::AlexAndDerMurder, body_problem::BodyProblem,
        other_programming::OtherProgramming, political_leaning::PoliticalLeaning,
        rust_crates::RustCrates,
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
                    "opacity-0 animate-fade-in-down [animation-delay:3500ms]"
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
                " eza "
                span {
                    class: "text-emerald-400",
                    "-la "
                }
                span {
                    class: "text-orange-400",
                    "projects"
                }
            }
            AlexAndDerMurder { greeting_is_finished },
            BodyProblem { greeting_is_finished },
            PoliticalLeaning { greeting_is_finished },
            RustCrates { greeting_is_finished },
            OtherProgramming { greeting_is_finished },
        }
    }
}
