use crate::views::{Home, NotFound};
use dioxus::prelude::*;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub(crate) enum Route {
    #[route("/")]
    Home {},
    #[route("/:..route")]
    NotFound {
        route: Vec<String>,
    },
}
