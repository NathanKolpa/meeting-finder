import './style/main.scss'

import * as page from './page';

document.body.onload = async () => {
    page.initialize();

    page.setOnViewOnMapClick((meeting) => {
        page.setMapFocus(meeting);
        page.setFocusTo(meeting);
    });

    page.setOnFocusClick((meeting) => {
        page.setFocusTo(meeting);
    })

    page.setMeetings([
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
    ]);
};
