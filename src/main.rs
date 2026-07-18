mod components;
mod dotenv;
mod internationalization;
mod route;
mod server;
mod views;

use crate::{
    dotenv::{DOMAIN, UMAMI_INSTANCE_DOMAIN, UMAMI_WEBSITE_ID},
    internationalization::Language,
    route::Route,
    server::get_language,
};
use dioxus::prelude::*;
use dioxus_sdk::storage::{LocalStorage, use_synced_storage};

const PROFILE_PICTURE: Asset = asset!("/assets/images/profile_picture.png");
const TAILWIND_CSS: Asset = asset!("/assets/styles/tailwind.css");
#[used]
static FONTS_DIRECTORY: Asset = asset!(
    "/assets/fonts",
    AssetOptions::builder().with_hash_suffix(false)
);

fn main() {
    dioxus::launch(App);
}

pub(crate) const MANUAL_LANGUAGE_STORAGE_KEY: &str = "manual_language";
static LANGUAGE: GlobalSignal<Language> = Signal::global(Language::default);

#[component]
fn App() -> Element {
    let detected_language = use_server_future(get_language)?.suspend()?()?;
    if let Some(manual_language) =
        use_synced_storage::<LocalStorage, _>(MANUAL_LANGUAGE_STORAGE_KEY.to_owned(), || None)()
    {
        *LANGUAGE.write() = manual_language;
    } else {
        *LANGUAGE.write() = detected_language;
    }

    rsx! {
        document::Link { rel: "icon", href: PROFILE_PICTURE }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }

        document::Meta { name: "application-name", content: "Matouš Volf" }
        document::Meta { name: "description", content: "Hello, world. A portfolio website." }
        document::Meta { name: "author", content: "Matouš Volf" }
        document::Meta { name: "keywords", content: "matouš,matous,volf,portfolio,projects,personal,contact,projekty,osobní,kontakt" }
        document::Meta { name: "og:title", content: "Matouš Volf" }
        document::Meta { name: "og:description", content: "Hello, world. A portfolio website." }
        document::Meta { name: "og:url", content: "https://matousvolf.cz" }
        document::Meta { name: "og:site_name", content: "Matouš Volf" }
        document::Meta { name: "og:locale", content: "en_US" }
        document::Meta { name: "og:image", content: PROFILE_PICTURE }

        {
            #[allow(clippy::const_is_empty)]
            if !UMAMI_WEBSITE_ID.is_empty() {
                rsx! {
                    document::Script {
                        defer: true,
                        src: "https://{UMAMI_INSTANCE_DOMAIN}/script.js",
                        "data-website-id": UMAMI_WEBSITE_ID,
                        "data-domains": DOMAIN
                    }
                }
            } else {
                VNode::empty()
            }
        }

        main {
            class: "min-h-screen p-4 sm:p-12 flex flex-col items-center bg-slate-800 text-slate-300 font-serif",
            Router<Route> {}
        }
    }
}
