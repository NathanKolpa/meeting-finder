use std::rc::Rc;

use yew::prelude::*;

use common::model::SearchMeeting;

use crate::view::components::Spinner;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub max_size: i32,
    pub meetings: Rc<Vec<SearchMeeting>>,
}

#[function_component(MeetingList)]
pub fn meeting_list(props: &Props) -> Html {
    html! {
        <div class="result-list">
            <ul class="results">
            </ul>

            <div class="pagination">
                <button class="prev-page">{"Prev"}</button>
                <button class="next-page">{"Next"}</button>
            </div>
        </div>
    }
}
