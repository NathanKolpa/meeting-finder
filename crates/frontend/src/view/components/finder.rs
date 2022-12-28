use yew::prelude::*;
use crate::view::components::*;

#[function_component(Finder)]
pub fn finder() -> Html {
    html! {
        <div class="meeting-finder">
            <div class="search">
                <h1>{"Find meetings in your area."}</h1>
                <Search />
                <MeetingList is_loading={true} />
            </div>
            <WorldMap />
        </div>
    }
}