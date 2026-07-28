use crate::components::Localized;
use dioxus::prelude::*;

#[component]
pub(crate) fn GoldAndSteel(greeting_is_finished: bool) -> Element {
    rsx! {
        div {
            class: format!("px-5 flex flex-col sm:flex-row-reverse items-center sm:items-end gap-4 bg-[#587d24] text-[#92aac9] font-pixel text-sm text-right rounded-xl {}", if greeting_is_finished {
                "opacity-0 animate-fade-in-down [animation-delay:3750ms]"
            } else {
                "hidden"
            }),
            div {
                class: "py-4 flex flex-col items-end gap-6 rounded-xl",
                div {
                    class: "flex flex-col gap-1",
                    h2 {
                        class: "text-2xl text-[#ffef3b]",
                        "Gold and steel"
                    }
                    p {
                        Localized {
                            czech: rsx! {
                                "Těžba a zbroj, mince a boj. Pečlivou správou surovin a strategickým plánováním můžeš odvrátit zkázu království."
                            },
                            english: rsx! {
                                "Ore and armor, coins and war. Through careful handling of resources and strategic planning, you can avert the doom of the kingdom."
                            },
                        }
                    }
                }
                Link {
                    class: "py-2.75 px-5 bg-[#2357b1] hover:bg-[#ffef3b] text-[#ffef3b] hover:text-[#2357b1] duration-150",
                    to: "https://gold-and-steel.matousvolf.cz",
                    new_tab: true,
                    Localized {
                        czech: rsx! { "Přijmout výzvu" },
                        english: rsx! { "Accept the challenge" },
                    }
                }
            }
            img {
                class: "h-40 [image-rendering:pixelated]",
                src: asset!("/assets/images/projects/gold_and_steel.png")
            }
        }
    }
}
