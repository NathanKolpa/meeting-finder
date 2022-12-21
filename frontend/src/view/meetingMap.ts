import {
    DivIcon,
    Icon,
    icon,
    LatLngExpression,
    Map,
    map,
    Marker,
    markerClusterGroup,
    MarkerClusterGroup,
    MarkerOptions,
    tileLayer
} from "leaflet";
import 'leaflet.markercluster';
import { Meeting, MeetingPosition, Organization } from "../models";
import { getLogoImgUrlByOrg } from "./logo";
import { MeetingCallback } from "./callback";

interface MapMeetingActions {
    remove: () => void,
    focus: () => void,
}

class MeetingMarker extends Marker {
    public get meeting(): Meeting {
        return this._meeting;
    }

    constructor(private _meeting: Meeting, latLng: LatLngExpression, opts?: MarkerOptions) {
        super(latLng, opts);
    }
}

const MAX_ZOOM = 16;

export class MeetingMap {
    private readonly map: Map;
    private readonly cluster: MarkerClusterGroup;

    private iconCache: { [key: string]: Icon } = {};
    private actions: { [id: number]: MapMeetingActions } = {};
    private clickCallback: MeetingCallback = null;


    public constructor(id: string) {
        const mapLayer = tileLayer('https://tile.openstreetmap.org/{z}/{x}/{y}.png', {
            maxZoom: MAX_ZOOM,
            noWrap: true,
            attribution: '&copy; <a href="https://www.openstreetmap.org/copyright">OpenStreetMap</a> contributors'
        });

        this.cluster = markerClusterGroup({
            animate: true,
            chunkedLoading: true,
            maxClusterRadius: z => 80 - ((z + 3) * 4),

            iconCreateFunction: (cluster) => {
                let markers = cluster.getAllChildMarkers() as MeetingMarker[];
                const sampleMarkers = this.sampleMarkers(markers, 5);

                let html = document.createElement('div');
                html.className = 'meeting-cluster';

                for (let marker of sampleMarkers) {
                    let container = document.createElement('div');
                    container.className = 'icon-container';

                    let icon = document.createElement('img') as HTMLImageElement;
                    icon.className = 'icon';
                    icon.src = getLogoImgUrlByOrg(marker.meeting.org);

                    container.appendChild(icon);
                    html.appendChild(container);
                }

                return new DivIcon({ html });
            }
        });

        this.map = map(id, {
            zoomControl: true,
            zoom: 2,
            minZoom: 2,
            center: { lng: 0, lat: 0 },
            layers: [mapLayer, this.cluster],
        });
    }

    private sampleMarkers(markers: MeetingMarker[], maxMarkers: number): MeetingMarker[] {
        const sampleMarkers = [];

        const usedOrgs = new Set<Organization>();
        const usedIndexes = new Set<number>();

        // select unique markers
        for (let [i, marker] of markers.entries()) {
            if (sampleMarkers.length >= maxMarkers) {
                break;
            }

            if (!usedOrgs.has(marker.meeting.org)) {
                sampleMarkers.push(marker);

                usedOrgs.add(marker.meeting.org);
                usedIndexes.add(i);
            }
        }

        // fill the rest up
        for (let [i, marker] of markers.entries()) {
            if (sampleMarkers.length >= maxMarkers) {
                break;
            }

            if (!usedIndexes.has(i)) {
                sampleMarkers.push(marker);
            }
        }

        // reverse so unique markers come on top
        sampleMarkers.reverse();

        return sampleMarkers;
    }

    public addMeetings(meetings: Meeting[]) {
        meetings = [...meetings];

        for (const meeting of meetings) {
            this.addMeeting(meeting);
        }
    }

    public addMeeting(meeting: Meeting) {
        if (!meeting.position) {
            return;
        }

        let pos: LatLngExpression = [meeting.position.latitude, meeting.position.longitude];

        let meetingMarker = new MeetingMarker(meeting, pos, {
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
                this.map.flyTo(pos, MAX_ZOOM, {
                    animate: true,
                });
            },
        };

        this.cluster.addLayer(meetingMarker);
    }

    public focus(meeting: Meeting) {
        document.scrollingElement?.scroll({ top: 0 });
        this.actions[meeting.id]?.focus();
    }

    public goToPosition(position: MeetingPosition) {
        this.map.flyTo([position.latitude, position.longitude], 10);
    }

    public clear() {
        this.cluster.clearLayers();
        this.actions = {};
    }

    private getMapIconByOrg(org: Organization): Icon {
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
