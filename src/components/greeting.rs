use crate::components::{LanguageSelect, Localized, Terminal, Typing, typing::TypingPart};
use dioxus::prelude::*;
use std::time::Duration;

#[component]
pub(crate) fn Greeting(is_finished: Signal<bool>) -> Element {
    rsx! {
        div {
            class: "flex flex-col gap-8",
            Terminal {
                Typing {
                    on_finish: move |_| {
                        is_finished.set(true);
                    },
                    parts: Box::new([
                        TypingPart::Element(rsx! {
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
                            " "
                        }),
                        TypingPart::Text {
                            text: "cargo ".to_owned(),
                            class: None,
                        },
                        TypingPart::Text {
                            text: "new greeting".to_owned(),
                            class: Some("text-emerald-400".to_owned()),
                        },
                        TypingPart::Pause(Duration::from_millis(300)),
                        TypingPart::Element(rsx! {
                            br {}
                            span {
                                class: "text-lime-400 font-bold",
                                "    Creating"
                            }
                            span {
                                class: "hidden sm:inline",
                                " binary (application) `greeting` package"
                            }
                            span {
                                class: "sm:hidden",
                                " binary package"
                            }
                        }),
                        TypingPart::Pause(Duration::from_millis(300)),
                        TypingPart::Element(rsx! {
                            br {}
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
                            " "
                        }),
                        TypingPart::Pause(Duration::from_millis(500)),
                        TypingPart::Text {
                            text: "z ".to_owned(),
                            class: None,
                        },
                        TypingPart::Text {
                            text: "greeting".to_owned(),
                            class: Some("text-orange-400".to_owned()),
                        },
                        TypingPart::Pause(Duration::from_millis(300)),
                        TypingPart::Element(rsx! {
                            br {}
                            span {
                                class: "sm:hidden text-transparent",
                                background: "linear-gradient(to right, var(--color-lime-500) 50%, var(--color-slate-300) 50%)",
                                "-webkit-background-clip": "text",
                                background_clip: "text",
                                "~>"
                            }
                            span {
                                class: "sm:hidden text-transparent",
                                " "
                            }
                            span {
                                class: "hidden sm:inline",
                                span {
                                    class: "text-lime-400",
                                    "matous"
                                }
                                "@optiplex "
                                span {
                                    class: "text-lime-400",
                                    "~/greeting"
                                }
                                " (main)> "
                            }
                        }),
                        TypingPart::Pause(Duration::from_millis(500)),
                        TypingPart::Text {
                            text: "zeditor ".to_owned(),
                            class: None,
                        },
                        TypingPart::Text {
                            text: "src/main.rs".to_owned(),
                            class: Some("text-orange-400".to_owned()),
                        },
                        TypingPart::Pause(Duration::from_millis(1000)),
                        TypingPart::Element(rsx! {
                            br {}
                            span {
                                class: "sm:hidden text-transparent",
                                background: "linear-gradient(to right, var(--color-lime-500) 50%, var(--color-slate-300) 50%)",
                                "-webkit-background-clip": "text",
                                background_clip: "text",
                                "~>"
                            }
                            span {
                                class: "sm:hidden text-transparent",
                                " "
                            }
                            span {
                                class: "hidden sm:inline",
                                span {
                                    class: "text-lime-400",
                                    "matous"
                                }
                                "@optiplex "
                                span {
                                    class: "text-lime-400",
                                    "~/greeting"
                                }
                                " (main)> "
                            }
                        }),
                        TypingPart::Pause(Duration::from_millis(500)),
                        TypingPart::Text {
                            text: "cargo ".to_owned(),
                            class: None,
                        },
                        TypingPart::Text {
                            text: "run --release".to_owned(),
                            class: Some("text-emerald-400".to_owned()),
                        },
                        TypingPart::Pause(Duration::from_millis(500)),
                        TypingPart::Element(rsx! {
                            br {}
                            span {
                                class: "text-lime-400 font-bold",
                                "   Compiling"
                            }
                            span {
                                class: "sm:hidden",
                                " greeting v0.1.0"
                            }
                            span {
                                class: "hidden sm:inline",
                                " greeting v0.1.0 (~/greeting)"
                            }
                        }),
                        TypingPart::Pause(Duration::from_millis(100)),
                        TypingPart::Element(rsx! {
                            br {}
                            span {
                                class: "text-lime-400 font-bold",
                                "    Finished"
                            }
                            span {
                                class: "sm:hidden",
                                " in 0.37s"
                            }
                            span {
                                class: "hidden sm:inline",
                                " `release` profile [optimized] target(s) in 0.37s"
                            }
                        }),
                        TypingPart::Element(rsx! {
                            br {}
                            span {
                                class: "text-lime-400 font-bold",
                                "     Running"
                            }
                            span {
                                class: "sm:hidden",
                                " `greeting`"
                            }
                            span {
                                class: "hidden sm:inline",
                                " `target/debug/greeting`"
                            }
                        }),
                    ])
                }
            }
            if is_finished() {
                div {
                    class: "flex flex-col gap-8",
                    h2 {
                        class: "text-4xl sm:text-5xl text-center opacity-0 animate-fade-in-down",
                        Localized {
                            czech: rsx! { "Ahoj, světe." },
                            english: rsx! { "Hello, world." },
                        }
                    }
                    LanguageSelect {
                        class: "opacity-0 animate-fade-in-down [animation-delay:1250ms]",
                        greeting_is_finished: is_finished()
                    }
                }
            }
        }
    }
}
