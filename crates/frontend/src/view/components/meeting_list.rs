use std::ops::Deref;
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
    let page = use_state(|| 0);

    let meetings = props.meetings.iter()
        .skip(page.deref() * props.max_size)
        .take(props.max_size);

    let next_page = {
        let page = page.clone();
        Callback::from(move |_| {
            page.set(page.deref() + 1);
        })
    };

    let prev_page = {
        let page = page.clone();
        Callback::from(move |_| {
            page.set(page.deref() - 1);
        })
    };

    let disable_prev = page.deref().eq(&0);
    let disable_next = page.deref().ge(&(props.meetings.len() / props.max_size));

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
                <button class="prev-page" disabled={disable_prev} onclick={prev_page}>{"Prev"}</button>
                <button class="next-page" disabled={disable_next} onclick={next_page}>{"Next"}</button>
            </div>
        </div>
    }
}
