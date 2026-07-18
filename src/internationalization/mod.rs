use serde::{Deserialize, Serialize};

#[derive(Default, PartialEq, Serialize, Deserialize, Clone)]
pub(crate) enum Language {
    Czech,
    #[default]
    English,
}

#[cfg(feature = "server")]
impl TryFrom<icu_locid::subtags::Language> for Language {
    type Error = ();

    fn try_from(value: icu_locid::subtags::Language) -> Result<Self, Self::Error> {
        match value.as_str() {
            "cs" => Ok(Self::Czech),
            "en" => Ok(Self::English),
            _ => Err(()),
        }
    }
}
