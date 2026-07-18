use std::time::Duration;

use async_std::task::sleep;
use dioxus::prelude::*;

const DURATION_BETWEEN_STROKES_DEFAULT: Duration = Duration::from_millis(50);

#[derive(PartialEq, Clone)]
pub(crate) enum TypingPart {
    Text { text: String, class: Option<String> },
    Element(Element),
    Pause(Duration),
}

#[component]
pub(crate) fn Typing(parts: Box<[TypingPart]>, on_finish: Option<Callback>) -> Element {
    if parts.is_empty() {
        return VNode::empty();
    }

    let mut current_part_index = use_signal(|| 0);
    let mut current_char_index = use_signal(|| 0);

    {
        let parts = parts.clone();
        use_effect(move || {
            let parts = parts.clone();
            spawn(async move {
                let part_count = parts.len();
                for (part_index, part) in parts.into_iter().enumerate() {
                    current_char_index.set(0);
                    current_part_index.set(part_index);
                    match part {
                        TypingPart::Text { text, .. } => {
                            for index in 0..text.len() {
                                sleep(DURATION_BETWEEN_STROKES_DEFAULT).await;
                                current_char_index.set(index);
                            }
                        }
                        TypingPart::Pause(duration) => sleep(duration).await,
                        TypingPart::Element(_) => (),
                    }
                }
                current_part_index.set(part_count);
                if let Some(on_finish) = on_finish {
                    on_finish.call(());
                }
            });
        });
    }

    rsx! {
        for part in parts.iter().take(current_part_index()) {
            match part {
                TypingPart::Text { text, class } => rsx! {
                    span {
                        class: class.clone(),
                        "{text}",
                    },
                },
                TypingPart::Element(element) => element.clone(),
                TypingPart::Pause(_) => VNode::empty(),
            }
        }
        if current_part_index() < parts.len() {
            match &parts[current_part_index()] {
                TypingPart::Text { text, class } => rsx! {
                    span {
                        class: class,
                        "{&text[..current_char_index()]}"
                    }
                },
                TypingPart::Element(element) => element.clone(),
                TypingPart::Pause(_) => VNode::empty(),
            }
        }
    }
}
