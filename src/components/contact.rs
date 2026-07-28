use crate::components::{Localized, Terminal};
use dioxus::prelude::*;

#[component]
pub(crate) fn Contact() -> Element {
    rsx! {
        div {
            class: "flex flex-col gap-8",
            Terminal {
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
                " finger"
            }
            div {
                class: "flex flex-col gap-2",
                p {
                    Localized {
                        czech: rsx! {
                            "Pakliže jste viděli, co dělám, a přesto si přejete se se mnou spojit, můžete tak učinit skrze (v pořadí podle mojí preference)"
                        },
                        english: rsx! {
                            "If you've seen what I do and nevertheless wish to reach me, you can do so through (in order of my preference)"
                        },
                    }
                }
                ul {
                    class: "pl-5 flex flex-col gap-2 list-disc",
                    li {
                        "Matrix: "
                        span {
                            class: "inline-block",
                            Link {
                                class: "underline",
                                to: "https://matrix.to/#/@matous-volf:matrix.matousvolf.cz",
                                new_tab: true,
                                span {
                                    class: "text-nowrap",
                                    "@matous-volf"
                                }
                                // Allow to break between the spans.
                                wbr {}
                                span {
                                    ":matrix.matousvolf.cz",
                                }
                            }
                            ","
                        }
                    }
                    li {
                        Localized {
                            czech: rsx! {
                                "spuštění příkazu "
                            },
                            english: rsx! {
                                "running the command "
                            },
                        }
                        span {
                            class: "px-2 py-1 bg-slate-900 font-mono rounded-lg",
                            "echo "
                            span {
                                class: "text-yellow-700",
                                Localized {
                                    czech: rsx! { "'vaše zpráva pro mě'" },
                                    english: rsx! { "'your message for me'" },
                                }
                            }
                            " "
                            span {
                                class: "text-emerald-400",
                                ">"
                            }
                            " "
                            span {
                                class: "text-orange-400",
                                "/dev/null"
                            }
                        }
                        ","
                    }
                    li {
                        "Discord: "
                        Link {
                            class: "underline",
                            to: "https://discord.com/users/727428884060045342",
                            new_tab: true,
                            "matous_volf",
                        }
                        ","
                    }
                    li {
                        "e-mail: ",
                        Link {
                            class: "underline",
                            to: "mailto:me@matousvolf.cz",
                            new_tab: true,
                            "me@matousvolf.cz"
                        }
                        "."
                    }
                }
            }
        }
    }
}
