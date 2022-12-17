import L from 'leaflet';
import {Meeting, Organization} from "./models";
import aaLogoUrl from './assets/logos/aa.png';

type MeetingCallback = ((meeting: Meeting) => void) | null;

let map: L.Map;
let loading: HTMLElement;
let results: HTMLUListElement;

let onFocusClick: MeetingCallback = null;
let onViewOnMapClick: MeetingCallback = null;

export function setOnViewOnMapClick(callback: MeetingCallback) {
    onViewOnMapClick = callback;
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

const iconCache: { [key: string]: L.Icon } = {};

function getMapIconByOrg(org: Organization): L.Icon {
    let cached = iconCache[org];

    if (cached) {
        return cached;
    }

    let icon = L.icon({
        iconUrl: getLogoImgUrlByOrg(org),
        iconSize: [30, 30],
    });

    iconCache[org] = icon;

    return icon;
}

function createHtmlCallback(meeting: Meeting, getCallback: () => MeetingCallback): () => void {
    return () => {
        let callback = getCallback();

        if (callback) {
            callback(meeting);
        }
    }
}

interface ResultListItemActions {
    setFocus: () => void,
    clearFocus: () => void
}

let resultListActionMap: { [id: number]: ResultListItemActions } = {};

function addMeetingToResultList(meeting: Meeting) {
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
                <li class="seperator">|</li>
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

    setTextOrRemoveIfNull(meeting.name, name);
    setTextOrRemoveIfNull(meeting.country, country);
    setTextOrRemoveIfNull(meeting.region, region);
    setTextOrRemoveIfNull(meeting.distance ? meeting.distance + ' km' : null, distance);
    setTextOrRemoveIfNull('Every Sunday 19:00 - 20:00', time);
    setTextOrRemoveIfNull(meeting.online ? 'online' : null, online);

    if (!meeting.position) {
        focus.classList.add('disabled');
        focus.title = 'No location available'
    } else {
        focus.onclick = createHtmlCallback(meeting, () => onViewOnMapClick);
    }


    logo.src = getLogoImgUrlByOrg(meeting.org);

    resultListActionMap[meeting.id] = {
        clearFocus: () => li.classList.remove('active'),
        setFocus: () => li.classList.add('active')
    };

    results.appendChild(li);
}

function clearResultList() {
    resultListActionMap = {};
    results.replaceChildren();
}

function clearMapMeetings() {
    for (const actions of Object.values(mapMeetingActionMap)) {
        actions.remove();
    }

    mapMeetingActionMap = {};
}

export function clearMeetings() {
    currentFocus = null;

    clearResultList();
    clearMapMeetings();
}

interface MapMeetingActions {
    remove: () => void,
    focus: () => void,
}

let mapMeetingActionMap: { [id: number]: MapMeetingActions } = {};

function addMeetingToMap(meeting: Meeting) {
    if (!meeting.position) {
        return;
    }

    let pos: L.LatLngExpression = [meeting.position.latitude, meeting.position.longitude];

    let icon = getMapIconByOrg(meeting.org);

    let marker = L.marker(pos, {
        icon,
        title: meeting.name,
        riseOnHover: true,
    });

    marker.addEventListener("click", createHtmlCallback(meeting, () => onFocusClick));

    mapMeetingActionMap[meeting.id] = {
        remove: () => marker.remove(),
        focus: () => {
            map.flyTo(pos, 8, {
                duration: 1,
                easeLinearity: 1,
            });

            marker.openPopup()
        },
    };

    marker.addTo(map);
}

export function setMapFocus(meeting: Meeting) {
    let actions = mapMeetingActionMap[meeting.id];

    if (actions) {
        actions.focus();
    }
}

function setResultListFocus(meeting: Meeting) {
    let action = resultListActionMap[meeting.id];

    if (action) {
        action.setFocus();
    }

    if (currentFocus) {
        let currentFocusAction = resultListActionMap[currentFocus.id];

        if (currentFocusAction) {
            currentFocusAction.clearFocus();
        }
    }
}


let currentFocus: Meeting | null = null;

export function setFocusTo(meeting: Meeting) {
    if (currentFocus == meeting) {
        return;
    }

    setResultListFocus(meeting);

    currentFocus = meeting;
}

export function setMeetings(meetings: Meeting[]) {
    setLoadingEnabled(false);

    for (const meeting of meetings) {
        addMeetingToResultList(meeting);
        addMeetingToMap(meeting);
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
        center: {lng: 0, lat: 0},
        layers: [mapLayer]
    });
}
