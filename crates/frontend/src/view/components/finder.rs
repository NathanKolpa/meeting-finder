use std::rc::Rc;
use yew::prelude::*;
use yew_hooks::prelude::*;

use crate::view::components::search::SearchFilters;
use crate::view::components::*;
use crate::view::hooks::use_config;
use crate::view::services::meeting_service;

#[function_component(Finder)]
pub fn finder() -> Html {
    let config = use_config();

    let filters = use_state(|| SearchFilters::default());

    let meetings = use_async(async move {
        let result = meeting_service::get_meetings(config.api_url()).await;

        result.map(|m| Rc::new(m))
    });

    {
        let meetings = meetings.clone();
        use_effect_with_deps(
            move |_| {
                meetings.run();
                || ()
            },
            filters.clone(),
        );
    }

    let update_filters = Callback::from(move |f| filters.set(f));

    html! {
        <div class="meeting-finder">
            <div class="search">
                <h1>{"Find meetings in your area."}</h1>
                <Search on_submit={update_filters} />
                if let Some(error) = &meetings.error {
                    { error.to_string() }
                }
                else if let Some(meetings) = &meetings.data {
                    <MeetingList max_size={20} meetings={meetings.clone()} />
                }
                else {
                    <Spinner />
                }
            </div>
            <WorldMap />
        </div>
    }
}
