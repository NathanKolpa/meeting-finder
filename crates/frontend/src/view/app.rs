use yew::prelude::*;

use crate::view::components::*;
use crate::view::hooks::use_config;

#[function_component(App)]
pub fn app() -> Html {
    let config = use_config();

    html! {
        <div class="app">
            <main>
                <div class="meeting-finder">
                    <div class="search">
                        <MeetingList is_loading={true} />
                    </div>
                    <WorldMap />
                </div>
                <About api_link={config.api_url()} />
            </main>
        </div>
    }
}