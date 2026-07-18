use dioxus::prelude::*;

#[component]
pub(crate) fn Terminal(children: Element, class: Option<String>) -> Element {
    rsx! {
        div {
            class: format!(
                "py-2 px-3 sm:py-4 sm:px-5 bg-slate-900 text-slate-300 font-mono whitespace-pre-wrap rounded-lg sm:rounded-xl {}",
                class.unwrap_or(String::new())
            ),
            {children}
        }
    }
}
