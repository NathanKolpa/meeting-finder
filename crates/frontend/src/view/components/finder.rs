use yew::prelude::*;
use yew_hooks::prelude::*;

use crate::view::components::*;
use crate::view::components::search::SearchFilters;
use crate::view::hooks::use_config;
use crate::view::services::meeting_service;

#[function_component(Finder)]
pub fn finder() -> Html {
    let config = use_config();

    let filters = use_state(|| SearchFilters::default());

    let meetings = use_async(async move {
        meeting_service::get_meetings(config.api_url())
            .await
    });

    {
        let meetings = meetings.clone();
        use_effect_with_deps(move |_| { meetings.run(); || () }, filters);
    }

    html! {
        <div class="meeting-finder">
            <div class="search">
                <h1>{"Find meetings in your area."}</h1>
                <Search />
                if let Some(error) = &meetings.error {
                    { error.to_string() }
                }
                else if let Some(meetings) = &meetings.data {
                    <MeetingList />
                }
                else {
                    <Spinner />
                }
            </div>
            <WorldMap />
        </div>
    }
}