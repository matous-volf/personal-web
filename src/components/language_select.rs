use crate::{LANGUAGE, MANUAL_LANGUAGE_STORAGE_KEY, internationalization::Language};
use dioxus::prelude::*;
use dioxus_sdk::storage::{LocalStorage, use_synced_storage};

#[component]
pub(crate) fn LanguageSelect(greeting_is_finished: bool, class: Option<String>) -> Element {
    let mut manual_language =
        use_synced_storage::<LocalStorage, _>(MANUAL_LANGUAGE_STORAGE_KEY.to_owned(), || None);
    let language = LANGUAGE();

    rsx! {
        div {
            class: format!("flex flex-row justify-center items-center gap-8 {}", class.unwrap_or(String::new())),
            button {
                class: format!("h-8 outline outline-red-400/80 outline-0 {} duration-150 transition-all", match language {
                    Language::Czech => "outline-6 rounded-sm",
                    Language::English => "cursor-pointer",
                }),
                onclick: move |_| {
                    manual_language.set(Some(Language::Czech));
                    *LANGUAGE.write() = Language::Czech;
                },
                img {
                    class: format!("h-full aspect-3/2 rounded-sm {} duration-150", match language {
                        Language::Czech => "brightness-80",
                        Language::English => "brightness-50 hover:brightness-80",
                    }),
                    src: asset!("/assets/images/languages/cz.png")
                }
            }
            button {
                class: format!("h-8 outline outline-red-400/80 outline-0 {} duration-150 transition-all", match language {
                    Language::Czech => "cursor-pointer",
                    Language::English => "outline-6 rounded-sm",
                }),
                onclick: move |_| {
                    manual_language.set(Some(Language::English));
                    *LANGUAGE.write() = Language::English;
                },
                img {
                    class: format!("h-full aspect-3/2 rounded-sm {} duration-150", match language {
                        Language::Czech => "brightness-50 hover:brightness-80",
                        Language::English => "brightness-80",
                    }),
                    src: asset!("/assets/images/languages/en.webp")
                }
            }
        }
    }
}
