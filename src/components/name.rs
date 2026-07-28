use crate::{PROFILE_PICTURE, components::Terminal};
use dioxus::prelude::*;

#[component]
pub(crate) fn Name(greeting_is_finished: bool) -> Element {
    rsx! {
        div {
            class: "flex flex-col gap-8",
            Terminal {
                class: if greeting_is_finished {
                    "opacity-0 animate-fade-in-down [animation-delay:2250ms]"
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
                " whoami"
            }
            div {
                class: format!(
                    "flex flex-row mr-3 sm:mr-4 items-center justify-center gap-5 {}",
                    if greeting_is_finished {
                        "opacity-0 animate-fade-in-down [animation-delay:2500ms]"
                    } else {
                        "hidden"
                    }
                ),
                img {
                    class: "w-14 sm:w-16 rounded-full",
                    src: PROFILE_PICTURE,
                }
                h1 {
                    class: "text-2xl sm:text-3xl text-center",
                    "Matouš Volf"
                }
            }
        }
    }
}
