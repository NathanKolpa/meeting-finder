import './style/main.scss'
import 'leaflet/dist/leaflet.css';

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
			longitude: 10,
			latitude: 15,
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
			longitude: 50,
			latitude: 50,
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
