use yew::prelude::*;

use common::model::Organization;
use crate::view::services::logo_mapper;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub org: Organization,
}

#[function_component(Logo)]
pub fn logo(props: &Props) -> Html {
    let url = logo_mapper::map_org_to_logo_url(&props.org);

    html! {
        <img class="logo" alt="Organization logo" src={url} />
    }
}
