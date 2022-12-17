import L from 'leaflet';
import { Meeting, Organization } from "./models";
import aaLogoUrl from './assets/logos/aa.png';

type MeetingCallback = ((meeting: Meeting) => void) | null;

let map: L.Map;
let loading: HTMLElement;
let results: HTMLUListElement;

let onInfoClick: MeetingCallback = null;
let onFocusClick: MeetingCallback = null;

export function setOnInfoClick(callback: MeetingCallback) {
	onInfoClick = callback;
}

export function setOnFocusClick(callback: MeetingCallback) {
	onFocusClick = callback;
}

function getLogoImgUrlByOrg(org: Organization): string {
	switch (org) {
		case "AnonymousAlcoholics":
			return aaLogoUrl;
	}
}

function createHtmlCallback(meeting: Meeting, getCallback: () => MeetingCallback): () => void {
	return () => {
		let callback = getCallback();

		if (callback) {
			callback(meeting);
		}
	}
}

export function setMeetings(meetings: Meeting[]) {
	setLoadingEnabled(false);

	for (const meeting of meetings) {
		let li = document.createElement("li");

		li.className = 'result';
		li.innerHTML =
			`
            <img class="logo" alt="Organization logo">
            <p class="title">
                <span class="name">[Title]</span>
                <span class="time">[time]</span>
            </p>
            
            <ul class="subtext">
                <li class="country">[Country]</li>
                <li class="region">[Region]</li>
                <li class="distance">[Distance] <span>km</span></li>
                <li class="online">[online]</li>
            </ul>
            
            <ul class="actions">
                <li class="link info">Info</li>
                <li>|</li>
                <li class="link focus">View on map</li>
            </ul>
            `;

		let logo = li.getElementsByClassName('logo')[0]! as HTMLImageElement;
		let name = li.getElementsByClassName('name')[0]!;
		let country = li.getElementsByClassName('country')[0]!;
		let region = li.getElementsByClassName('region')[0]!;
		let distance = li.getElementsByClassName('distance')[0]!;
		let time = li.getElementsByClassName('time')[0]!;
		let online = li.getElementsByClassName('online')[0]!;

		let focus = li.getElementsByClassName('focus')[0]! as HTMLElement;
		let info = li.getElementsByClassName('info')[0]! as HTMLElement;

		setTextOrRemoveIfNull(meeting.name, name);
		setTextOrRemoveIfNull(meeting.country, country);
		setTextOrRemoveIfNull(meeting.region, region);
		setTextOrRemoveIfNull(meeting.distance ? meeting.distance + ' km' : null, distance);
		setTextOrRemoveIfNull('Every Sunday 19:00 - 20:00', time);
		setTextOrRemoveIfNull(meeting.online ? 'online' : null, online);

		focus.onclick = createHtmlCallback(meeting, () => onFocusClick);
		info.onclick = createHtmlCallback(meeting, () => onInfoClick);

		logo.src = getLogoImgUrlByOrg(meeting.org);

		results.appendChild(li);
	}
}

function setTextOrRemoveIfNull(value: any, element: Element) {
	if (value) {
		element.textContent = value;
	} else {
		element.remove();
	}
}

function setLoadingEnabled(enabled: boolean) {
	loading.hidden = !enabled;
}

export function showMeetingInfoPopup() {

}

export function initialize() {
	loadElements();
	loadMap();
}

function loadElements() {
	loading = document.getElementById('loading')!;
	results = document.getElementById('results')! as HTMLUListElement;
}

function loadMap() {
	const mapLayer = L.tileLayer('https://tile.openstreetmap.org/{z}/{x}/{y}.png', {
		maxZoom: 19,
		noWrap: true,
		attribution: '&copy; <a href="https://www.openstreetmap.org/copyright">OpenStreetMap</a> contributors'
	})

	map = L.map('map', {
		zoomControl: true,
		zoom: 2,
		minZoom: 2,
		center: { lng: 0, lat: 0 },
		layers: [mapLayer]
	});
}
