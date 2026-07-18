use crate::internationalization::Language;
#[cfg(feature = "server")]
use dioxus::fullstack::HeaderMap;
use dioxus::prelude::*;

#[cfg(feature = "server")]
mod server_only {
    use dioxus::{fullstack::HeaderMap, server::http::header};
    use fluent_langneg::{NegotiationStrategy, negotiate_languages, parse_accepted_languages};
    use icu_locid::{LanguageIdentifier, langid};

    use crate::internationalization::Language;

    pub(super) async fn get_language(header_map: &HeaderMap) -> Language {
        const AVAILABLE_IDENTIFIERS: [LanguageIdentifier; 2] = [langid!("en"), langid!("cs")];
        header_map
            .get(header::ACCEPT_LANGUAGE)
            .and_then(|header_value| {
                header_value.to_str().ok().and_then(|header_value| {
                    negotiate_languages(
                        parse_accepted_languages(header_value).as_slice(),
                        AVAILABLE_IDENTIFIERS.as_slice(),
                        None,
                        NegotiationStrategy::Lookup,
                    )
                    .first()
                    .and_then(|language_identifier| {
                        (**language_identifier).clone().language.try_into().ok()
                    })
                })
            })
            .unwrap_or(Language::default())
    }
}

#[get("/api/language", header_map: HeaderMap)]
pub(crate) async fn get_language() -> Result<Language> {
    Ok(server_only::get_language(&header_map).await)
}
