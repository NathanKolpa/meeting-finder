use yew::prelude::*;

#[derive(Default)]
pub struct About;

impl Component for About {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self::default()
    }


    fn view(&self, _ctx: &Context<Self>) -> Html {
        let api_link = env!("API_URL");

        html! {
            <section class="about">
                <h2>{"About"}</h2>
                <p>
                    {"Welcome to our website, where you can find and filter 12-step meetings in your area.
                    We created this site because we found it annoying to have to search multiple
                    websites for the same information, and we wanted to make it easier for people to find the support
                    and fellowship they need in recovery."}
                </p>
                <p>
                    {"Our goal is to provide a convenient and comprehensive way for people to find 12-step meetings in
                    their area, whether they're new to recovery or have been in the program for a while. We cover
                    multiple groups and locations, and offer a range of filters to help people find the
                    right meetings for their needs."}
                </p>

                <h3>{"Project info"}</h3>
                <p>
                    {"This project is free and open-source, you can view the repository on "}
                    <a href={api_link}>{"Github"}</a>
                    {"."}
                    <br />
                    {"For more information on our API, visit the documentation."}
                </p>
            </section>
        }
    }
}