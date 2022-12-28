use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub api_link: AttrValue,
}

#[function_component(About)]
pub fn about(props: &Props) -> Html {
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
                {"This project is free and open-source, you can view the repository on "}<a href="https://github.com/NathanKolpa/meeting-finder">{"Github"}</a>{"."}
                <br />
                {"For more information on our API, visit the documentation."}
                <a href={&props.api_link}>{"documentation"}</a>
                {"."}
            </p>
        </section>
    }
}
