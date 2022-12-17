import './style/main.scss'

import {Meeting} from "./models";
import {initialize} from "./view";


let meetings: Meeting[] = [
    {
        id: 1,
        position: {
            longitude: 10,
            latitude: 15,
        },
        org: 'AnonymousAlcoholics',
        country: 'Nederland',
        region: 'Gelderland',
        distance: 10,
        online: true,
        name: 'Warsaw Friday',
        conferenceUrl: 'https://alcoholics-anonymous.eu/meetings/?tsml-day=5&tsml-query=test',
    },
    {
        id: 2,
        position: {
            longitude: 50,
            latitude: 50,
        },
        org: 'AnonymousAlcoholics',
        country: 'Nederland',
        region: 'Gelderland',
        distance: 10,
        online: true,
        name: 'Warsaw Friday',
        conferenceUrl: 'https://alcoholics-anonymous.eu/meetings/?tsml-day=5&tsml-query=test'
    },
    {
        id: 3,
        position: null,
        org: 'AnonymousAlcoholics',
        country: 'Nederland',
        region: 'Gelderland',
        distance: 10,
        online: true,
        name: 'Warsaw Friday',
        conferenceUrl: 'https://alcoholics-anonymous.eu/meetings/?tsml-day=5&tsml-query=test'
    }
];

document.body.onload = async () => {
    let [results, map] = initialize();

    map.setMeetingClickCallback(meeting => {
        results.focus(meeting);
    });

    results.setViewOnMapCallback(meeting => {
        map.focus(meeting);
    })

    results.setLoading(false);
    results.addMeetings(meetings);

    map.addMeetings(meetings);
};
