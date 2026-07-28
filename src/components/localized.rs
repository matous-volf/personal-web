use dioxus::prelude::*;

use crate::{LANGUAGE, internationalization::Language};

#[component]
pub(crate) fn Localized(czech: Element, english: Element) -> Element {
    match LANGUAGE() {
        Language::Czech => czech,
        Language::English => english,
    }
}
