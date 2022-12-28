use yew::prelude::*;

use crate::view::components::Spinner;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub is_loading: bool,
}

#[function_component(MeetingList)]
pub fn meeting_list(props: &Props) -> Html {
    html! {
        <div class="result-list">
            if props.is_loading {
                <Spinner />
            } else {
                <ul class="results"></ul>

                <div class="pagination">
                    <button class="prev-page">{"Prev"}</button>
                    <button class="next-page">{"Next"}</button>
                </div>
            }
        </div>
    }
}