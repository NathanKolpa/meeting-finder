import './style/main.scss'

import { initialize } from "./view";
import { fetchMeetings, fetchPositionByQuery } from "./api";


document.body.onload = async () => {
    let [results, map, popup, searchBar] = initialize();

    map.setMeetingClickCallback(m => popup.showMeeting(m));
    results.setShowInfoCallback(m => popup.showMeeting(m));
    results.setViewOnMapCallback(m => map.focus(m));

    searchBar.setOnSearchCallback(async query => {
        let position = null;
        let distanceOpt = undefined;

        if (query.location) {
            position = await fetchPositionByQuery(query.location);

            if (!position) {
                searchBar.setLocationError('Cannot find any matches');
                return;
            }
        }

        map.clear();
        results.clear();

        results.setLoading(true);

        let meetings = await fetchMeetings({
            distance: position ? { position, distance: query.distance } : undefined,
        })

        results.setLoading(false);

        results.addMeetings(meetings);
        map.addMeetings(meetings);

        if (position) {
            map.goToPosition(position);
        }
    })

    let meetings = await fetchMeetings();

    results.setLoading(false);
    results.addMeetings(meetings);

    map.addMeetings(meetings);
};
