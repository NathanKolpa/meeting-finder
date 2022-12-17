import './style/main.scss'

import {initialize} from "./view";
import {fetchMeetings} from "./api";


document.body.onload = async () => {
    let [results, map] = initialize();

    map.setMeetingClickCallback(meeting => {
        results.focus(meeting);
    });

    results.setViewOnMapCallback(meeting => {
        map.focus(meeting);
    })

    let meetings = await fetchMeetings();

    results.setLoading(false);
    results.addMeetings(meetings);

    map.addMeetings(meetings);
};
