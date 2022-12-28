use yew::prelude::*;

use crate::view::about::About;

#[derive(Default)]
pub struct App {}

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self::default()
    }


    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div class="app">
                <main>
                    <About />
                </main>
            </div>
        }
    }
}