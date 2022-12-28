use std::rc::Rc;

use yew::prelude::*;

use common::model::SearchMeeting;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub meeting: Rc<SearchMeeting>,
}

#[function_component(MeetingListItem)]
pub fn meeting_list_item(props: &Props) -> Html {
    html! {
        <li class="result">
            <img class="logo" alt="Organization logo" />
            <div class="container">
                <div class="inner">
                    <div class="title">
                        <span class="name">{ &props.meeting.meeting.name }</span>
                        <span class="time">{ props.meeting.meeting.time.to_string() }</span>
                    </div>

                    <ul class="subtext">
                        if let Some(distance) = &props.meeting.distance {
                            <li class="distance">{ distance }</li>
                        }
                        if let Some(country) = &props.meeting.meeting.location.country {
                            <li class="country">{ country }</li>
                        }
                        if let Some(region) = &props.meeting.meeting.location.region {
                            <li class="region">{ region }</li>
                        }
                        if let Some(address) = &props.meeting.meeting.location.address {
                            <li class="address">{ address }</li>
                        }
                        if props.meeting.meeting.online_options.is_online {
                            <li class="online">{"Online"}</li>
                        }
                    </ul>

                    <ul class="actions">
                        <li class="link info">{"Info"}</li>
                        <li class="separator">{"|"}</li>
                        <li class="link focus">{"View on map"}</li>
                    </ul>
                </div>
            </div>
        </li>
    }
}
