use yew::prelude::*;

#[derive(Clone, PartialEq)]
pub struct UseConfigHandle {
    api_url: &'static str,
}

impl UseConfigHandle {
    pub fn api_url(&self) -> &'static str {
        self.api_url
    }
}

#[hook]
pub fn use_config() -> UseConfigHandle {
    UseConfigHandle {
        api_url: env!("API_URL"),
    }
}
