use yew::prelude::*;

use crate::view::components::Spinner;

#[derive(Properties, PartialEq)]
pub struct Props {
}

#[function_component(MeetingList)]
pub fn meeting_list(props: &Props) -> Html {
    html! {
        <div class="result-list">
            <ul class="results"></ul>

            <div class="pagination">
                <button class="prev-page">{"Prev"}</button>
                <button class="next-page">{"Next"}</button>
            </div>
        </div>
    }
}