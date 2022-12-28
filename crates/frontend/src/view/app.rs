use yew::prelude::*;

#[derive(Default)]
pub struct App {
}

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self::default()
    }


    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <h1>{ "Hi" }</h1>
        }
    }
}