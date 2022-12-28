use yew::prelude::*;

#[function_component(Spinner)]
pub fn spinner() -> Html {
    html! {
        <p class="loading">{"Loading..."}</p>
    }
}
