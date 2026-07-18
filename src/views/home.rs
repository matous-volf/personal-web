use crate::components::{Contact, Footer, Greeting, Name, projects};
use dioxus::prelude::*;

#[component]
pub(crate) fn Home() -> Element {
    let greeting_is_finished = use_signal(|| false);

    rsx! {
        div {
            class: "grow w-full max-w-200 flex flex-col gap-24",
            Greeting {
                is_finished: greeting_is_finished,
            }
            Name {
                greeting_is_finished: greeting_is_finished()
            }
            projects::International {
                greeting_is_finished: greeting_is_finished()
            }
            projects::Czech {
                greeting_is_finished: greeting_is_finished()
            }
            div {
                class: format!("flex flex-col gap-12 {}",
                    if greeting_is_finished() {
                        "opacity-0 animate-fade-in-down [animation-delay:1250ms]"
                    } else {
                        "hidden"
                    }
                ),
                Contact {}
                Footer {}
            }
        }
    }
}
