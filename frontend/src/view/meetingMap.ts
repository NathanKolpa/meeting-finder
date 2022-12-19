import {
    circleMarker,
    Icon,
    icon,
    LatLngExpression,
    Map,
    map,
    tileLayer,
    markerClusterGroup,
    MarkerClusterGroup,
    marker,
    Point, DivIcon
} from "leaflet";
import 'leaflet.markercluster';
import {Meeting, Organization} from "../models";
import {getLogoImgUrlByOrg} from "./logo";
import {MeetingCallback} from "./callback";

interface MapMeetingActions {
    remove: () => void,
    focus: () => void,
}

export class MeetingMap {
    private readonly map: Map;
    private readonly cluster: MarkerClusterGroup;

    private iconCache: { [key: string]: Icon } = {};
    private actions: { [id: number]: MapMeetingActions } = {};
    private clickCallback: MeetingCallback = null;


    public constructor(id: string) {
        const mapLayer = tileLayer('https://tile.openstreetmap.org/{z}/{x}/{y}.png', {
            maxZoom: 19,
            noWrap: true,
            attribution: '&copy; <a href="https://www.openstreetmap.org/copyright">OpenStreetMap</a> contributors'
        });

        this.cluster = markerClusterGroup({
            animate: true,
            chunkedLoading: true,


            iconCreateFunction: (cluster) => {
                let childCount = cluster.getChildCount();

                let classes = ' marker-cluster-';

                if (childCount < 10) {
                    classes += 'small';
                }
                else if (childCount < 100) {
                    classes += 'medium';
                }
                else {
                    classes += 'large';
                }

                return new DivIcon({ html: '<div><span>' + childCount + '</span></div>',
                    className: 'marker-cluster' + classes, iconSize: new Point(40, 40) });
            }
        });

        this.map = map(id, {
            zoomControl: true,
            zoom: 2,
            minZoom: 2,
            center: {lng: 0, lat: 0},
            layers: [mapLayer, this.cluster],
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

        let meetingMarker = marker(pos, {
            icon: this.getMapIconByOrg(meeting.org),
            title: meeting.name,
            riseOnHover: true,
        });

        meetingMarker.on('click', () => {
            if (this.clickCallback) {
                this.clickCallback(meeting);
            }
        })

        this.actions[meeting.id] = {
            remove: () => meetingMarker.remove(),
            focus: () => {
                this.map.flyTo(pos, 13, {
                    animate: true,
                    duration: 1
                });
            },
        };

        this.cluster.addLayer(meetingMarker);
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