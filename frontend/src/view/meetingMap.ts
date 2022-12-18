import L, {circleMarker, Icon, icon, LatLngExpression, Map, map, tileLayer} from "leaflet";
import {Meeting, Organization} from "../models";
import {getLogoImgUrlByOrg} from "./logo";
import {MeetingCallback} from "./callback";

interface MapMeetingActions {
    remove: () => void,
    focus: () => void,
}

export class MeetingMap {
    private readonly map: Map;
    private iconCache: { [key: string]: Icon } = {};
    private actions: { [id: number]: MapMeetingActions } = {};
    private clickCallback: MeetingCallback = null;

    public constructor(id: string) {
        const mapLayer = tileLayer('https://tile.openstreetmap.org/{z}/{x}/{y}.png', {
            maxZoom: 19,
            noWrap: true,
            attribution: '&copy; <a href="https://www.openstreetmap.org/copyright">OpenStreetMap</a> contributors'
        });

        this.map = map(id, {
            zoomControl: true,
            zoom: 2,
            minZoom: 2,
            center: {lng: 0, lat: 0},
            layers: [mapLayer],
        });
    }

    public addMeetings(meetings: Meeting[]) {
        for (const meeting of meetings) {
            this.addMeeting(meeting);
        }
    }

    public addMeeting(meeting: Meeting) {
        if (!meeting.position) {
            return;
        }

        let pos: LatLngExpression = [meeting.position.latitude, meeting.position.longitude];

        let marker = L.marker(pos, {
            icon: this.getMapIconByOrg(meeting.org),
            title: meeting.name,
            riseOnHover: true,
        });

        marker.on('click', () => {
            if (this.clickCallback) {
                this.clickCallback(meeting);
            }
        })

        this.actions[meeting.id] = {
            remove: () => marker.remove(),
            focus: () => {
                this.map.flyTo(pos, 13, {
                    animate: false
                });

                marker.openPopup()
            },
        };

        marker.addTo(this.map);
    }

    public focus(meeting: Meeting) {
        this.actions[meeting.id]?.focus();
    }

    public clear() {
        for (const actions of Object.values(this.actions)) {
            actions.remove();
        }

        this.actions = {};
    }

    private getMapIconByOrg(org: Organization): L.Icon {
        const cached = this.iconCache[org];

        if (cached) {
            return cached;
        }

        const newIcon = icon({
            iconUrl: getLogoImgUrlByOrg(org),
            iconSize: [40, 40],
        });

        this.iconCache[org] = newIcon;

        return newIcon;
    }

    public setMeetingClickCallback(callback: MeetingCallback) {
        this.clickCallback = callback;
    }
}