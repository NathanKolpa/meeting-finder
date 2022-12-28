use std::ops::Deref;

use web_sys::HtmlInputElement;
use yew::prelude::*;

use crate::view::components::search::_Props::on_submit;

#[derive(Default, PartialEq)]
pub struct SearchFilters {
    pub location: Option<String>,
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub on_submit: Callback<SearchFilters>,
}

#[function_component(Search)]
pub fn search(props: &Props) -> Html {
    let location = use_state(|| None);

    let on_location_input_change = {
        let location = location.clone();

        Callback::from(move |event: Event| {
            use wasm_bindgen::JsCast;
            let value = event
                .target()
                .unwrap()
                .unchecked_into::<HtmlInputElement>()
                .value();
            location.set(if value.is_empty() { None } else { Some(value) })
        })
    };

    let submit_handler = {
        let callback = props.on_submit.clone();
        let location = location.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();

            callback.emit(SearchFilters {
                location: location.deref().clone(),
            });
        })
    };

    html! {
        <form method="GET" class="filters" id="search" onsubmit={submit_handler}>
            <div class="input-fields">
                <label class="form-group location">
                    <input class="form-control control-left" type="text"
                        placeholder="Enter a city, address, postal code, etc..." name="location" onchange={on_location_input_change} />
                    <div class="absolute-container">
                        <small class="absolute-inner location-feedback" hidden={true}></small>
                    </div>
                </label>
                <label class="form-group distance">
                    <select name="distance" class="form-control control-right">
                        <option value="10">{"10 km"}</option>
                        <option value="25">{"25 km"}</option>
                        <option value="50">{"50 km"}</option>
                        <option value="100" selected={true}>{"100 km"}</option>
                        <option value="200">{"200 km"}</option>
                        <option value="400">{"400 km"}</option>
                        <option value="800">{"800 km"}</option>
                        <option value="all">{"> 200 km"}</option>
                    </select>
                </label>
            </div>
            <button class="submit">{"Search"}</button>
        </form>
    }
}
