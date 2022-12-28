use yew::prelude::*;

use crate::view::components::*;
use crate::view::hooks::use_config;

#[function_component(App)]
pub fn app() -> Html {
    let config = use_config();

    html! {
        <div class="app">
            <main>
                <Finder />
                <About api_link={config.api_url()} />
            </main>
        </div>
    }
}
