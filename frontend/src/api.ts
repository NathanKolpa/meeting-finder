import {Meeting} from "./models";

export async function fetchMeetings(): Promise<Meeting[]> {
    const request = await fetch(import.meta.env.VITE_API_URL);
    const response = await request.json() as ApiMeeting[];

    let id = 0;

    return response.map(apiMeeting => ({
        name: apiMeeting.name,
        id: ++id,

        notes: apiMeeting.notes,
        day: apiMeeting.time.recurring.day,
        country: apiMeeting.location.country,
        position: apiMeeting.location.position,
        online: apiMeeting.online_options.is_online,
        org: apiMeeting.org,
        address: apiMeeting.location.address,
        region: apiMeeting.location.address,
        onlineUrl: apiMeeting.online_options.online_url,
        distance: 0,
        source: apiMeeting.source,
        time: apiMeeting.time.recurring.time,
        durationInSecs: apiMeeting.duration.secs,
        locationName: apiMeeting.location.location_name,
        onlineNotes: apiMeeting.online_options.notes,
        locationNotes: apiMeeting.location.location_notes
    }))
}

interface ApiMeeting {
    name: string;
    org: ApiOrg;
    notes: null | string;
    source: string;
    contact: ApiContact;
    location: ApiLocation;
    online_options: ApiOnlineOptions;
    time: ApiTime;
    duration: ApiDuration;
}

interface ApiContact {
    email: null | string;
    phone: null | string;
}

interface ApiDuration {
    secs: number;
    nanos: number;
}

interface ApiLocation {
    position: ApiPosition | null;
    location_name: null | string;
    location_notes: null | string;
    country: null | string;
    region: null | string;
    address: string;
}

interface ApiPosition {
    latitude: number;
    longitude: number;
}

interface ApiOnlineOptions {
    online_url: null | string;
    notes: null | string;
    is_online: boolean;
}

enum ApiOrg {
    AnonymousAlcoholics = "AnonymousAlcoholics",
}

interface ApiTime {
    recurring: ApiRecurring;
}

interface ApiRecurring {
    day: ApiDay;
    time: string;
}

enum ApiDay {
    Friday = "Friday",
    Monday = "Monday",
    Saturday = "Saturday",
    Sunday = "Sunday",
    Thursday = "Thursday",
    Tuesday = "Tuesday",
    Wednesday = "Wednesday",
}