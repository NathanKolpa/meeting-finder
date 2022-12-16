import './style/main.scss'
import 'leaflet/dist/leaflet.css';

import * as page from './page';

document.body.onload = async () => {
    page.initialize();

    page.setMeetings([
        {
            longitude: 10,
            latitude: 15,
            org: 'AnonymousAlcoholics',

            country: 'Nederland',
            region: 'Gelderland',
            distance: 10,
            online: true,
            name: 'Warsaw Friday',
            conferenceUrl: 'https://alcoholics-anonymous.eu/meetings/?tsml-day=5&tsml-query=test'
        },
        {
            longitude: 10,
            latitude: 15,
            org: 'AnonymousAlcoholics',

            country: 'Nederland',
            region: 'Gelderland',
            distance: 10,
            online: true,
            name: 'Warsaw Friday',
            conferenceUrl: 'https://alcoholics-anonymous.eu/meetings/?tsml-day=5&tsml-query=test'
        }
    ])
};