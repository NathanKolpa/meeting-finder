import './style/main.scss'

import {initialize} from "./view";
import {fetchMeetings} from "./api";


document.body.onload = async () => {
    let [results, map, popup] = initialize();


    map.setMeetingClickCallback(m => popup.showMeeting(m));
    results.setShowInfoCallback(m => popup.showMeeting(m));
    results.setViewOnMapCallback(m => map.focus(m));

    let meetings = await fetchMeetings();

    results.setLoading(false);
    results.addMeetings(meetings);

    map.addMeetings(meetings);
};
