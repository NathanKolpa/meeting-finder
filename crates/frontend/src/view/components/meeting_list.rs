use std::rc::Rc;

use yew::prelude::*;

use common::model::SearchMeeting;

use crate::view::components::MeetingListItem;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub max_size: usize,
    pub meetings: Rc<Vec<Rc<SearchMeeting>>>,
}

#[function_component(MeetingList)]
pub fn meeting_list(props: &Props) -> Html {
    let meetings = props.meetings.iter().take(props.max_size);

    html! {
        <div class="result-list">
            <ul class="results">
                {for meetings.map(|m| {
                    html! {
                        <MeetingListItem meeting={m.clone()} />
                    }
                })}
            </ul>

            <div class="pagination">
                <button class="prev-page">{"Prev"}</button>
                <button class="next-page">{"Next"}</button>
            </div>
        </div>
    }
}
